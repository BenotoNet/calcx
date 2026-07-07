use crate::calc::Calc;
use crate::calc::expr::Expr;


pub fn is_function(token_str: &str) -> bool {
    true
}

impl Calc {
    pub fn eval_function(&self, func: &str) -> Result<Expr, String> {
        return Err(String::new());
    }
}
