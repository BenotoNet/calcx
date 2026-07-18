use super::Calc;
use super::expr::Expr;
use super::num::Num;
use super::token::Token;


impl Calc {
    pub fn build_tree(&mut self) -> Result<Expr, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_keywords();

        while
            match self.peek() {
            Ok(Token::Add) => true,
            Ok(Token::Sub) => {
                // Decide by context if minus should be considered an operation or a negate sign
                // FIX: Here, I need to still figure out how to multiply by -1 if the negative sign
                // is unary => just a sign and not an operation
                match self.last_token() {
                    Err(_)|Ok(Token::LBrac|Token::Add|Token::Sub|Token::Mod|Token::Seperator|Token::Div|Token::Mul|Token::Pow|Token::Assign) => false,
                    _ => true
                }
            }
            _ => false,
        } {
            let operation = self.advance()?;
            let right = self.parse_keywords();
            left = Ok(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) });
        }
        return left;
    }

    fn parse_keywords(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term();

        while 
            match self.peek() {
                Ok(Token::Keyword(_)) => true,
                Ok(Token::Func(_)) => {
                    let func_token = self.advance()?;
                    let args = self.parse_function_arguments();
                    left = Ok(Expr::Binary { left: Box::new(left), op: func_token, right: Box::new(args) });
                    true
                },
                _ => false,
            }
        {
            // We have found a Keyword
            let keyword = match self.advance() {
                Ok(Token::Keyword(key)) => Token::Keyword(key),
                _ => {return left}
            };
            let right = self.parse_term();

            left = Ok(Expr::Binary { left: Box::new(left), op: keyword, right: Box::new(right) });
        }
        return left;
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bracket();

        while
            match self.peek() {
                Ok(Token::Mul)|Ok(Token::Div) => true,
                _ => false,
        } {
            let operation = match self.advance() {
                Ok(v) => v,
                _ => return left,
            };
            let right = self.parse_bracket();
            left = Ok(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) });
        }
        return left;
    }

    fn parse_bracket(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_exponents();

        while match self.peek() {
            Ok(Token::LBrac) => true,
            _ => {false},
        } {
            let operation = Token::Mul;
            let right = self.parse_exponents();
            left = Ok(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) })
        }
        return left;
    }

    fn parse_exponents(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor();

        while
            match self.peek() {
                Ok(Token::Mod)|Ok(Token::Pow) => true,
                _ => false,
            } {
                let operation = self.advance()?;
                let right = self.parse_factor();
                left = Ok(Expr::Binary { left: Box::new(left), op: operation, right: Box::new(right) });
            }
        return left;
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Ok(Token::Number(num)) => {self.advance()?; Ok(Expr::Number(num))},
            // Bracket Logic => Starting a new branch
            Ok(Token::LBrac) => {
                self.advance()?;
                let expr = self.parse_expression()?;

                // EXPECT RBrac
                self.expect(Token::RBrac)?;

                return Ok(expr);
            }
            // We have a negate-sign, therefore, we multiply with -1
            Ok(Token::Sub) => {
                self.advance()?;
                return Ok(Expr::Binary { 
                    left: Box::new(Ok(Expr::Number(Num::unitless("-1")))), 
                    op: Token::Mul, 
                    right: Box::new(self.parse_term()) })
            }
            // We found a variable!
            Ok(Token::Var(var)) => {
                self.advance()?;
                match self.peek() {
                    Ok(Token::Assign) => {
                            self.advance()?;
                            let right = self.parse_expression();
                            match self.eval(right) {
                                Ok(Expr::Number(num)) => {
                                    self.variables.set_var(var, num.clone());
                                    Ok(Expr::Number(num))
                                },
                                _ => return Err(String::from("Could not evaluate right side of assignment!"))
                            }
                    }
                    _ => {
                        return self.variables.get_var(var)
                    }
                }
            }
            _ => {
                // an unknown Token!
                return Err(String::from("Operation not possible"));
            },
        }
    }

    pub fn parse_function_arguments(&mut self) -> Result<Expr, String> {
        let mut args = vec![];

        // Splitting Arguments with ','
        let mut temp_arg: Vec<Token> = vec![];
        let mut brackets = 0;

        // First opening Bracket -> Opening Bracket should not be added to the calculation, but is optional nonetheless
        match self.peek() {
            Ok(Token::LBrac) => {
                brackets += 1;
                self.advance()?;
            }
            _ => {}
        }

        while match self.advance() {
            Ok(Token::Seperator) => {
                if brackets <= 1 {
                    // one argument finished, appending to args, then continue (but only if we are
                    // in the root bracket structure)
                    args.push(temp_arg.clone());
                    temp_arg = vec![];
                    true
                }
                // We are not at the root branch yet, so just append
                else {
                    temp_arg.push(Token::Seperator);
                    true
                }
            }
            // Increase Bracket count by one, so that when we have the matching RBrac, we don't stop
            // parsing args, e.g. func((5+2), 2)
            // Also add the brackets to the argument => ((5+2) / 2) otherwise becomes 5 + 2 / 2 
            //    -> Different results
            Ok(Token::LBrac) => {temp_arg.push(Token::LBrac); brackets += 1; true},
            // Check if we are at the last closing bracket, then append final arg and stop
            Ok(Token::RBrac) => {brackets -= 1; 
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
            Err(_) => {
                args.push(temp_arg.clone());
                false
            }

            // we got anything other than specified above, then append if not none and continue
            var => {
                // If we do not have brackets (-> originally brackets var is 0) we should stop right
                // here...
                match (brackets<=0, var) {
                    (true, Ok(v)) => {
                        temp_arg.push(v);
                        args.push(temp_arg.clone());
                        false
                    },
                    (false, Ok(v)) => {
                        temp_arg.push(v);
                        true
                    },
                    _ => true,
                }
            }
        } {};

        let mut out = Err(String::from("No Arguments"));
        for arg in args {
            // println!{"{arg:?}"}
            out = Ok(Expr::Arg {arg: Box::new(self.build_tree_from(arg.clone())), right: Box::new(out) });
        }

        // Undo the last going forward
        self.rewind();

        out
    }

}
