mod tokenize;
pub mod num;
pub mod units;
pub mod token;
pub mod expr;

use token::Token;
use expr::Expr;


pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { tokens: vec![], current: 0 }
    }

    fn expect(&mut self, token: Token) {
        // TODO: Does not quite work yet
        match self.advance().unwrap() {
            token => {},
            _ => {panic!{"This is not the expected Token!"}}
        }
    }

    fn peek(&self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    fn advance(&mut self) -> Option<Token> {
        let current_token = self.tokens.get(self.current).cloned();
        self.current += 1;
        return current_token;
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();

        while
            match self.peek() {
            Some(Token::Add)|Some(Token::Sub) => true,
            _ => false,
        } {
            let operation = self.advance().unwrap();
            let right = self.parse_term();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
        }
        return left;
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_exponents();

        while
            match self.peek() {
            Some(Token::Mul)|Some(Token::Div) => true,
            Some(Token::LBrac) => {
                // Leaving out the mul sign for brackets
                let right = self.parse_expression();
                return self.eval(Expr::Binary { left: Box::new(left), op: Token::Mul, right: Box::new(right) })
            }
            _ => false,
        } {
            let operation = self.advance().unwrap();
            let right = self.parse_exponents();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
        }
        return left;
    }

    fn parse_exponents(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while
            match self.peek() {
                Some(Token::Mod)|Some(Token::Pow) => true,
                _ => false,
            } {
                let operation = self.advance().unwrap();
                let right = self.parse_factor();
                left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
            }
        return left;
    }

    fn parse_factor(&mut self) -> Expr {
        match self.peek() {
            Some(Token::Number(num)) => {self.advance(); Expr::Number(num)},
            Some(Token::LBrac) => {
                self.advance();
                let expr = self.parse_expression();
                // EXPECT RBrac
                self.expect(Token::RBrac);
                return expr;
            }
            Some(Token::Sub) => {
                self.advance();
                return Expr::Number(num::Num::new(-1.0, vec![]));
            }
            _ => {panic!{"{:?}", self.peek()}},
        }
    }

    fn build_tree(&mut self) -> Expr {
        self.parse_expression()
    }

    fn eval(&mut self, tree: Expr) -> Expr {
        match tree {
            Expr::Binary { left, op, right } => {
                match (&*left, &*right) {
                    (Expr::Number(num1), Expr::Number(num2)) => {
                        // Be have an atomic Expression (only numbers)
                        match op {
                            Token::Add => {Expr::Number(num1.add(num2).unwrap())},
                            Token::Sub => {Expr::Number(num1.sub(num2).unwrap())},
                            Token::Mul => {Expr::Number(num1.mul(num2).unwrap())},
                            Token::Div => {Expr::Number(num1.div(num2).unwrap())},
                            Token::Mod => {Expr::Number(num1.modf(num2).unwrap())},
                            Token::Pow => {Expr::Number(num1.powf(num2).unwrap())},
                            _ => {panic!{"This is not an Operator!"}},
                        }
                    },
                    _ => {
                        // Not atomic yet, so evaluate: 
                        // return self.eval(self.eval(*left))
                        let new_op = Expr::Binary { left: Box::new(self.eval(*left)), op, right: Box::new(self.eval(*right))};
                        return self.eval(new_op);
                    }
                }
            },
            _ => {
                return tree;
            }
        }
    }

    // API to run a specific command and capture its output
    pub fn run(&mut self, query: &str) -> String {
        self.current = 0;
        // This function is supposed to tokenize the given query
        self.tokens = tokenize::tokenize(query);

        let tree = self.build_tree();
        self.eval(tree).display()
    }
}
