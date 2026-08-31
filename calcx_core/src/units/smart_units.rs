pub fn try_match(second: &f64, metre: &f64, kilogram: &f64, ampere: &f64, kelvin: &f64, candela: &f64) -> Option<String> {
    return Some(String::from(match (second, metre, kilogram, ampere, kelvin, candela) {
        // (Second, Meter, Kilogram, Ampere, Kelvin, Candela)
        (-2.0, 2.0, 1.0, 0.0, 0.0, 0.0)    => {"joule"},
        (-2.0, 1.0, 1.0, 0.0, 0.0, 0.0)    => {"newton"},
        (-2.0, -1.0, 1.0, 0.0, 0.0, 0.0)   => {"pascal"},
        (-3.0, 2.0, 1.0, 0.0, 0.0, 0.0)    => {"watt"},
        (1.0, 0.0, 0.0, 1.0, 0.0, 0.0)     => {"coulomb"},
        (-3.0, 2.0, 1.0, -1.0, 0.0, 0.0)   => {"volt"},
        (-3.0, 2.0, 1.0, -2.0, 0.0, 0.0)   => {"ohm"},
        (3.0, -2.0, -1.0, 2.0, 0.0, 0.0)   => {"siemens"},
        (-2.0, 2.0, 0.0, 0.0, 0.0, 0.0)    => {"sievert"},
        (4.0, -2.0, -1.0, 2.0, 0.0, 0.0)   => {"farad"},
        (-2.0, 2.0, 1.0, -1.0, 0.0, 0.0)   => {"weber"},
        (-1.0, 0.0, 0.0, 0.0, 0.0, 0.0)    => {"hertz"},
        (0.0, -2.0, 0.0, 0.0, 0.0, 1.0)    => {"lux"},

        _ => return None,
    }));
}
