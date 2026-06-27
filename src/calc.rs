mod tokenize;
pub mod num;
pub mod units;
pub mod token;
pub mod expr;
pub mod conversion;

use token::Token;
use expr::Expr;


pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
}

mod parse;
mod eval;

impl Calc {
    pub fn new() -> Calc {
        Calc { tokens: vec![], current: 0 }
    }

    fn expect(&mut self, token: Token) {
        assert!{std::mem::discriminant(&self.advance().unwrap()) == std::mem::discriminant(&token)};
    }

    fn peek(&self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    fn advance(&mut self) -> Option<Token> {
        let current_token = self.tokens.get(self.current).cloned();
        self.current += 1;
        return current_token;
    }

    // API to run a specific command and capture its output
    pub fn run_ouput(&mut self, query: &str) -> String {
        self.run(query).display()
    }

    pub fn run(&mut self, query: &str) -> Expr {
        self.current = 0;

        // This function is supposed to tokenize the given query
        self.tokens = tokenize::tokenize(query);

        let tree = self.build_tree();
        // println!{"{tree:?}"};
        match self.eval(tree) {
            Ok(expr) => expr,
            Err(_) => {panic!{"Deal with Errors!"}},
        }
    }
}
