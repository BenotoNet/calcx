use crate::calc::num::Num;
pub fn unit_to_num(ul: &str) -> Option<Num> {
    Some(match ul {

        // Powers of Ten
        "exa" => Num::unitless(1e18),
        "peta" => Num::unitless(1e15),
        "tera" => Num::unitless(1e12),
        "giga" => Num::unitless(1e9),
        "mega" => Num::unitless(1e6),
        "kilo" => Num::unitless(1e3),
        "hecto" => Num::unitless(1e2),
        "deca" => Num::unitless(1e1),
        "deci" => Num::unitless(1e-1),
        "centi" => Num::unitless(1e-2),
        "milli" => Num::unitless(1e-3),
        "micro" => Num::unitless(1e-6),
        "nano" => Num::unitless(1e-9),
        "pico" => Num::unitless(1e-12),

        "ton" => Num::new(1e3, vec![('K', 1)]),

        // Bit units
        "kibi" => Num::unitless(2.0f64.powf(10.0)),
        "mebi" => Num::unitless(2.0f64.powf(20.0)),
        "gibi" => Num::unitless(2.0f64.powf(30.0)),
        "tebi" => Num::unitless(2.0f64.powf(40.0)),
        "pebi" => Num::unitless(2.0f64.powf(50.0)),
        "exbi" => Num::unitless(2.0f64.powf(60.0)),

        // Time Units
        "minute"|"minutes" => {Num::new(60.0, vec![('s', 1)])}
        "hour"|"hours"|"hr"|"hrs" => {Num::new(3600.0, vec![('s', 1)])}
        "day"|"days" => {Num::new(86400.0, vec![('s', 1)])}
        "week"|"weeks" => {Num::new(604800.0, vec![('s', 1)])}
        "year"|"years" => {Num::new(31557600.0, vec![('s', 1)])}

        // Computers
        "byte"|"bytes" => {Num::unitless(8.0)}

        // Physics
        "newton"|"newtons" => {Num::new(1.0, vec![('K', 1), ('m', 1), ('s', -2)])}
        "pascal"|"pascals" => {Num::new(1.0, vec![('K', 1), ('m', -1), ('s', -2)])}
        "joule"|"joules" => {Num::new(1.0, vec![('K', 1), ('m', 2), ('s', -2)])}
        "cal"|"calorie" => {Num::new(4.1868, vec![('K', 1), ('m', 2), ('s', -2)])}
        "kcal" => {Num::new(4.1868e3, vec![('K', 1), ('m', 2), ('s', -2)])}
        "watt"|"watts" => {Num::new(1.0, vec![('K', 1), ('m', 2), ('s', -3)])}
        "coulomb" => {Num::new(1.0, vec![('a', 1), ('s', 1)])}
        "volt"|"volts" => {Num::new(1.0, vec![('K', 1), ('m', 2), ('s', -3), ('a', -1)])}
        "ohm"|"ohms" => {Num::new(1.0, vec![('K', 1), ('m', 2), ('s', -3), ('a', -2)])}
        "siemens" => {Num::new(1.0, vec![('K', -1), ('m', -2), ('s', 3), ('a', 2)])}
        "farad" => {Num::new(1.0, vec![('K', -1), ('m', -2), ('s', 4), ('a', 2)])}
        "weber" => {Num::new(1.0, vec![('K', 1), ('m', 2), ('s', -2), ('a', -1)])}
        "hertz"|"hz" => {Num::new(1.0, vec![('s', -1)])}
        "bar"|"bars" => {Num::new(1e5, vec![('K', 1), ('m', -1), ('s', -2)])}
        "litre"|"liter"|"liters"|"litres" => {Num::new(1e-3, vec![('m', 3)])}
        "atomic_mass_unit"|"u"|"amu" => {Num::new(1.660538921e-27, vec![('K', 1)])}

        "gravity"|"gravity_constant" => {Num::new(9.80665, vec![('m', 1), ('s', -2)])}
        "atm"|"atmosphere"|"atmospheric_pressure" => {Num::new(101325.0, vec![('K', 1), ('m', -1), ('s', -2)])}
        "mach" => {Num::new(331.46, vec![('m', 1), ('s', -1)])}
        "room_temperature" => Num::new(294.15, vec![('k', 1)]),
        "1°C"|"standard_temperature" => Num::new(273.15, vec![('k', 1)]),

        // MISC
        "degree"|"degrees" => {Num::unitless(0.01745329251994329576923690768488612713442871888542)}
        "micron"|"microns" => {Num::new(1e-6, vec![('m', 1)])}
        "circle"|"circles" => {Num::unitless(6.28318530717958647692528676655900576839433879875021)}

        "pi" => {Num::unitless(3.14159265358979323846264338327950288419716939937511)}
        "lightspeed"|"light"|"speedoflight"|"speed_of_light" => {Num::new(2.99792458e8, vec![('m', 1), ('s', -1)])}

        "percent"|"%"|"percents" => {Num::unitless(1e-2)}
        "ppm"|"partspermillion" => {Num::unitless(1e-6)}
        "ppb"|"partsperbillion" => {Num::unitless(1e-9)}
        "ppt"|"partspertrillion" => {Num::unitless(1e-12)}

        "karat" => {Num::unitless(1.0/24.0)}

        // TODO: Continue on https://github.com/darius/unitcalc/blob/master/definitions.units at
        // line 976

        _ => return None
    })
}
