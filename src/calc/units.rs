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
