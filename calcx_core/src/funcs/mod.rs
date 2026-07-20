use crate::Expr;
use crate::Num;
use crate::Calc;
use crate::tokens::misc_units::unit_to_num;

// Horrible Way, but works -> Evaluating an argument by using a new calculator instance
// FIX: Please solve any other way!
fn eval_argument(arg: Expr) -> Result<Expr, String> {
    let temp_calc = Calc::new(crate::PRECISION as usize);
    temp_calc.eval(Ok(arg))
}

// When should it be a function, and when a keyword / Variable (-> Continue Categorization)?
pub fn is_function(token_str: &str) -> bool {
    match run_func(token_str, vec![]) {
        Err(v) => v.as_str() != "Not a Function",
        _ => true
    }
}

fn unwrap_args(mut args: Result<Expr, String>) -> Vec<Expr> {
    let mut output = vec![];
    loop {
        match args {
            Ok(Expr::Arg { ref arg, ref right }) => {
                match *arg.clone() {
                    Ok(v) => output.push(v),
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

pub fn func_call(func_str: &str, args: Result<Expr, String>) -> Result<Expr, String> {
    let args = unwrap_args(args); // This function unwraps the Arguments into a simple array of
                                  // expressions
    return run_func(func_str, args);
}

fn expect(args: &Vec<Num>, num: usize, needs_unitless: bool) -> Result<(), String> {
    if args.iter().any(|arg| {!arg.is_unitless()}) && needs_unitless {Err(String::from("Argument is required to be dimensionless (unitless)"))}
    else if args.len() < num {Err(String::from("Too few Arguments"))}
    else if args.len() > num {Err(String::from("Too many Arguments!"))}
    else {
        Ok(())
    }
}

fn expect_unitless(args: &Vec<Num>) -> Result<(), String> {
    if args.iter().any(|arg| {!arg.is_unitless()}) {Err(String::from("Argument is required to be dimensionless (unitless)"))}
    else {
        Ok(())
    }
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

    let wrap = |num| {return Ok(Expr::Number(num))};

    return match func_str {
        // Returns input value; used for testing
        "test" => {
            expect(&args, 1, false)?;
            wrap(args[0].clone())
        }
        "add_one" => {
            expect(&args, 1, true)?;
            wrap(args[0].add(&Num::unitless("1.0"))?)
        },

        "root"|"nth_root"|"n_root" => {
            expect(&args, 2, true)?;
            wrap(args[1].powf(&Num::unitless("1.0").div(&args[0])?)?)
        }
        "log" => {
            expect(&args, 2, true)?;
            wrap(args[1].log(&args[0])?)
        }

        // All ported functions (like sqrt)
        "sqrt"|"square_root"|"2root"|"root2" => {
            expect(&args, 1, false)?;
            wrap(args[0].powf(&Num::unitless("0.5"))?)
        }
        "sin"|"sine" => {
            expect(&args, 1, true)?;
            wrap(args[0].sin()?)
        }
        "cos"|"cosine" => {
            expect(&args, 1, true)?;
            wrap(args[0].cos()?)
        }
        "tan"|"tangent" => {
            expect(&args, 1, true)?;
            wrap(args[0].tan()?)
        }
        "arcsine"|"arcsin"|"asin"|"asine" => {
            expect(&args, 1, true)?;
            wrap(args[0].arcsin()?)
        }
        "arccosine"|"arccos"|"arcos"|"acos"|"acosine" => {
            expect(&args, 1, true)?;
            wrap(args[0].arccos()?)
        }
        "arctan"|"arctangent"|"atan"|"atangent" => {
            expect(&args, 1, true)?;
            wrap(args[0].arctan()?)
        }
        "ln"|"natural_log"|"natural_ln"|"log_natural" => {
            expect(&args, 1, true)?;
            wrap(args[0].log(&unit_to_num("e")?)?)
        }
        "lg"|"log10"|"10log"|"log_base_10" => {
            expect(&args, 1, true)?;
            wrap(args[0].log(&Num::unitless("10"))?)
        }
        "log2"|"2log"|"log_base_2" => {
            expect(&args, 1, true)?;
            wrap(args[0].log(&Num::unitless("2"))?)
        }
        "exp" => {
            expect(&args, 1, true)?;
            wrap(args[0].exp()?)
        }
        "round_down"|"floor"|"rdown"|"roundd" => {
            expect(&args, 1, true)?;
            wrap(args[0].floor()?)
        }
        "round_up"|"ceil"|"rup"|"roundu"|"ceiling" => {
            expect(&args, 1, true)?;
            wrap(args[0].ceil()?)
        }
        "round" => {
            expect_unitless(&args)?;
            if args.len() == 1 {
                wrap(args[0].round()?)
            }
            else if args.len() == 2 {
                wrap(args[0].round_to(args[1].get_quant().round().to_f32() as u32)?)
            }
            else {
                Err(String::from("Wrong number of arguments"))
            }
        }
        "abs"|"absolute"|"absol"|"absolutes" => {
            expect(&args, 1, true)?;
            wrap(args[0].floor()?)
        }

        _ => {Err(String::from("Not a Function"))},
    }
}
