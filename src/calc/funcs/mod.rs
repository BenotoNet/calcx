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

fn expect(args: &Vec<Num>, num: usize) -> Result<(), String> {
    if args.len() == num {Ok(())}
    else if args.len() < num {Err(String::from("Too few Arguments"))}
    else {Err(String::from("Too many Arguments!"))}
}

fn run_func(func_str: &str, args: Vec<Expr>) -> Result<Expr, String> {

    // Eval each argument
    let args: Vec<Result<Expr, String>> = args.iter().map(|arg| {eval_argument(arg.clone())}).collect();

    // If one argument resulted in an Error, return that error: 
    let mut new_args = vec![];
    for arg in args {
        match arg {
            Ok(Expr::Number(num)) => new_args.push(num),
            Err(err) => return Err(err),
            _ => return Err(String::from("There was no Number in the Arguments!")),
        }
    }
    let args = new_args;

    return match func_str {
        "add_one" => {
            expect(&args, 1)?;
            Ok(Expr::Number(args[0].add(&Num::unitless("1.0")).unwrap()))
        },

        "root"|"nth_root"|"n_root" => {
            expect(&args, 2)?;
            Ok(Expr::Number(args[1].powf(&Num::unitless("1.0").div(&args[0]).unwrap()).unwrap()))
        }

        _ => {Err(String::from("Not a Function"))},
    }
}
