use crate::Num;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Number(Num),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    
    LBrac,
    RBrac,

    Assign,

    Var(String),

    Keyword(String),
    Func(String),
    Seperator,
    Unknown(String),
}
