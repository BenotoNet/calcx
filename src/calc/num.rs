use crate::calc::units::Units;

#[derive(Debug, Clone)]
pub struct Num {
    quantity: f64,
    units: Units,
}

#[allow(unused)]
impl Num {
    pub fn new(quantity: f64, units_vec: Vec<(char, i8)>) -> Num {
        Num { quantity, units: Units::new(units_vec) }
    }

    pub fn unitless(quantity: f64) -> Num {
        Num::new(quantity, vec![])
    }

    pub fn is_unitless(&self) -> bool {
        return self.units.is_unitless();
    }

    pub fn get_quant(&self) -> f64 {
        return self.quantity;
    }

    pub fn get_units(&self) -> &Units {
        return &self.units;
    }

    pub fn append(&self, num: &Num) -> Num {
        // Numbers are expected to be unitless, therefore just append the quantities
        assert!{self.is_unitless()};
        assert!{num.is_unitless()};

        return Num::unitless(format!{"{}{}", self.quantity, num.quantity}.parse::<f64>().unwrap())
    }

    pub fn from(quantity: f64, units: Units) -> Num {
        Num { quantity, units }
    }

    pub fn add(&self, num2: &Num) -> Option<Num> {
        if self.units == num2.units {
            return Some(Num::from(self.quantity+num2.quantity, self.units.clone()))
        }
        None
    }

    pub fn sub(&self, num2: &Num) -> Option<Num> {
        if self.units == num2.units {
            return Some(Num::from(self.quantity-num2.quantity, self.units.clone()))
        }
        None
    }

    pub fn mul(&self, num2: &Num) -> Option<Num> {
        let output_units = Units::combine(&self.units.clone(), &num2.units, |unit1, unit2| {unit1+unit2});

        return Some(Num::from(self.quantity*num2.quantity, output_units))
    }

    pub fn div(&self, num2: &Num) -> Option<Num> {
        let output_units = Units::combine(&self.units.clone(), &num2.units, |unit1, unit2| {unit1-unit2});

        return Some(Num::from(self.quantity/num2.quantity, output_units))
    }

    pub fn modf(&self, num2: &Num) -> Option<Num> {
        // Numbers are expected to be unitless
        assert!(self.is_unitless() && num2.is_unitless());

        return Some(Num::unitless(self.quantity % num2.quantity));
    }

    pub fn powf(&self, num2: &Num) -> Option<Num> {
        // exponent is expected to be unitless / dimensionless
        assert!(num2.is_unitless());
        
        let output_quantity = self.quantity.powf(num2.quantity);
        // TODO: Problem: Since we do not have units as floats, this is a sacrifice
        let output_units = Units::operation(self.units.clone(), |unit| {unit * num2.quantity as i8});

        return Some(Num::from(output_quantity, output_units));
    }

    pub fn sin(&self) -> Option<Num> {
        match self.is_unitless() {
            true => Some(Num::unitless(self.quantity.sin())),
            false => None,
        }
    }

    pub fn display(&self) -> String {
        format!("{} {}", self.quantity, self.units.display())
    }
}
