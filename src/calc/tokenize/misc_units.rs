use crate::calc::num::Num;
pub fn unit_to_num(ul: &str) -> Option<Num> {
    Some(match ul {

        // Powers of Ten
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

        // Time Units
        "minute"|"minutes" => {Num::new(60.0, vec![('s', 1)])}
        "hour"|"hours" => {Num::new(3600.0, vec![('s', 1)])}
        "day"|"days" => {Num::new(86400.0, vec![('s', 1)])}
        "week"|"weeks" => {Num::new(604800.0, vec![('s', 1)])}
        "year"|"years" => {Num::new(31557600.0, vec![('s', 1)])}

        // Computers
        "byte"|"bytes" => {Num::unitless(8.0)}

        // Physics
        "joule"|"joules" => {Num::new(1000.0, vec![('g', 1), ('m', 2), ('s', -2)])}
        "watt"|"watts" => {Num::new(1000.0, vec![('g', 1), ('m', 2), ('s', -3)])}

        _ => return None
    })
}
