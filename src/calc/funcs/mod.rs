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
        "add_one"|","|"root"|"nth_root"|"n_root" => true,
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
    // Eval each argument
    let args: Vec<Result<Expr, String>> = args.iter().map(|arg| {eval_argument(arg.clone())}).collect();
    let wa = || {Err(String::from("Error: Something went wrong"))};
    return match (func_str, args.len()) {
        ("add_one", 1) => {
            match &args[0] {
                Ok(Expr::Number(num)) => {
                    Ok(Expr::Number(num.add(&Num::unitless("1.0")).unwrap()))
                }
                _ => {wa()}
            }
        },
        ("root"|"nth_root"|"n_root", 2) => {
            match (&args[0], &args[1]) {
                (Ok(Expr::Number(root)), Ok(Expr::Number(base))) => {
                    Ok(Expr::Number(base.powf(&Num::unitless("1.0").div(root).unwrap()).unwrap()))
                }
                _ => {wa()}
            }
        }
        _ => {Err(String::from("Error: Wrong Number of Arguments or not a function!"))},
    }
    }
