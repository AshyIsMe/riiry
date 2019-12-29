use log::debug;
use rayon::prelude::*;
use sublime_fuzzy::best_match;

use rff;

use super::worker::Cancel;

pub fn filter_lines(query: &str, strlines: Vec<String>) -> Vec<String> {
    if query.is_empty() {
        return strlines;
    }

    debug!(
        "filter_lines() query: {}, strlines.len(): {}",
        query,
        strlines.len()
    );

    let mut matches: Vec<(isize, String)> = strlines
        .into_iter()
        .map(|s| match best_match(query, &s) {
            Some(m) => (m.score(), s),
            None => (0, s),
        })
        .filter(|t| t.0 > 0)
        .collect();
    matches.sort_by(|a, b| a.0.cmp(&b.0).reverse());

    let results: Vec<String> = matches.into_iter().map(|t| t.1).collect();

    debug!("filter_lines() FINISHED query: {}", query);

    results
}

/// Returns `None` if aborted.
pub fn filter_lines_rff(
    query: &str,
    strlines: &Vec<String>,
    cancel: &Cancel,
) -> Option<Vec<String>> {
    if query.is_empty() {
        return Some(strlines.clone());
    }

    debug!(
        "filter_lines_rff() query: {}, strlines.len(): {}",
        query,
        strlines.len()
    );

    let mut matches: Vec<_> = strlines
        .par_iter()
        .filter_map(|line| {
            if cancel.check_done() {
                return None;
            }
            rff::match_and_score(query, &line)
        })
        .collect();

    if cancel.check_done() {
        return None;
    }

    matches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

    let results: Vec<String> = matches.into_iter().map(|t| String::from(t.0)).collect();

    debug!("filter_lines_rff() FINISHED query: {}", query);

    Some(results)
}
