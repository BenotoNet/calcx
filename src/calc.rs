use crate::utils;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Expression(String),
}

pub struct Calc {
    query: Vec<Token>,
}

const OPERATORS: [char; 7] = ['*', '^', '+', '-', '(', ')', '/'];

impl Calc {
    pub fn new() -> Calc {
        Calc { query: vec![] }
    }

    // API to run a specific command
    pub fn run(&mut self, query: &str) -> String {
        // This function is supposed to tokenize the given query
        self.query = Calc::tokenize(query);

        format!{"{:?}", self.query}
        // String::new()
    }


    // Function to put string query into a more readable format for the computer (Seperate each
    // token)
    fn tokenize(query: &str) -> Vec<Token> {
        let mut tokens = vec![];

        let rough_tokens = query.split(" ").collect::<Vec<&str>>();

        for rough_token in rough_tokens {
            let mut part_token = String::new();
            for c in rough_token.chars() {
                if OPERATORS.contains(&c) {
                    // TODO: Remove unwraps
                    tokens.push(Token::Number(part_token.clone().parse().unwrap()));
                    tokens.push(Token::Expression(String::from(c)));
                    part_token = String::new();
                } else {
                    part_token += &c.to_string();
                }
            }
            tokens.push(Token::Number(part_token.clone().parse().unwrap()));
        }

        // TODO: Clean Up Tokens (remove spaces, etc)

        // Seperate Blocks with arithmetic operators:

        return tokens;
    }
}
