use super::Calc;
use super::{expr::Expr, token::Token, num::Num};
use crate::calc::conversion;

// TODO: Split up into smaller parts!

impl Calc {
    fn eval_atomic(&self, num1: &Num, op: Token, num2: &Num) -> Result<Expr, &str> {
        match op {
            // When we have simple operations between the two numbers, we simply apply the operation
            Token::Add => {Ok(Expr::Number(num1.add(&num2).unwrap()))},
            Token::Sub => {Ok(Expr::Number(num1.sub(&num2).unwrap()))},
            Token::Mul => {Ok(Expr::Number(num1.mul(&num2).unwrap()))},
            Token::Div => {Ok(Expr::Number(num1.div(&num2).unwrap()))},
            Token::Mod => {Ok(Expr::Number(num1.modf(&num2).unwrap()))},
            Token::Pow => {Ok(Expr::Number(num1.powf(&num2).unwrap()))},

            // We found a Keyword
            Token::Keyword(key) => {
                match key.as_str() {
                    "to"|"in" => {
                        // FIX: WTF AM I EVEN CODING RIGHT NOW?!
                        match conversion::convert(&num1, "micro farad") {
                            Some(output) => {println!{"{output}"}},
                            _ => {println!{"Conversion impossible"}},
                        }
                    }
                    _ => {},
                }
                return Ok(Expr::Number(Num::unitless(0.0)));
            }

            _ => Err("Not an Operator!"),
        }

    }

    pub fn eval(&self, tree: Expr) -> Result<Expr, &str> {
        match tree {
            Expr::Binary { left, op, right } => {
                let left = self.eval(*left)?;
                let right = self.eval(*right)?;

                match (left, right) {
                    (Expr::Number(num1), Expr::Number(num2)) => {
                        // We have an atomic Expression (only numbers on both sides of the operator)
                        return self.eval_atomic(&num1, op, &num2);
                    },

                    (left, right) => Ok(Expr::Binary { 
                        // Not atomic yet, so evaluate: 
                        left: Box::new(left), 
                        op, 
                        right: Box::new(right),
                    }),
                }
            }
            _ => Ok(tree)
        }
    }
}
