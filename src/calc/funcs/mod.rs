use crate::calc::expr::Expr;
use crate::calc::num::Num;

// Horrible Way, but works -> Evaluating an argument by using a new calculator instance
fn eval_argument(arg: Expr) -> Result<Expr, String> {
    let temp_calc = crate::calc::Calc::new(crate::PRECISION as usize);
    temp_calc.eval(Some(arg))
}

// When should it be a function, and when a keyword / Variable (-> Continue Categorization)?
pub fn is_function(token_str: &str) -> bool {
    match run_func(token_str, vec![]) {
        Err(v) => v.as_str() != "Not a Function",
        _ => true
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
    return run_func(func_str, args);
}

fn run_func(func_str: &str, args: Vec<Expr>) -> Result<Expr, String> {

    // Eval each argument
    let args: Vec<Result<Expr, String>> = args.iter().map(|arg| {eval_argument(arg.clone())}).collect();
    return match func_str {
        // FIX: Here, we need to make a better system for precicely doing functions with arguments
        // We should give different errors for too few Arguments and not having a valid function
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
