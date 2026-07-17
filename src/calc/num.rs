use crate::calc::units::Units;
use crate::{Float, utils};

#[derive(Debug, Clone)]
pub struct Num {
    quantity: Float,
    units: Units,
}

#[allow(unused)]
impl Num {
    pub fn new(quantity: &str, units_vec: Vec<(char, i8)>) -> Num {
        let quantity = Float::parse(quantity).unwrap();
        let quantity = Float::with_val(crate::PRECISION, quantity);
        Num { quantity, units: Units::new(units_vec) }
    }

    pub fn new_float(quantity: Float, units_vec: Vec<(char, i8)>) -> Num {
        Num { quantity, units: Units::new(units_vec) }
    }

    pub fn unitless(quantity: &str) -> Num {
        Num::new(quantity, vec![])
    }

    pub fn unitless_float(quantity: Float) -> Num {
        Num::new_float(quantity, vec![])
    }

    pub fn is_unitless(&self) -> bool {
        return self.units.is_unitless();
    }

    pub fn get_quant(&self) -> Float {
        return self.quantity.clone();
    }

    pub fn get_units(&self) -> &Units {
        return &self.units;
    }

    pub fn append(&self, num: &Num) -> Num {
        // Numbers are expected to be unitless, therefore just append the quantities
        assert!{self.is_unitless()};
        assert!{num.is_unitless()};

        let mut power_of_ten = num.get_quant().clone().log10().ceil();

        // Edge case of number 1
        if num.get_quant() == 1.0 || num.get_quant() == 0 {
            power_of_ten = Float::with_val(crate::PRECISION, 1);
        }
        else if num.get_quant().clone().log10() == power_of_ten {
            power_of_ten += 1;
        }

        let combined_value = num.get_quant() + self.get_quant() * (Float::with_val(crate::PRECISION, 10.0).ln() * power_of_ten).exp();

        return Num::unitless_float(combined_value);
    }

    pub fn from(quantity: Float, units: Units) -> Num {
        Num { quantity, units }
    }

    pub fn add(&self, num2: &Num) -> Option<Num> {
        if self.units == num2.units {
            return Some(Num::from(self.quantity.clone()+num2.quantity.clone(), self.units.clone()))
        }
        None
    }

    pub fn sub(&self, num2: &Num) -> Option<Num> {
        if self.units == num2.units {
            return Some(Num::from(self.quantity.clone()-num2.quantity.clone(), self.units.clone()))
        }
        None
    }

    pub fn mul(&self, num2: &Num) -> Option<Num> {
        let output_units = Units::combine(&self.units.clone(), &num2.units, |unit1, unit2| {unit1+unit2});

        return Some(Num::from(self.quantity.clone()*num2.quantity.clone(), output_units))
    }

    pub fn div(&self, num2: &Num) -> Option<Num> {
        let output_units = Units::combine(&self.units.clone(), &num2.units, |unit1, unit2| {unit1-unit2});

        return Some(Num::from(self.quantity.clone() / num2.quantity.clone(), output_units))
    }

    pub fn modf(&self, num2: &Num) -> Option<Num> {
        // Numbers are expected to be unitless
        assert!(self.is_unitless() && num2.is_unitless());

        return Some(Num::unitless_float(self.quantity.clone() % num2.quantity.clone()));
    }

    pub fn powf(&self, num2: &Num) -> Option<Num> {
        // exponent is expected to be unitless / dimensionless
        assert!(num2.is_unitless());

        // Taking a negative Number to the power of something is impossible (if exponent is *not* an
        // integer, so we have different cases: )


        let output_units = Units::operation(self.units.clone(), |unit| {unit * num2.get_quant().to_f64()});
        println!{"{output_units:?}"}
        match self.get_quant() < 0 {
            true => {
                // we have a negative number, therefore we need to see if exponent is an integer
                match utils::is_int(num2) {
                    true => {
                        // We can calculate that!
                        // Remove negative Sign
                        let positive = Num::unitless_float(self.get_quant()).abs().unwrap();
                        let output_real = positive.powf(num2).unwrap();
                        let pos_or_neg = num2.modf(&Num::unitless("2.0")).unwrap();
                        // Uneven, therefore we still have a Negative sign
                        if utils::eq(&pos_or_neg, &Num::unitless("1.0")) {
                            return Some(Num::from(output_real.mul(&Num::unitless("-1")).unwrap().get_quant(), output_units));
                        }
                        else {
                            return Some(Num::from(output_real.get_quant(), output_units));
                        }
                    },
                    false => return None,
                }
            }
            false => {
                // Normal calculation
                // x^y = e^(y ln x)
                let output_quantity = (self.quantity.clone().ln() * num2.quantity.clone()).exp();
                // TODO: Problem: Since we do not have units as floats, this is a sacrifice
                return Some(Num::from(output_quantity, output_units));
            }
        }
        

    }

    pub fn sin(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().sin())),
            false => None,
        }
    }

    pub fn cos(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().cos())),
            false => None,
        }
    }

    pub fn tan(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().tan())),
            false => None,
        }
    }

    pub fn arcsin(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().asin())),
            false => None,
        }
    }

    pub fn arccos(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().acos())),
            false => None,
        }
    }

    pub fn arctan(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().atan())),
            false => None,
        }
    }

    pub fn abs(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().abs())),
            false => None,
        }
    }

    pub fn round(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().round())),
            false => None,
        }
    }

    pub fn ceil(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().ceil())),
            false => None,
        }
    }

    pub fn floor(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().floor())),
            false => None,
        }
    }

    pub fn log(&self, base: &Num) -> Option<Num> {
        if !self.is_unitless() || !base.is_unitless() {return None};
        Some(Num::unitless_float(self.quantity.clone().log2()).div(&Num::unitless_float(base.get_quant().log2())).unwrap())
    }

    pub fn exp(&self) -> Option<Num> {
        if !self.is_unitless() {return None};
        Some(Num::unitless_float(self.quantity.clone().exp()))
    }

    pub fn display(&self, precision: usize) -> String {
        format!("{} {}", utils::format_float(&self.quantity, precision), self.units.display())
    }
}
