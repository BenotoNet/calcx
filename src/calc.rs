pub struct Calc {
    query: String,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { query: String::new() }
    }

    // Function to parse given query (e.g. 5+(9*2) and give a result, needs to be written very
    // generally => later 'to keyword' needs to work as well)

    pub fn run(&mut self, query: &str) -> String {
        // This function is supposed to parse the given query
        self.query = String::from(query);
        // TODO
        // println!{"{}", self.internal_parse(&self.query)};
        String::new()
    }
}
