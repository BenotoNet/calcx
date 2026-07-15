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

    pub fn get_ans(&self) -> Option<Expr> {
        match self.history.last() {
            Some(Expr::Number(num)) => Some(Expr::Number(num.clone())),
            _ => None,
        }
    }

    pub fn change_precision(&mut self, precision: usize) {
        self.precision = precision;
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

    pub fn run(&mut self, query: &str) -> Result<Expr, String> {
        self.current = 0;

        // This function is supposed to tokenize the given query
        self.tokens = tokenize::tokenize(query);
        // println!{"{:?}", self.tokens}

        // println!{"Parsing Done"};

        let tree = self.build_tree();
        println!{"{tree:?}"};
        let output = self.eval(tree);

        match &output {
            Ok(Expr::Number(num)) => {self.history.push(Expr::Number(num.clone()))}
            _ => {},
        }

        output
    }
}
