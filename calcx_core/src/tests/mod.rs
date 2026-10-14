#![allow(unused)]

use crate::{Calc, Num, Expr};

fn make_default_calc() -> Calc {
    Calc::new(15)
}

fn query(calc: &mut Calc, query: &str, output_number_query: &str) {
    assert_eq!{calc.run_output(query, true), calc.run_output(output_number_query, false)};
}

#[test]
fn simple_arithmatic() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("5+5*2", true), Ok(Expr::Number(Num::unitless("15")))
    };
}

#[test]
fn creating_calc() {
    make_default_calc();
}

#[test]
fn simple_parsing() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1", true), Ok(Expr::Number(Num::unitless("1")))
    };
}

#[test]
fn simple_query() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1+1", true), Ok(Expr::Number(Num::unitless("2")))
    };
}

#[test]
fn arithmatic_queries() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1+(4-2)-1+7--2-2++5-0.5+.2", true), Ok(Expr::Number(Num::unitless("13.7")))
    };
}

#[test]
fn advanced_arithmatic() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("3+4*2/(1-5)^(2^3)", true), Ok(Expr::Number(Num::unitless("3.0001220703125")))
    };
}

#[test]
fn units() {
    let mut calc = make_default_calc();
    assert_eq!{
        calc.run("1 meter second ampere kilogram candela kelvin", true), Ok(Expr::Number(Num::new("1.0", vec![('m', 1), ('s', 1), ('a', 1), ('K', 1), ('k', 1), ('c', 1)])))
    };
}

#[test]
fn failure() {
    let mut calc = make_default_calc();
    match calc.run("Hello", true) {
        Err(_) => {},
        _ => panic!{"This query should result in a failure!"}
    }
}

#[test]
fn variable_storage() {
    let mut calc = make_default_calc();
    calc.run("var = 4", true);
    assert_eq!{
        calc.run("var * 5", true), Ok(Expr::Number(Num::unitless("20")))
    };
}

#[test]
fn negative_signs() {
    let mut calc = make_default_calc();

    query(&mut calc, "1-2^2", "(0-1)*(3)");
    query(&mut calc, "-1*-1", "1");
    query(&mut calc, "1-1", "0");
    query(&mut calc, "2^-2", "0.25");
    query(&mut calc, "2^-2 meter", "0.25 meter");
    query(&mut calc, "2^-2 - 2", "-1.75");
    query(&mut calc, "-2^2", "-4");
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


#[test]
fn last_answer() {
    let mut calc = make_default_calc();

    calc.run("2", true);
    query(&mut calc, "ans", "2");
    query(&mut calc, "2 ans", "4");
    query(&mut calc, "2 * ans", "8");
    query(&mut calc, "ans 2", "16");
    query(&mut calc, "ans * 2", "32");
    query(&mut calc, "ans - 1", "31");
    query(&mut calc, "nth_last 2", "32");
    query(&mut calc, "nth_last 2 * 2", "62");
    query(&mut calc, "2 * nth_last 2", "64");
    query(&mut calc, "2 nth_last 2", "124");
    calc.run("4", true);
    query(&mut calc, "sqrt(ans)", "2");
}

#[test]
fn precedence() {
    let mut calc = make_default_calc();

    query(&mut calc, "2+3*4", "14");
    query(&mut calc, "(2+3)*4", "20");
    query(&mut calc, "2*3+4", "10");
    query(&mut calc, "8/4/2", "1");
    query(&mut calc, "8/(4/2)", "4");
    query(&mut calc, "2^3^2", "512");
    query(&mut calc, "(2^3)^2", "64");
    query(&mut calc, "2 * 3 ^ 2", "18");
    query(&mut calc, "-2^2", "-4");
    query(&mut calc, "1 / 2 meter", "0.5 meter^-1");
    query(&mut calc, "2 (3 + 4) meter", "14 meter");
    query(&mut calc, "10^-7 meter", "(10^(-7)) * meter");
    query(&mut calc, "2^log2(8)", "8");
}

#[test]
fn user_defined_functions() {
    let mut calc = make_default_calc();

    query(&mut calc, "custom_add_one(x) = x+1", "1");
    query(&mut calc, "custom_add_one(0)", "1");

    // Multiple Variables
    query(&mut calc, "custom_add(x,y) = x+y", "1");
    query(&mut calc, "custom_add(2,7)", "9");

    // No Variable
    query(&mut calc, "custom_add_no_var(x) = 6+1", "1");
    query(&mut calc, "custom_add_no_var(0)", "7");
    query(&mut calc, "custom_add_no_var(500)", "7");

    // Edge cases
    // This could fail, because variables should not be numbers, only letters? 
    // FIX: DECIDE FOR ONE OF THE OPTIONS
    query(&mut calc, "custom_add_fail(3) = 3+1", "1");
    query(&mut calc, "custom_add_fail(0)", "4");
}
