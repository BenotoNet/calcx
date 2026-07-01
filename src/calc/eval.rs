use super::Calc;
use super::{expr::Expr, token::Token, num::Num};

impl Calc {
    fn eval_atomic(&self, num1: Option<&Num>, op: Token, num2: Option<&Num>) -> Result<Expr, String> {
        // println!{"{num1:?}, {op:?}, {num2:?}"};
        match op {
            // When we have simple operations between the two numbers, we simply apply the operation
            // FIX: Remove all Unwraps
            Token::Add => {Ok(Expr::Number(num1.unwrap().add(&num2.unwrap()).unwrap()))},
            Token::Sub => {Ok(Expr::Number(num1.unwrap().sub(&num2.unwrap()).unwrap()))},
            Token::Mul => {Ok(Expr::Number(num1.unwrap().mul(&num2.unwrap()).unwrap()))},
            Token::Div => {Ok(Expr::Number(num1.unwrap().div(&num2.unwrap()).unwrap()))},
            Token::Mod => {Ok(Expr::Number(num1.unwrap().modf(&num2.unwrap()).unwrap()))},
            Token::Pow => {Ok(Expr::Number(num1.unwrap().powf(&num2.unwrap()).unwrap()))},

            // We found a Keyword
            Token::Keyword(key) => {
                self.eval_keyword(key.as_str(), num1, num2)
            }

            _ => {println!{"{op:?}"}; Err(String::from("Not an Operator / Operation!"))},
        }

    }

    pub fn eval(&self, tree: Option<Expr>) -> Result<Expr, String> {
        match tree {
            Some(Expr::Binary { left, op, right }) => {
                let left = self.eval(*left);
                let right = self.eval(*right);

                match (left, right) {
                    (Ok(Expr::Number(num1)), Ok(Expr::Number(num2))) => {
                        // We have an atomic Expression (only numbers on both sides of the operator)
                        return self.eval_atomic(Some(&num1), op, Some(&num2));
                    },

                    (Ok(Expr::Number(num1)), Err(_)) => {
                        // Right Eval failed
                        return self.eval_atomic(Some(&num1), op, None);
                    }

                    (Err(_), Ok(Expr::Number(num2))) => {
                        // Left Eval failed
                        return self.eval_atomic(None, op, Some(&num2));
                    }

                    (Err(_), Err(_)) => {
                        // Both sides failed
                        return self.eval_atomic(None, op, None);
                    }

                    (left, right) => Ok(Expr::Binary { 
                        // Not atomic yet, so evaluate: 
                        left: Box::new(Some(left?)), 
                        op, 
                        right: Box::new(Some(right?)),
                    }),
                }
            }
            Some(v) => {Ok(v)}
            _ => Err(String::from("idk"))
        }
    }
}
