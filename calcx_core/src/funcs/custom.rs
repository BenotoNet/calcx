// TODO: system for handling custom user-defined functions

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::{Calc, Expr, Token};

fn replace_variable(calc_recur: Result<Expr, String>, from_var: Token, to: Expr) -> Box<Result<Expr, String>> {
    match calc_recur {
        // Unpacking Binary Operators
        Ok(Expr::Binary { left, op, right }) => {
            if op == from_var {
                // This is a variable we want to replace
                return Box::from(Ok(to));
            }
            else {
                // Unpack left & Right
                return Box::from(Ok(Expr::Binary{left: replace_variable(*left, from_var.clone(), to.clone()), op, right: replace_variable(*right, from_var, to) }))
            }
        }
        // Unpacking Args
        Ok(Expr::Arg { arg, right }) => {
            return Box::from(Ok(Expr::Arg { arg: replace_variable(*arg, from_var.clone(), to.clone()), right: replace_variable(*right, from_var, to) }))
        }

        // Otherwise just return whatever we just got, because we can't unpack it
        _ => {
            return Box::from(calc_recur);
        }
    }
}

impl Calc {
    pub fn is_custom_func(&self, func_str: &str) -> bool {
        match self.userfunctions.get_func(func_str) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn run_custom_func(&self, func_str: &str, args: Vec<Expr>) -> Result<Expr, String> {
        // println!{"{args:?}"}
        let mut calculation_recursive: Result<Expr, String> = self.userfunctions.get_func(func_str);

        // First test if we have the right number of arguments
        // TODO: Test if the number of arguments is correct

        // We want to replace Binary operator Tokens (e.g. Var("00")) with the actual numbers given
        // for these variable places (e.g. Var("00") represents the first variable)

        // Here, I turn Binary(left: none, op: Var("00"), right: binary...) into
        // binary(left: [value of var00], op: [operation next], right: ...)
        for (i,v) in args.iter().enumerate() {
            calculation_recursive = *replace_variable(calculation_recursive, Token::Var(format!{"0{i}"}), v.clone());
        }
        
        // final output is the eval of the final token string
        self.eval(calculation_recursive)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionStorage {
    storage: HashMap<String, Expr>,
}

impl FunctionStorage {
    pub fn new() -> FunctionStorage {
        FunctionStorage { storage: HashMap::new() }
    }

    pub fn get_func(&self, func: &str) -> Result<Expr, String> {
        match self.storage.get(func) {
            Some(v) => Ok(v.clone()),
            _ => Err(String::from("Could not get Function")),
        }
    }

    pub fn set_func(&mut self, key: String, value: Result<Expr, String>) -> Result<(), String> {
        self.storage.insert(key, value?);
        Ok(())
    }
}
