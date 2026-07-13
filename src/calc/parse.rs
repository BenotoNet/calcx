use super::Calc;
use super::expr::Expr;
use super::token::Token;


impl Calc {
    pub fn build_tree(&mut self) -> Option<Expr> {
        self.parse_expression()
    }

    pub fn parse_function_arguments(&mut self) -> Option<Expr> {
        let mut args = vec![];

        // Splitting Arguments with ','
        let mut temp_arg: Vec<Token> = vec![];
        let mut brackets = 0;

        // First opening Bracket -> Opening Bracket should not be added to the calculation, but is optional nonetheless
        match self.peek() {
            Some(Token::LBrac) => {
                brackets += 1;
                self.advance();
            }
            _ => {}
        }

        while match self.advance() {
            Some(Token::Func(func)) => {
                if func.as_str() == "," {
                    // one argument finished, appending to args, then continue
                    args.push(temp_arg.clone());
                    temp_arg = vec![];
                    true
                }
                else {
                    // We have a function as an argument for a function...? I guess that's okay...
                    temp_arg.push(Token::Func(func));
                    true
                }
            }
            // Increase Bracket count by one, so that when we have the matching RBrac, we don't stop
            // parsing args, e.g. func((5+2), 2)
            // Also add the brackets to the argument => ((5+2) / 2) otherwise becomes 5 + 2 / 2 
            //    -> Different results
            Some(Token::LBrac) => {temp_arg.push(Token::LBrac); brackets += 1; true},
            // Check if we are at the last closing bracket, then append final arg and stop
            Some(Token::RBrac) => {brackets -= 1; 
                if brackets <= 0 {
                    args.push(temp_arg.clone());
                    false
                } 
                else {
                   temp_arg.push(Token::RBrac);
                   true
                }
            },

            // We are at the end, because self.advance does not give anything anymore, therefore
            // stop
            // But first, append what the temp is right now
            None => {
                args.push(temp_arg.clone());
                false
            }

            // we got anything other than specified above, then append if not none and continue
            var => {
                // If we do not have brackets (-> originally brackets var is 0) we should stop right
                // here...
                match (brackets<=0, var) {
                    (true, Some(v)) => {
                        temp_arg.push(v);
                        args.push(temp_arg.clone());
                        false
                    },
                    (false, Some(v)) => {
                        temp_arg.push(v);
                        true
                    },
                    _ => true,
                }
            }
        } {};

        let mut out = None;
        for arg in args {
            let mut temp_calc = Calc::new(self.precision);
            temp_calc.set_tokens(arg.clone());
            out = Some(Expr::Arg {arg: Box::new(temp_calc.build_tree()), right: Box::new(out) });
        }

        out
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
                Some(Token::Func(_)) => {
                    let func_token = self.advance().unwrap();
                    let args = self.parse_function_arguments();
                    left = Some(Expr::Binary { left: Box::new(left), op: func_token, right: Box::new(args) });
                    true
                },
                _ => false,
            }
        {
            // We have found a Keyword
            let keyword = match self.advance() {
                Some(v) => v,
                _ => {return left}
            };
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
                return Some(Expr::Number(super::num::Num::new("-1.0", vec![])));
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
                            _ => None
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
