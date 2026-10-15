use crate::Calc;
use crate::Expr;
use crate::Num;
use crate::tokens::Token;


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
                let right = self.parse_exponents();
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
                    right: Box::new(self.parse_exponents()) })
            }
            // We found a variable!
            Ok(Token::Var(var)) => {
                self.advance()?;
                // Next, we see if there is an assign keyword LATER in the query:
                if self.contains_later(Token::Assign) {
                    // We either have a function or variable definition
                    // if the next token is an assign sign, it's a variable definition
                    match self.advance()? {
                        Token::Assign => {
                            let right = self.parse_expression();
                            match self.eval(right) {
                                Ok(Expr::Number(num)) => {
                                    self.variables.set_var(var, num.clone());
                                    Ok(Expr::Number(num))
                                },
                                _ => return Err(String::from("Could not evaluate right side of assignment"))
                            }
                        },
                        Token::LBrac => {
                            // We have a function assignment / definition

                            let mut variable_list_ordered = vec![];

                            // First, see what input variables we have
                            while match self.advance()? {
                                Token::Assign => {
                                    // We are at the end, now the right side has to be parsed
                                    false
                                }, 
                                Token::Var(v) => {variable_list_ordered.push(v); true},
                                _ => {true},
                            } {}

                            fn token_replace(tokens: &Vec<Token>, from: Token, to: Token) -> Vec<Token> {
                                let mut output_tokens = tokens.clone();
                                let mut index = 0;
                                while index < output_tokens.len() {
                                    if output_tokens[index..=index].contains(&from) {
                                        output_tokens[index] = to.clone();
                                    }
                                    index += 1;
                                }
                                output_tokens
                            }

                            // Replace all Variables with "0[respective arg number]"
                            for (i, v) in variable_list_ordered.iter().enumerate() {
                                self.tokens = token_replace(&self.tokens, Token::Var(v.clone()), Token::Var(format!{"0{i}"}))
                            }

                            let right = self.parse_expression();
                            
                            self.userfunctions.set_func(var, right)?;
                            Ok(Expr::Number(Num::unitless("1.0")))
                        },
                        _ => {return Err(String::from("Something is wrong, there is an assign sign at the wrong place"))}
                    }
                }
                else {
                    // we just have a normal variable

                    // It might be a custom function argument, therefore we test if the first
                    // character is a 0
                    if var.chars().nth(0) == Some('0') {
                        // It's an argument for a function, just pass back as variable
                        return Ok(Expr::Binary { 
                            left: Box::new(Err(String::from("Please report if you see this error"))), 

                            // This is the important part
                            op: Token::Var(var), 

                            right: Box::from(Err(String::from("Please report if you see this error")))
                        })
                    }
                    // Otherwise it really is just a normal variable
                    return self.variables.get_var(var)
                }
            }
            _ => {
                // an unknown Token!
                return Err(String::from("Operation is not possible, there is an unknown token"));
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
