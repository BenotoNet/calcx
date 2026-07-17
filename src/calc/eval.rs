use crate::calc::funcs;
use super::Calc;
use super::{expr::Expr, token::Token, num::Num};

impl Calc {
    fn eval_atomic(&self, num1: Result<&Num, String>, op: Token, num2: Result<&Num, String>) -> Result<Expr, String> {
        // println!{"{num1:?}, {op:?}, {num2:?}"};
        match op {
            // When we have simple operations between the two numbers, we simply apply the operation
            Token::Add => {Ok(Expr::Number(num1?.add(&num2.cloned()?)?))},
            Token::Sub => {Ok(Expr::Number(num1?.sub(&num2.cloned()?)?))},
            Token::Mul => {Ok(Expr::Number(num1?.mul(&num2.cloned()?)?))},
            Token::Div => {Ok(Expr::Number(num1?.div(&num2.cloned()?)?))},
            Token::Mod => {Ok(Expr::Number(num1?.modf(&num2.cloned()?)?))},
            Token::Pow => {Ok(Expr::Number(num1?.powf(&num2.cloned()?)?))},

            // We found a Keyword
            Token::Keyword(key) => {
                self.eval_keyword(key.as_str(), num1, num2)
            }

            _ => {println!{"Error... {op:?}"}; Err(String::from("Not an Operator / Operation!"))},
        }

    }

    pub fn eval(&self, tree: Result<Expr, String>) -> Result<Expr, String> {
        match tree {
            Ok(Expr::Binary { left, op, right }) => {
                // if we have a function as the operator, run the code with given arguments
                match op {
                    Token::Func(func) => {
                        // We run the function code and also pass along the arguments
                        return funcs::func_call(&func, *right);
                    }
                    _ => {}
                }

                let left = self.eval(*left);
                let right = self.eval(*right);

                match (left, right) {
                    (Ok(Expr::Number(num1)), Ok(Expr::Number(num2))) => {
                        // We have an atomic Expression (only numbers on both sides of the operator)
                        return self.eval_atomic(Ok(&num1), op, Ok(&num2));
                    },

                    (Ok(Expr::Number(num1)), Err(v)) => {
                        // Right Eval failed
                        return self.eval_atomic(Ok(&num1), op, Err(v));
                    }

                    (Err(v), Ok(Expr::Number(num2))) => {
                        // Left Eval failed
                        return self.eval_atomic(Err(v), op, Ok(&num2));
                    }

                    (Err(v1), Err(v2)) => {
                        // Both sides failed
                        return self.eval_atomic(Err(v1), op, Err(v2));
                    }

                    (left, right) => Ok(Expr::Binary { 
                        // Not atomic yet, so evaluate: 
                        left: Box::new(Ok(left?)), 
                        op, 
                        right: Box::new(Ok(right?)),
                    }),
                }
            }
            Ok(v) => {Ok(v)}
            Err(v) => Err(v),
        }
    }
}
