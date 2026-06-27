use crate::Num;
use crate::calc;

// fn can_be_converted_to(base: &Num, to_units: &str) -> bool {
//     can_be_converted(base, &calc::Calc::new().run(to_units))
// }

// Function to check if a number can be converted to some arbetrary units
fn can_be_converted(base: &Num, units: &calc::expr::Expr) -> bool {
    // TODO: Simplify expression via calc function (instead of calc.run(string))
    // let calc = calc::Calc::new();
    // let units = calc.eval(*units);
    match units {
        calc::expr::Expr::Number(num) => {
            // Doing so by checking if when dividing by new units results in unitless number =>
            // Units match and can be converted
            return base.div(&num).unwrap().is_unitless();
        },
        _ => {return false}
    }
}

// pub fn convert_to(base: &Num, to_units: &str) -> Option<String> {
//     let mut calc = calc::Calc::new();
//     let expr = calc.run(to_units);
//     match can_be_converted(base, &expr) {
//         true => {
//             return convert(base, expr);
//         }
//         false => None,
//     }
// }

pub fn convert(base: &Num, units: calc::expr::Expr) -> Option<Num> {
    // Convert by dividing quantity by parsed quantity of units -> This is the quantity of the
    // output num
    match units {
        calc::expr::Expr::Number(num) => {
            let output = base.div(&num).unwrap();
            match output.is_unitless() {
                true => Some(output),
                false => None
            }
        }
        _ => None
    }
}
