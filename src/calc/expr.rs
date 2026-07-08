use super::num::Num;
use super::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(Num),
    Binary {
        left: Box<Option<Expr>>,
        op: Token,
        right: Box<Option<Expr>>,
    },
    Arg {
        arg: Box<Option<Expr>>,
        right: Box<Option<Expr>>,
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
