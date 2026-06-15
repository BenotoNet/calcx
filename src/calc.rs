mod tokenize;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    
    LBrac,
    RBrac,

    Assign,

    Var(String),

    Keyword(String),
    Unknown(String),
}

#[derive(Debug)]
enum Expr {
    Number(f64),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    }
}

pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { tokens: vec![], current: 0 }
    }

    fn peek(&self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    fn advance(&mut self) -> Option<Token> {
        self.current += 1;
        return self.tokens.get(self.current).cloned();
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();

        while
            match self.peek() {
            Some(Token::Add)|Some(Token::Sub) => true,
            _ => false,
        } {
            let operation = self.peek().unwrap(); self.advance();
            let right = self.parse_term();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
        }
        return left;
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while
            match self.peek() {
            Some(Token::Mul)|Some(Token::Div) => true,
            _ => false,
        } {
            let operation = self.peek().unwrap(); self.advance();
            let right = self.parse_factor();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
        }
        return left;
    }

    fn parse_factor(&mut self) -> Expr {
        println!{"getting factor"}
        match self.peek() {
            Some(Token::Number(num)) => {self.advance(); Expr::Number(num)},
            Some(Token::LBrac) => {
                self.advance();
                let expr = self.parse_expression();
                // EXPECT RBrac
                self.advance();
                return expr;
            }
            _ => {panic!{}},
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
                            Token::Add => {Expr::Number(num1 + num2)},
                            Token::Sub => {Expr::Number(num1 - num2)},
                            Token::Mul => {Expr::Number(num1 * num2)},
                            Token::Div => {Expr::Number(num1 / num2)},
                            _ => {panic!{}}
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
        // This function is supposed to tokenize the given query
        self.tokens = tokenize::tokenize(query);

        // FIX: Debug
        let tree = self.build_tree();
        println!("{:?}", self.eval(tree));
        // format!{"{:?}", self.build_tree()}
        String::new()
    }
}
