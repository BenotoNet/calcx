#![allow(unused)]
use super::*;
use super::calc::Calc;
use super::calc::expr::Expr;
use super::calc::num::Num;

fn make_default_calc() -> Calc {
    Calc::new(15)
}

fn eq(num1: &Num, num2: &Num) -> bool {
    num1.get_quant() == num2.get_quant() && num1.get_units() == num2.get_units()
}

fn query_eq(query: &str, num: Num) -> bool {
    let mut calc = make_default_calc();
    match calc.run(query) {
        Ok(Expr::Number(out_num)) => eq(&out_num, &num),
        _ => false
    }
}

#[test]
fn simple_arithmatic() {
}

#[test]
fn creating_calc() {
    make_default_calc();
}

#[test]
fn simple_query() {
    assert!{
        query_eq("1+1", Num::unitless("2"))
    };
}

#[test]
fn arithmatic_queries() {
    assert! {
        query_eq("1+(4-2)-1+7--2-2++5-0.5+.2", Num::unitless("13.7"))
    }
}
