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

    pub fn combine<T: Fn(i8, i8) -> i8>(unit1: Units, unit2: Units, operation: T) -> Units {
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

#[derive(Debug)]
pub struct Num {
    quantity: f64,
    units: Units,
}

impl Num {
    pub fn new(quantity: f64, units_vec: Vec<(char, i8)>) -> Num {
        let mut units = Units::new(units_vec);
        Num { quantity, units }
    }

    pub fn from(quantity: f64, units: Units) -> Num {
        Num { quantity, units }
    }

    pub fn add(&self, num2: Num) -> Option<Num> {
        if self.units == num2.units {
            return Some(Num::from(self.quantity+num2.quantity, self.units.clone()))
        }
        None
    }

    pub fn sub(&self, num2: Num) -> Option<Num> {
        if self.units == num2.units {
            return Some(Num::from(self.quantity-num2.quantity, self.units.clone()))
        }
        None
    }

    pub fn mul(&self, num2: Num) -> Option<Num> {
        let output_units = Units::combine(self.units.clone(), num2.units, |unit1, unit2| {unit1+unit2});

        return Some(Num::from(self.quantity*num2.quantity, output_units))
    }

    pub fn div(&self, num2: Num) -> Option<Num> {
        let output_units = Units::combine(self.units.clone(), num2.units, |unit1, unit2| {unit1-unit2});

        return Some(Num::from(self.quantity/num2.quantity, output_units))
    }

    pub fn pow(&self, num2: Num) -> Option<Num> {
        // FIX: ALL Units must be 0, otherwise return None
        // Then, all units from number 1 need to be multiplied by the quanity of num2
        return Some(Num::from(self.quantity.powf(num2.quantity), Units::new(vec![])));
    }
}
