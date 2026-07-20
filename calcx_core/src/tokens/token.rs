use crate::Num;

#[derive(Debug, Clone, PartialEq)]
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
