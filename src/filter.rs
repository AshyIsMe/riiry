use sublime_fuzzy::best_match;

pub fn filter_lines<'a>(query: &str, strlines: Vec<&'a str>) -> Vec<&'a str> {
    if query.is_empty() {
        return strlines;
    }

    //let mut results: Vec<(isize, &str)> = strlines
    let mut results: Vec<&str> = strlines
        .into_iter()
        .map(|s| match best_match(query, s) {
            Some(m) => (m.score(), s),
            None => (0, s),
        })
        .filter(|t| t.0 > 0)
        .map(|t| t.1)
        .collect();
    results.sort();
    results.reverse();

    //let sorted_matches: Vec<&str> = results
        //.into_iter()
        //.filter(|t| t.0 > 0)
        //.map(|t| t.1)
        //.collect();

    //sorted_matches
    results
}
