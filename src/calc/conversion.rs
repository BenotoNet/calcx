use crate::Num;
use crate::calc;

// Function to check if a number can be converted to some arbetrary units
pub fn can_be_converted(base: Num, to_units: &str) -> bool {
    match calc::Calc::new().run(to_units) {
        calc::expr::Expr::Number(num) => {
            // Doing so by checking if when dividing by new units results in unitless number =>
            // Units match and can be converted
            return base.div(&num).unwrap().is_unitless();
        },
        _ => {return false}
    }
}
