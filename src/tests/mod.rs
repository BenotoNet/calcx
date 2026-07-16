#![allow(unused)]
use super::*;
use super::calc::Calc;
use super::calc::expr::Expr;
use super::calc::num::Num;
use super::utils::eq;

fn make_default_calc() -> Calc {
    Calc::new(15)
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

#[test]
fn units() {
    assert! {
        query_eq("1 meter second ampere kilogram candela kelvin", Num::new("1.0", vec![('m', 1), ('s', 1), ('a', 1), ('K', 1), ('k', 1), ('c', 1)]))
    }
}

#[test]
fn num_power() {
    assert! {
        eq(Num::unitless("-2").powf(), Num::unit)
    }
}
