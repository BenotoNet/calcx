use crate::{Calc, Num, Expr};
use crate::conversion::convert;

impl Calc {
    pub fn eval_keyword(&self, key: &str, num1: Result<&Num, String>, num2: Result<&Num, String>) -> Result<Expr, String> {
        match (key, num1, num2) {
            // The last few functions actually requiring both sides of the current expression
            ("to"|"in"|"convert"|"convert_to", Ok(num1), Ok(num2)) => {
                return match convert(&num1, Expr::Number(num2.clone())) {
                    Ok(output) => {Ok(Expr::Number(output))},
                    Err(v) => Err(v),
                }
            }
            ("ans"|"last", num1, num2) => {
                let mut last_answer = match self.get_ans() {
                    Ok(Expr::Number(num3)) => {num3}
                    _ => {return Err(String::from("Last Answer not accessible!"))},
                };
                match num1 {
                    Ok(num) => {last_answer = last_answer.mul(num)?},
                    _ => {}
                };
                match num2 {
                    Ok(num) => {last_answer = last_answer.mul(num)?},
                    _ => {}
                };
                return Ok(Expr::Number(last_answer));
            }
            // Copying last answer to clipboard:
            ("clip"|"copy"|"clipboard", _, _) => {
                return self.get_ans();
            }
            _ => Err(String::from("Unknown Keyword or not enough Arguments! (Maybe not implemented yet?)")),
        }
    }
}
