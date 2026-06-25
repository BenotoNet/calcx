use super::num::Num;
use super::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(Num),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Keyword(String),
}

impl Expr {
    pub fn display(&self) -> String {
        match self {
            Expr::Number(num) => {format!{"{}", num.display()}},
            _ => {panic!("Cannot display an operation, should only ever display display")}
        }
    }
}
