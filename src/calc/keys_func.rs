use cli_clipboard::set_contents;

use crate::calc::Calc;
use crate::calc::{num::Num, expr::Expr};
use crate::calc::conversion::convert;

impl Calc {
    pub fn eval_keyword(&self, key: &str, num1: Option<&Num>, num2: Option<&Num>) -> Result<Expr, String> {
        let mul_ans_or_ans = |num: Option<&Num>, ans| -> Result<Expr, String> {
            return Ok(
                Expr::Number(
                    match num {
                        Some(num) => {num.mul(&ans).unwrap()},
                        _ => ans,
                    }
                )
            )
        };

        match (key, num1, num2) {
            // The last few functions actually requiring both sides of the current expression
            ("to"|"in"|"convert"|"convert_to", Some(num1), Some(num2)) => {
                return match convert(&num1, Expr::Number(num2.clone())) {
                    Some(output) => {Ok(Expr::Number(output))},
                    _ => Err(String::from("Conversion Impossible!")),
                }
            }
            ("ans"|"last", num1, num2) => {
                let mut last_answer = match self.get_ans() {
                    Some(Expr::Number(num3)) => {num3}
                    _ => {return Err(String::from("Last Answer not accessible!"))},
                };
                match num1 {
                    Some(num) => {last_answer = last_answer.mul(num).unwrap()},
                    _ => {}
                };
                match num2 {
                    Some(num) => {last_answer = last_answer.mul(num).unwrap()},
                    _ => {}
                };
                return Ok(Expr::Number(last_answer));
            }
            // Copying last answer to clipboard:
            ("clip"|"copy"|"clipboard", _, _) => {
                return match self.get_ans() {
                    Some(v) => {
                        // Copying
                        set_contents(v.display(self.precision)).unwrap();

                        Ok(v)
                    },
                    _ => Err(String::from("Could not copy last answer to Clipboard, maybe not accessible")),
                }
            }
            _ => Err(String::from("Unknown Keyword Or Not enough Arguments! (Maybe not implemented yet?)")),
        }
    }
}
