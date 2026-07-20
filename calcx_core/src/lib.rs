mod tokens;
pub mod num;
pub mod units;
pub mod expr;
pub mod conversion;
pub mod variables;
pub mod keys_func;
pub mod utils;

pub use num::Num;
pub use expr::Expr;

use tokens::Token;

const PRECISION: u32 = 4096;

// Testing of Calcx-Core:
#[cfg(test)]
mod tests;

pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
    variables: variables::VariableStorage,
    precision: usize,
    history: Vec<Expr>,
}

mod parse;
mod eval;
mod funcs;

impl Calc {
    pub fn new(precision: usize) -> Calc {
        Calc { tokens: vec![], current: 0, variables: variables::VariableStorage::new(), precision: precision, history: vec![] }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    pub fn get_ans(&self) -> Result<Expr, String> {
        match self.history.last() {
            Some(Expr::Number(num)) => Ok(Expr::Number(num.clone())),
            _ => Err(String::from("There is no last answer")),
        }
    }

    pub fn change_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    // General Use Functions
    fn expect(&mut self, token: Token) -> Result<(), String> {
        match std::mem::discriminant(&self.advance()?) == std::mem::discriminant(&token) {
            true => Ok(()),
            false => Err(String::from("Expected a Token which was not next."))
        }
    }

    // Get current token
    fn peek(&self) -> Result<Token, String> {
        return match self.tokens.get(self.current).cloned() {
            Some(v) => Ok(v),
            _ => Err(String::from("self.peek failed"))
        };
    }

    // Get current token, then advance pointer
    fn advance(&mut self) -> Result<Token, String> {
        self.current += 1;
        match self.tokens.get(self.current-1).cloned() {
            Some(v) => Ok(v),
            _ => Err(String::from("self.advance failed")),
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
    pub fn run_ouput(&mut self, query: &str) -> String {
        match self.run(query) {
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

    pub fn run(&mut self, query: &str) -> Result<Expr, String> {
        self.current = 0;

        // This function is supposed to tokenize the given query
        self.tokens = tokens::tokenize(query);
        // println!{"{:?}", self.tokens}

        // println!{"Parsing Done"};

        let tree = self.build_tree();
        // println!{"{tree:?}"};
        let output = self.eval(tree);

        match &output {
            Ok(Expr::Number(num)) => {self.history.push(Expr::Number(num.clone()))}
            _ => {},
        }

        output
    }
}
