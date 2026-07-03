use cli_clipboard::set_contents;

use crate::calc::Calc;
use crate::calc::{num::Num, expr::Expr};
use crate::calc::conversion::convert;

impl Calc {
    pub fn eval_keyword(&self, key: &str, num1: Option<&Num>, num2: Option<&Num>) -> Result<Expr, String> {
        match (key, num1, num2) {
            ("to"|"in", Some(num1), Some(num2)) => {
                return match convert(&num1, Expr::Number(num2.clone())) {
                    Some(output) => {Ok(Expr::Number(output))},
                    _ => Err(String::from("Conversion Impossible!")),
                }
            }
            ("sqrt", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&num2.powf(&Num::unitless("0.5")).unwrap()).unwrap()},
                            _ => {num2.powf(&Num::unitless("0.5")).unwrap()},
                        }
                        )
                    );
            }
            ("sin", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&num2.sin().unwrap()).unwrap()},
                            _ => {num2.sin().unwrap()},
                        }
                        )
                    );
            }
            ("cos", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&(num2.add(&Num::unitless("1.57079632679489661923132169163975144209858469968755")).unwrap()).sin().unwrap()).unwrap()},
                            _ => {(num2.add(&Num::unitless("1.57079632679489661923132169163975144209858469968755")).unwrap()).sin().unwrap()},
                        }
                        )
                    );
            }
            ("ans", num1, num2) => {
                let mut last_answer = match self.get_ans() {
                    Some(Expr::Number(num3)) => {num3}
                    _ => {return Err(String::from("Last Answer not accessible!"))},
                };
                match num1 {
                    Some(num) => {last_answer = last_answer.mul(num).unwrap()},
                    None => {}
                };
                match num2 {
                    Some(num) => {last_answer = last_answer.mul(num).unwrap()},
                    None => {}
                };
                return Ok(Expr::Number(last_answer));
            }
            // Copying last answer to clipboard:
            ("clip"|"copy", None, None) => {
                return match self.get_ans() {
                    Some(v) => {
                        // Copying
                        set_contents(v.display(self.precision)).unwrap();

                        Ok(v)
                    },
                    _ => Err(String::from("Could not copy last answer to Clipboard, maybe not accessible")),
                }
            }
            // "arcsin" => {
            //     // TODO: add arcsine function
            // }
            _ => Err(String::from("Unknown Keyword Or Not enough Arguments! (Maybe not implemented yet?)")),
        }
    }
}
