use crate::calc::Calc;
use crate::calc::{num::Num, expr::Expr};
use crate::calc::conversion::convert;

impl Calc {
    pub fn eval_keyword(&self, key: &str, num1: Option<&Num>, num2: Option<&Num>) -> Result<Expr, String> {
        match key {
            "to"|"in" => {
                return match convert(&num1.unwrap(), Expr::Number(num2.unwrap().clone())) {
                    Some(output) => {Ok(Expr::Number(output))},
                    _ => Err(String::from("Conversion Impossible!")),
                }
            }
            "sqrt" => {
                return Ok(Expr::Number(num1.unwrap().mul(&num2.unwrap().powf(&Num::unitless(0.5)).unwrap()).unwrap()));
            }
            "sin" => {
                return Ok(Expr::Number(num1.unwrap().mul(&num2.unwrap().sin().unwrap()).unwrap()))
            }
            "cos" => {
                return Ok(Expr::Number(num1.unwrap().mul(&(num2.unwrap().add(&Num::unitless(1.57079632679489661923132169163975144209858469968755)).unwrap()).sin().unwrap()).unwrap()))
            }
            "ans" => {
                match (self.get_ans(), num1, num2) {
                    (Some(Expr::Number(num3)), Some(num1), Some(num2)) => Ok(Expr::Number(num1.mul(&num2.mul(&num3).unwrap()).unwrap())),
                    _ => Err(String::from("The last answer is not accessible")),
                }
            }
            // "arcsin" => {
            //     // TODO: add arcsine function
            // }
            _ => Err(String::from("Unknown Keyword! (Maybe not implemented yet?)")),
        }
    }
}
