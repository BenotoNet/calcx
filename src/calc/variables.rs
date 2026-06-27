use crate::calc::num::Num;
use super::expr::Expr;
use std::collections::HashMap;

pub struct VariableStorage {
    storage: HashMap<String, Num>,
}

impl VariableStorage {
    pub fn new() -> VariableStorage {
        VariableStorage { storage: HashMap::new() }
    }

    pub fn get_var(&self, var: String) -> Option<Expr> {
        match self.storage.get(&var) {
            Some(v) => Some(Expr::Number(v.clone())),
            _ => None,
        }
    }

    pub fn set_var(&mut self, key: String, value: Num) {
        self.storage.insert(key, value);
    }
}

