use super::Calc;
use super::expr::Expr;
use super::token::Token;


impl Calc {
    pub fn build_tree(&mut self) -> Option<Expr> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        let mut left = self.parse_keywords();

        while
            match self.peek() {
            Some(Token::Add)|Some(Token::Sub) => true,
            _ => false,
        } {
            let operation = self.advance().unwrap();
            let right = self.parse_keywords();
            left = Some(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) });
        }
        return left;
    }

    fn parse_keywords(&mut self) -> Option<Expr> {
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

            left = Some(Expr::Binary { left: Box::new(left), op: keyword, right: Box::new(right) });
        }
        return left;
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut left = self.parse_bracket();

        while
            match self.peek() {
                Some(Token::Mul)|Some(Token::Div) => true,
                _ => false,
        } {
            let operation = match self.advance() {
                Some(v) => v,
                _ => return left,
            };
            let right = self.parse_bracket();
            left = Some(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) });
        }
        return left;
    }

    fn parse_bracket(&mut self) -> Option<Expr> {
        let mut left = self.parse_exponents();

        while match self.peek() {
            Some(Token::LBrac) => true,
            _ => {false},
        } {
            let operation = Token::Mul;
            let right = self.parse_exponents();
            left = Some(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) })
        }
        return left;
    }

    fn parse_exponents(&mut self) -> Option<Expr> {
        let mut left = self.parse_factor();

        while
            match self.peek() {
                Some(Token::Mod)|Some(Token::Pow) => true,
                _ => false,
            } {
                let operation = self.advance().unwrap();
                let right = self.parse_factor();
                left = Some(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) });
            }
        return left;
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        match self.peek() {
            Some(Token::Number(num)) => {self.advance(); Some(Expr::Number(num))},
            // Bracket Logic => Starting a new branch
            Some(Token::LBrac) => {
                self.advance();
                let expr = self.parse_expression()?;

                // EXPECT RBrac
                self.expect(Token::RBrac);

                return Some(expr);
            }
            // make - 2 into the number "-2" by parsing into (-1)2
            Some(Token::Sub) => {
                self.advance();
                return Some(Expr::Number(super::num::Num::new(-1.0, vec![])));
            }
            // We found a variable!
            Some(Token::Var(var)) => {
                self.advance();
                match self.peek() {
                    Some(Token::Assign) => {
                            self.advance();
                            let right = self.parse_expression();
                            match self.eval(right) {
                                Ok(Expr::Number(num)) => {
                                    self.variables.set_var(var, num.clone());
                                    Some(Expr::Number(num))
                                },
                                _ => {panic!{"Could not evaluate right side of assignment!"};}
                            }
                    }
                    _ => {
                        return match self.variables.get_var(var) {
                            Some(v) => Some(v),
                            _ => panic!{"Undefined Variable"}
                        };
                    }
                }
            }
            _ => {
                // an unknown Token!
                return None;
            },
        }
    }
}
