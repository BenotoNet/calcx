use super::Calc;
use super::expr::Expr;
use super::num::Num;
use super::token::Token;


impl Calc {
    pub fn build_tree(&mut self) -> Expr {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_keywords();

        while
            match self.peek() {
            Some(Token::Add)|Some(Token::Sub) => true,
            _ => false,
        } {
            let operation = self.advance().unwrap();
            let right = self.parse_keywords();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
        }
        return left;
    }

    fn parse_keywords(&mut self) -> Expr {
        let mut left = self.parse_term();

        while 
            match self.peek() {
                Some(Token::Keyword(_)) => true,
                _ => false,
            }
        {
            // We have found a Keyword
            let keyword = self.advance().unwrap();
            let right = self.parse_term();

            left = Expr::Binary { left: Box::new(left), op: keyword, right: Box::new(right) };
        }
        return left;
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_exponents();

        while
            match self.peek() {
                Some(Token::Mul)|Some(Token::Div) => true,
                _ => false,
        } {
            let operation = match self.advance() {
                Some(v) => v,
                _ => return left,
            };
            let right = self.parse_exponents();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
        }
        return left;
    }

    fn parse_exponents(&mut self) -> Expr {
        let mut left = self.parse_bracket();

        while
            match self.peek() {
                Some(Token::Mod)|Some(Token::Pow) => true,
                _ => false,
            } {
                let operation = self.advance().unwrap();
                let right = self.parse_bracket();
                left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) };
            }
        return left;
    }

    fn parse_bracket(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while match self.peek() {
            Some(Token::LBrac) => true,
            _ => {false},
        } {
            let operation = Token::Mul;
            let right = self.parse_factor();
            left = Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) }
        }
        return left;
    }

    fn parse_factor(&mut self) -> Expr {
        match self.peek() {
            Some(Token::Number(num)) => {self.advance(); Expr::Number(num)},
            // Bracket Logic => Starting a new branch
            Some(Token::LBrac) => {
                self.advance();
                let expr = self.parse_expression();
                // EXPECT RBrac
                self.expect(Token::RBrac);
                return expr;
            }
            // make - 2 into the number "-2"
            Some(Token::Sub) => {
                self.advance();
                return Expr::Number(super::num::Num::new(-1.0, vec![]));
            }
            // We found a variable!
            Some(Token::Var(var)) => {
                self.advance();
                match self.peek() {
                    Some(Token::Assign) => {
                            self.advance();
                            let right = self.parse_expression();
                            match self.eval(right).unwrap() {
                                Expr::Number(num) => {
                                    self.variables.set_var(var, num.clone());
                                    Expr::Number(num)
                                },
                                _ => {panic!{"Could not evaluate right side of assignment!"};}
                            }
                    }
                    _ => {
                        return match self.variables.get_var(var) {
                            Some(v) => v,
                            _ => panic!{"Undefined Variable"}
                        };
                    }
                }
            }
            Some(Token::Keyword(_)) => {
                return Expr::Number(Num::unitless(1.0));
            }
            _ => {
                // an unknown Token!
                panic!{"Unknown Token!"}
            },
        }
    }
}
