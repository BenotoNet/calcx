mod tokens;
pub mod num;
pub mod units;
pub mod expr;
pub mod variables;
pub mod keys_func;
pub mod utils;
pub mod persistence;

pub use num::Num;
pub use expr::Expr;

use tokens::Token;

use serde::{Serialize, Deserialize};

const PRECISION: u32 = 4096;

// Testing of Calcx-Core:
#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Debug)]
pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
    variables: variables::VariableStorage,
    userfunctions: funcs::custom::FunctionStorage,
    precision: usize,
    history: Vec<Expr>,
}

mod parse;
mod eval;
mod funcs;

impl Calc {
    pub fn new(precision: usize) -> Calc {
        Calc { tokens: vec![], current: 0, variables: variables::VariableStorage::new(), userfunctions: funcs::custom::FunctionStorage::new(), precision: precision, history: vec![] }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    pub fn get_ans(&self) -> Result<Expr, String> {
        match self.history.last() {
            Some(Expr::Number(num)) => Ok(Expr::Number(num.clone())),
            _ => Err(String::from("There is no last answer yet or it is not accessible")),
        }
    }

    pub fn get_last_nth_ans(&self, nth: usize) -> Result<Expr, String> {
        if nth >= self.history.len() {return Err(String::from("That answer does not exist"))}
        match self.history.iter().nth(self.history.len()-nth) {
            Some(Expr::Number(num)) => Ok(Expr::Number(num.clone())),
            _ => Err(String::from("That answer is not accessible")),
        }
    }

    pub fn change_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    // General Use Functions
    fn expect(&mut self, token: Token) -> Result<(), String> {
        match std::mem::discriminant(&self.advance()?) == std::mem::discriminant(&token) {
            true => Ok(()),
            false => Err(String::from("Unexpected token (different one was expected)"))
        }
    }

    // Get current token
    fn peek(&self) -> Result<Token, String> {
        return match self.tokens.get(self.current).cloned() {
            Some(v) => Ok(v),
            _ => Err(String::from("There is a character missing"))
        };
    }

    // Check if there is a token anywhere in the tokens
    #[allow(unused)]
    fn contains(&self, tokentype: Token) -> bool {
        return self.tokens.contains(&tokentype)
    }

    // Check if there is a specific token later in the query
    fn contains_later(&self, tokentype: Token) -> bool {
        self.tokens[self.current..self.tokens.len()].contains(&tokentype)
    }

    // Get current token, then advance pointer
    fn advance(&mut self) -> Result<Token, String> {
        self.current += 1;
        match self.tokens.get(self.current-1).cloned() {
            Some(v) => Ok(v),
            _ => Err(String::from("Another character was expected but did not exist")),
        }
    }

    fn last_token(&mut self) -> Result<Token, String> {
        self.rewind();
        self.advance()
    }

    fn rewind(&mut self) {
        self.current -= 1;
    }

    // API to run a specific command and capture its output
    pub fn run_output(&mut self, query: &str, save_history: bool) -> String {
        match self.run(query, save_history) {
            Ok(output) => output.display(self.precision),
            Err(error) => error
        }
    }

    pub fn build_tree_from(&mut self, tokens: Vec<Token>) -> Result<Expr, String> {
        let old_tokens = self.tokens.clone();
        let old_current = self.current;
        self.set_tokens(tokens);
        self.current = 0;
        let tree = self.build_tree();
        self.set_tokens(old_tokens);
        self.current = old_current;
        tree
    }

    pub fn get_customs(&self) -> Vec<String> {
        let mut output = vec![];
        let variables: Vec<String> = self.variables.get_keys();
        let functions: Vec<String> = self.userfunctions.get_keys();
        output.extend(variables);
        output.extend(functions);
        output
    }

    pub fn run(&mut self, query: &str, save_history: bool) -> Result<Expr, String> {
        self.current = 0;

        // This function is supposed to tokenize the given query
        self.tokens = tokens::tokenize(self, query);
        // println!{"{:?}", self.tokens}

        // println!{"Parsing Done"};

        let tree = self.build_tree();
        // println!{"{tree:?}"};
        let output = self.eval(tree);

        match (&output, save_history) {
            (Ok(Expr::Number(num)), true) => {self.history.push(Expr::Number(num.clone()))}
            _ => {},
        }

        output
    }
}
