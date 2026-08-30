pub fn try_match(second: &f64, metre: &f64, kilogram: &f64, ampere: &f64, kelvin: &f64, candela: &f64) -> Option<String> {
    return Some(String::from(match (second, metre, kilogram, ampere, kelvin, candela) {
        (-2.0, 2.0, 1.0, 0.0, 0.0, 0.0) => {"joule"},
        _ => return None,
    }));
}
