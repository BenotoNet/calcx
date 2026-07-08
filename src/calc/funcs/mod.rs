use crate::calc::expr::Expr;


pub fn is_function(token_str: &str) -> bool {
    true
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
    // FIX: We need to evaluate each argument, so that we have only one number left, not an
    // expression of tokens
    match func_str {
        "add_one" => {
            // FIX: Implement a function to add one to the first argument
        },
        _ => {},
    }
    Err(String::new())
}
