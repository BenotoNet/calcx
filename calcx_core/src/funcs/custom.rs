// TODO: system for handling custom user-defined functions

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::{Calc, Expr};

impl Calc {
    pub fn is_custom_func(&self, func_str: &str) -> bool {
        true
    }

    pub fn run_custom_func(&self, func_str: &str, args: Vec<Expr>) -> Result<Expr, String> {
        Err(String::from("testing123"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionStorage {
    storage: HashMap<String, String>,
}

impl FunctionStorage {
    pub fn new() -> FunctionStorage {
        FunctionStorage { storage: HashMap::new() }
    }

    pub fn get_func(&self, func: String) -> Result<String, String> {
        match self.storage.get(&func) {
            Some(v) => Ok(v.clone()),
            _ => Err(String::from("Could not get Variable")),
        }
    }

    pub fn set_var(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }
}
