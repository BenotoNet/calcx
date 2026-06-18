#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Units {
    second: i8,
    metre: i8,
    gram: i8,
    ampere: i8,
    kelvin: i8,
    candela: i8,
}

impl Units {
    pub fn new(units_construct: Vec<(char, i8)>) -> Units {
        let mut units = Units {
            second: 0, metre: 0, gram: 0, ampere: 0, kelvin: 0, candela: 0,
        };
        for (c, amount) in units_construct {
            match c {
                's' => {units.second  = amount}
                'm' => {units.metre   = amount}
                'g' => {units.gram    = amount}
                'a' => {units.ampere  = amount}
                'k' => {units.kelvin  = amount}
                'c' => {units.candela = amount}
                _ => {}
            }
        }
        return units;
    }

    // Check if number is unitless
    pub fn is_unitless(&self) -> bool {
        return self.second == 0 && self.metre == 0 && self.gram == 0 && self.ampere == 0 && self.kelvin == 0 && self.candela == 0;
    }

    pub fn operation<T: Fn(i8) -> i8>(mut unit1: Units, operation: T) -> Units {
        unit1.second = operation(unit1.second);
        unit1.metre = operation(unit1.metre);
        unit1.gram = operation(unit1.gram);
        unit1.ampere = operation(unit1.ampere);
        unit1.kelvin = operation(unit1.kelvin);
        unit1.candela = operation(unit1.candela);
        return unit1;
    }

    // When we have two Unit 
    pub fn combine<T: Fn(i8, i8) -> i8>(unit1: &Units, unit2: &Units, operation: T) -> Units {
        let mut output_units = Units::new(vec![]);
        output_units.second = operation(unit1.second, unit2.second);
        output_units.metre = operation(unit1.metre, unit2.metre);
        output_units.gram = operation(unit1.gram, unit2.gram);
        output_units.ampere = operation(unit1.ampere, unit2.ampere);
        output_units.kelvin = operation(unit1.kelvin, unit2.kelvin);
        output_units.candela = operation(unit1.candela, unit2.candela);

        return output_units;
    }
}

#[derive(Debug, Clone)]
pub struct Num {
    quantity: f64,
    units: Units,
}

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

    pub fn display(&self) -> String {
        format!("{} {:?}", self.quantity, self.units)
    }
}
