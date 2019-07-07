
pub struct RiiryState {
    haystack: Vec<String>,
    needle: String
}

impl RiiryState {
    pub fn new() -> RiiryState {
        RiiryState {
            haystack: Vec::new(),
            needle: String::new() }
    }

    pub fn get_haystack<'a>(&'a self) -> &'a Vec<String> { &self.haystack }
    pub fn extend_haystack(&mut self, data: Vec<String>) {
        self.haystack.extend(data);
    }

    pub fn get_needle<'a>(&'a self) -> &'a String { &self.needle }
    pub fn set_needle(&mut self, data: String) { self.needle = data; }
}
