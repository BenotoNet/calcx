// TODO: system for handling custom user-defined functions

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::{Calc, Expr, Token};

impl Calc {
    pub fn is_custom_func(&self, func_str: &str) -> bool {
        match self.userfunctions.get_func(func_str) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn run_custom_func(&self, func_str: &str, args: Vec<Expr>) -> Result<Expr, String> {
        println!{"{args:?}"}
        let how_to_calculate_func = self.userfunctions.get_func(func_str);

        // We want to replace Binary operators as Variables for function with actual numbers given
        println!{"{how_to_calculate_func:?}"};
        todo!{}
        // FIX: Here, I need to turn Binary(left: none, op: Var("00"), right: binary...) into
        // binary(left: [value of var00], op: [operation next], right: ...)

        Err(String::from("testing123"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionStorage {
    storage: HashMap<String, Result<Expr, String>>,
}

impl FunctionStorage {
    pub fn new() -> FunctionStorage {
        FunctionStorage { storage: HashMap::new() }
    }

    pub fn get_func(&self, func: &str) -> Result<Expr, String> {
        match self.storage.get(func) {
            Some(v) => v.clone(),
            _ => Err(String::from("Could not get Function")),
        }
    }

    pub fn set_func(&mut self, key: String, value: Result<Expr, String>) {
        self.storage.insert(key, value);
    }
}
