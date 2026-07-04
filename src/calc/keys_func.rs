use cli_clipboard::set_contents;

use crate::calc::Calc;
use crate::calc::{num::Num, expr::Expr};
use crate::calc::conversion::convert;

impl Calc {
    pub fn eval_keyword(&self, key: &str, num1: Option<&Num>, num2: Option<&Num>) -> Result<Expr, String> {
        match (key, num1, num2) {
            ("to"|"in"|"convert"|"convert_to", Some(num1), Some(num2)) => {
                return match convert(&num1, Expr::Number(num2.clone())) {
                    Some(output) => {Ok(Expr::Number(output))},
                    _ => Err(String::from("Conversion Impossible!")),
                }
            }
            ("sqrt"|"root2", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&num2.powf(&Num::unitless("0.5")).unwrap()).unwrap()},
                            _ => {num2.powf(&Num::unitless("0.5")).unwrap()},
                        }
                        )
                    );
            }
            ("sin"|"sine", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&num2.sin().unwrap()).unwrap()},
                            _ => {num2.sin().unwrap()},
                        }
                        )
                    );
            }
            ("cos"|"cosine", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&(num2.add(&Num::unitless("1.57079632679489661923132169163975144209858469968755")).unwrap()).sin().unwrap()).unwrap()},
                            _ => {(num2.add(&Num::unitless("1.57079632679489661923132169163975144209858469968755")).unwrap()).sin().unwrap()},
                        }
                        )
                    );
            }
            ("tan"|"tangent", num1, Some(num2)) => {
                let ans = num2.tan().unwrap();

                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("arcsin"|"arcsine"|"asin", num1, Some(num2)) => {
                let ans = num2.arcsin().unwrap();

                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("arccos"|"arccosine"|"acos", num1, Some(num2)) => {
                let ans = num2.arccos().unwrap();

                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("arctan"|"arctangent"|"atan", num1, Some(num2)) => {
                let ans = num2.arctan().unwrap();

                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("ln", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {
                                num1.mul(&(num2.log(&crate::calc::tokenize::misc_units::unit_to_num("e").unwrap())).unwrap()).unwrap()
                            }
                            _ => {num2.log(&crate::calc::tokenize::misc_units::unit_to_num("e").unwrap()).unwrap()},
                        }
                    )
                )
            }
            ("lg"|"log10", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {
                                num1.mul(&(num2.log(&Num::unitless("10"))).unwrap()).unwrap()
                            }
                            _ => {num2.log(&Num::unitless("10")).unwrap()},
                        }
                    )
                )
            }
            ("log2", num1, Some(num2)) => {
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {
                                num1.mul(&(num2.log(&Num::unitless("2"))).unwrap()).unwrap()
                            }
                            _ => {num2.log(&Num::unitless("2")).unwrap()},
                        }
                    )
                )
            }
            ("exp", num1, Some(num2)) => {
                let ans = num2.exp().unwrap();
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("floor"|"round_down"|"rdown"|"roundd", num1, Some(num2)) => {
                let ans = num2.floor().unwrap();
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("ceil"|"round_up"|"ceiling"|"rup"|"roundu", num1, Some(num2)) => {
                let ans = num2.ceil().unwrap();
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("round", num1, Some(num2)) => {
                let ans = num2.round().unwrap();
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("abs"|"absolute"|"absolutes", num1, Some(num2)) => {
                let ans = num2.abs().unwrap();
                return Ok(
                    Expr::Number(
                        match num1 {
                            Some(num1) => {num1.mul(&ans).unwrap()},
                            _ => ans,
                        }
                    )
                )
            }
            ("ans"|"last", num1, num2) => {
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
            ("clip"|"copy"|"clipboard", None, None) => {
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
