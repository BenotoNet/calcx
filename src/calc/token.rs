use super::num::Num;

#[derive(Debug, Clone)]
#[allow(unused)]
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
    Septerator,
    Unknown(String),
}
