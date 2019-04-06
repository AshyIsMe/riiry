
use sublime_fuzzy::best_match;


// TODO: Hilariously slow when there is a large haystack.
// get_home_files() can't be used until filter_lines() is faster...
pub fn filter_lines(query: &str, strlines: &str) -> String {
    if query.is_empty() {
        return String::from(strlines);
    }

    let v: Vec<&str> = strlines.split('\n').collect();
    let mut results: Vec<(isize, &str)> = v
        .into_iter()
        .map(|s| match best_match(query, s) {
            Some(m) => (m.score(), s),
            None => (0, s),
        })
        .collect();
    results.sort();
    results.reverse();

    let sorted_matches: Vec<&str> = results
        .into_iter()
        .filter(|t| t.0 > 0)
        .map(|t| t.1)
        .collect();

    sorted_matches.join("\n")
}
