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
            ("to"|"in"|"convert"|"convert_to", Some(num1), Some(num2)) => {
                return match convert(&num1, Expr::Number(num2.clone())) {
                    Some(output) => {Ok(Expr::Number(output))},
                    _ => Err(String::from("Conversion Impossible!")),
                }
            }
            ("sqrt"|"root2"|"2root", num1, Some(num2)) => {
                let ans = num2.powf(&Num::unitless("0.5")).unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("sin"|"sine", num1, Some(num2)) => {
                let ans = num2.sin().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("cos"|"cosine", num1, Some(num2)) => {
                let ans = num2.cos().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("tan"|"tangent", num1, Some(num2)) => {
                let ans = num2.tan().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("arcsin"|"arcsine"|"asin", num1, Some(num2)) => {
                let ans = num2.arcsin().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("arccos"|"arccosine"|"acos", num1, Some(num2)) => {
                let ans = num2.arccos().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("arctan"|"arctangent"|"atan", num1, Some(num2)) => {
                let ans = num2.arctan().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("ln", num1, Some(num2)) => {
                let ans = num2.log(&crate::calc::tokenize::misc_units::unit_to_num("e").unwrap()).unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("lg"|"log10", num1, Some(num2)) => {
                let ans = num2.log(&Num::unitless("10")).unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("log2", num1, Some(num2)) => {
                let ans = num2.log(&Num::unitless("2")).unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("exp", num1, Some(num2)) => {
                let ans = num2.exp().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("floor"|"round_down"|"rdown"|"roundd", num1, Some(num2)) => {
                let ans = num2.floor().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("ceil"|"round_up"|"ceiling"|"rup"|"roundu", num1, Some(num2)) => {
                let ans = num2.ceil().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("round", num1, Some(num2)) => {
                let ans = num2.round().unwrap();
                return mul_ans_or_ans(num1, ans);
            }
            ("abs"|"absolute"|"absolutes", num1, Some(num2)) => {
                let ans = num2.abs().unwrap();
                return mul_ans_or_ans(num1, ans);
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
