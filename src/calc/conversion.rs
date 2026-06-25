use crate::Num;
use crate::calc;

// Function to check if a number can be converted to some arbetrary units
fn can_be_converted(base: &Num, to_units: &str) -> bool {
    match calc::Calc::new().run(to_units) {
        calc::expr::Expr::Number(num) => {
            // Doing so by checking if when dividing by new units results in unitless number =>
            // Units match and can be converted
            return base.div(&num).unwrap().is_unitless();
        },
        _ => {return false}
    }
}

pub fn convert(base: &Num, to_units: &str) -> Option<String> {
    match can_be_converted(base, to_units) {
        true => {
            // Convert by dividing quantity by parsed quantity of units and then append unit string
            let mut calc = calc::Calc::new();
            let expr = calc.run(to_units);
            match expr {
                calc::expr::Expr::Number(num) => {
                    let output_quant = base.div(&num).unwrap().get_quant();
                    return Some(format!{"{output_quant} {to_units}"});
                }
                _ => None
            }
        }
        false => None,
    }
}
