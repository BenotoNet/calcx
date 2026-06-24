use crate::calc::num::Num;
pub fn unit_to_num(ul: &str) -> Option<Num> {
    Some(match ul {
        "exa" => Num::unitless(10.0f64.powf(18.0)),
        "peta" => Num::unitless(10.0f64.powf(15.0)),
        "tera" => Num::unitless(10.0f64.powf(12.0)),
        "giga" => Num::unitless(10.0f64.powf(9.0)),
        "mega" => Num::unitless(10.0f64.powf(6.0)),
        "kilo" => Num::unitless(10.0f64.powf(3.0)),
        "hecto" => Num::unitless(10.0f64.powf(2.0)),
        "deca" => Num::unitless(10.0f64.powf(1.0)),
        "deci" => Num::unitless(10.0f64.powf(-1.0)),
        "centi" => Num::unitless(10.0f64.powf(-2.0)),
        "milli" => Num::unitless(10.0f64.powf(-3.0)),
        "micro" => Num::unitless(10.0f64.powf(-6.0)),
        "nano" => Num::unitless(10.0f64.powf(-9.0)),
        "pico" => Num::unitless(10.0f64.powf(-12.0)),
        _ => return None
    })
}
