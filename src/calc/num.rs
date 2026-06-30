use crate::calc::units::Units;
use crate::{Float, utils};

#[derive(Debug, Clone)]
pub struct Num {
    quantity: Float,
    units: Units,
}

#[allow(unused)]
impl Num {
    pub fn new(quantity: f64, units_vec: Vec<(char, i8)>) -> Num {
        let quantity = utils::f64_to_float(quantity);
        Num { quantity, units: Units::new(units_vec) }
    }

    pub fn new_float(quantity: Float, units_vec: Vec<(char, i8)>) -> Num {
        Num { quantity, units: Units::new(units_vec) }
    }

    pub fn unitless(quantity: f64) -> Num {
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
        
        // x^y = e^(y ln x)
        let output_quantity = (self.quantity.clone().ln() * num2.quantity.clone()).exp();
        // TODO: Problem: Since we do not have units as floats, this is a sacrifice
        let output_units = Units::operation(self.units.clone(), |unit| {unit * num2.quantity.to_f64() as i8});

        return Some(Num::from(output_quantity, output_units));
    }

    pub fn sin(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless_float(self.quantity.clone().sin())),
            false => None,
        }
    }

    pub fn display(&self, precision: usize) -> String {
        format!("{} {}", utils::format_float(&self.quantity, precision), self.units.display())
    }
}
