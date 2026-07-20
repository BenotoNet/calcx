use super::num::Num;
use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(Num),
    Binary {
        left: Box<Result<Expr, String>>,
        op: Token,
        right: Box<Result<Expr, String>>,
    },
    Arg {
        arg: Box<Result<Expr, String>>,
        right: Box<Result<Expr, String>>,
    },
}

impl Expr {
    pub fn display(&self, precision: usize) -> String {
        match self {
            Expr::Number(num) => {format!{"{}", num.display(precision)}},
            _ => {panic!("Cannot display an operation, should only ever display Number!")}
        }
    }
}
