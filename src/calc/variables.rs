use crate::calc::num::Num;

use super::expr::Expr;

pub fn get_var(_var: String) -> Expr {
    Expr::Number(Num::unitless(1.0))
}
