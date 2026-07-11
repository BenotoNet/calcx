use crate::calc::expr::Expr;
use crate::calc::num::Num;

// Horrible Way, but works -> Evaluating an argument by using a new calculator instance
fn eval_argument(arg: Expr) -> Result<Expr, String> {
    let temp_calc = crate::calc::Calc::new(crate::PRECISION as usize);
    temp_calc.eval(Some(arg))
}

// FIX: When should it be a function, and when a keyword / Variable?
pub fn is_function(token_str: &str) -> bool {
    match token_str {
        "add_one" => true,
        _ => false,
    }
}

pub fn unwrap_args(mut args: Option<Expr>) -> Vec<Expr> {
    let mut output = vec![];
    loop {
        match args {
            Some(Expr::Arg { ref arg, ref right }) => {
                match *arg.clone() {
                    Some(v) => output.push(v),
                    _ => {},
                }
                args = *right.clone()
            }
            _ => {break}
        }
    }
    output.reverse();
    output
}

pub fn func_call(func_str: &str, args: Option<Expr>) -> Result<Expr, String> {
    let args = unwrap_args(args); // This function unwraps the Arguments into a simple array of
                                  // expressions
    match func_str {
        "add_one" => {
            match eval_argument(args.get(0).unwrap().clone()) {
                Ok(Expr::Number(num)) => {
                    return Ok(Expr::Number(num.add(&Num::unitless("1.0")).unwrap()));
                }
                _ => {}
            };
        },
        _ => {},
    }
    Err(String::new())
}
