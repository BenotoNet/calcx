mod tokenize;
pub mod num;
pub mod units;
pub mod token;
pub mod expr;
pub mod conversion;
pub mod variables;
pub mod keys_func;

use token::Token;
use expr::Expr;


pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
    variables: variables::VariableStorage,
}

mod parse;
mod eval;

impl Calc {
    pub fn new() -> Calc {
        Calc { tokens: vec![], current: 0, variables: variables::VariableStorage::new() }
    }

    // General Use Functions
    fn expect(&mut self, token: Token) {
        assert!{std::mem::discriminant(&self.advance().unwrap()) == std::mem::discriminant(&token)};
    }

    // Get current token
    fn peek(&self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    // Get current token, then advance pointer
    fn advance(&mut self) -> Option<Token> {
        let current_token = self.tokens.get(self.current).cloned();
        self.current += 1;
        return current_token;
    }

    // API to run a specific command and capture its output
    pub fn run_ouput(&mut self, query: &str) -> String {
        match self.run(query) {
            Ok(output) => output.display(),
            Err(error) => error
        }
    }

    pub fn run(&mut self, query: &str) -> Result<Expr, String> {
        self.current = 0;

        // This function is supposed to tokenize the given query
        self.tokens = tokenize::tokenize(query);
        // println!{"{:?}", self.tokens}

        let tree = self.build_tree();
        // println!{"{tree:?}"};
        self.eval(tree)
    }
}
