pub enum Token {
    Number(f64),
    Arithmetic(String),
    Operation(String) // TODO: -> Add Special Operations enum (e.g. Conversion, Time, etc)
}

pub struct Calc {
    query: Vec<Token>,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { query: vec![] }
    }

    // Function to parse / Tokenize given query (e.g. 5+(9*2) and give a result, needs to be written very
    // generally => later 'to keyword' needs to work as well)

    pub fn run(&mut self, query: &str) -> String {
        // This function is supposed to tokenize the given query
        self.query = Calc::tokenize(query);

        // Calculate the answer using the parsed tokens

        String::new()
    }

    // TODO: 
    fn is_number() {}

    // TODO: 

    // Function to put string query into a more readable format for the computer
    pub fn tokenize(query: &str) -> Vec<Token> {
        for a in query.chars() {
            // Check if Number
        }
        return vec![];
    }
}
