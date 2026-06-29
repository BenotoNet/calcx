use super::num::Num;
use super::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(Num),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    }
}

impl Expr {
    pub fn display(&self, precision: usize) -> String {
        match self {
            Expr::Number(num) => {format!{"{}", num.display(precision)}},
            _ => {panic!("Cannot display an operation, should only ever display Number!")}
        }
    }
}
