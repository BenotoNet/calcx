use crate::calc::Calc;
use crate::calc::{num::Num, expr::Expr};
use crate::calc::conversion::convert;

impl Calc {
    pub fn eval_keyword(&self, key: &str, num1: &Num, num2: &Num) -> Result<Expr, String> {
        match key {
            "to"|"in" => {
                return match convert(&num1, Expr::Number(num2.clone())) {
                    Some(output) => {Ok(Expr::Number(output))},
                    _ => Err(String::from("Conversion Impossible!")),
                }
            }
            "sqrt" => {
                return Ok(Expr::Number(num1.mul(&num2.powf(&Num::unitless(0.5)).unwrap()).unwrap()));
            }
            "sin" => {
                return Ok(Expr::Number(num1.mul(&num2.sin().unwrap()).unwrap()))
            }
            "cos" => {
                return Ok(Expr::Number(num1.mul(&(num2.add(&Num::unitless(1.57079632679489661923132169163975144209858469968755)).unwrap()).sin().unwrap()).unwrap()))
            }
            // "arcsin" => {
            //     // TODO: add arcsine function
            // }
            _ => Err(String::from("Unknown Keyword! (Maybe not implemented yet?)")),
        }
    }
}
