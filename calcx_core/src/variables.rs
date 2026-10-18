use crate::Num;
use crate::Expr;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VariableStorage {
    storage: HashMap<String, Num>,
}

impl VariableStorage {
    pub fn new() -> VariableStorage {
        VariableStorage { storage: HashMap::new() }
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.storage.keys().into_iter().map(|v| {v.to_string()}).collect()
    }

    pub fn get_var(&self, var: String) -> Result<Expr, String> {
        match self.storage.get(&var) {
            Some(v) => Ok(Expr::Number(v.clone())),
            _ => Err(String::from("Could not get Variable")),
        }
    }

    pub fn set_var(&mut self, key: String, value: Num) {
        self.storage.insert(key, value);
    }
}

