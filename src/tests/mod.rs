#![allow(unused)]
use super::*;
use super::calc::Calc;
use super::calc::expr::Expr;
use super::calc::num::Num;

fn make_default_calc() -> Calc {
    Calc::new(15)
}

fn query(calc: &mut Calc, query: &str, output_number_query: &str) {
    assert_eq!{calc.run(query), calc.run(output_number_query)};
}

#[test]
fn simple_arithmatic() {
}

#[test]
fn creating_calc() {
    make_default_calc();
}

#[test]
fn simple_parsing() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1"), Ok(Expr::Number(Num::unitless("1")))
    };
}

#[test]
fn simple_query() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1+1"), Ok(Expr::Number(Num::unitless("2")))
    };
}

#[test]
fn arithmatic_queries() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1+(4-2)-1+7--2-2++5-0.5+.2"), Ok(Expr::Number(Num::unitless("13.7")))
    };
}

#[test]
fn advanced_arithmatic() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("3+4*2/(1-5)^(2^3)"), Ok(Expr::Number(Num::unitless("3.0001220703125")))
    };
}

#[test]
fn units() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1 meter second ampere kilogram candela kelvin"), Ok(Expr::Number(Num::new("1.0", vec![('m', 1), ('s', 1), ('a', 1), ('K', 1), ('k', 1), ('c', 1)])))
    };
}

#[test]
fn failure() {
    let mut calc = make_default_calc();
    match calc.run("Hello") {
        Err(_) => {},
        _ => panic!{"This query should result in a failure!"}
    }
}

#[test]
fn variable_storage() {
    let mut calc = make_default_calc();
    calc.run("var = 4");
    assert_eq!{
        calc.run("var * 5"), Ok(Expr::Number(Num::unitless("20")))
    };
}

#[test]
fn negative_signs() {
    let mut calc = make_default_calc();

    query(&mut calc, "1-2^2", "(0-1)*(3)");
    query(&mut calc, "-1*-1", "1");
    query(&mut calc, "1-1", "0");
}

#[test]
fn pre_defined_variables() {
    let mut calc = make_default_calc();

    query(&mut calc, "round(pi, 2)", "3.14");
}

#[test]
fn functions() {
    let mut calc = make_default_calc();

    query(&mut calc, "test(add_one(sqrt(2^2)))", "3")
}
