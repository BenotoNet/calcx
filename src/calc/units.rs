#[derive(Debug, PartialEq, Clone)]
pub struct Units {
    second: f64,
    metre: f64,
    kilogram: f64,
    ampere: f64,
    kelvin: f64,
    candela: f64,
}

impl Units {
    pub fn new(units_construct: Vec<(char, i8)>) -> Units {
        let mut units = Units {
            second: 0.0, metre: 0.0, kilogram: 0.0, ampere: 0.0, kelvin: 0.0, candela: 0.0,
        };
        for (c, amount) in units_construct {
            match c {
                's' => {units.second   = amount as f64}
                'm' => {units.metre    = amount as f64}
                'K' => {units.kilogram = amount as f64}
                'a' => {units.ampere   = amount as f64}
                'k' => {units.kelvin   = amount as f64}
                'c' => {units.candela  = amount as f64}
                _ => {}
            }
        }
        return units;
    }

    pub fn new_frac(units_construct: Vec<(char, f64)>) -> Units {
        let mut units = Units {
            second: 0.0, metre: 0.0, kilogram: 0.0, ampere: 0.0, kelvin: 0.0, candela: 0.0,
        };
        for (c, amount) in units_construct {
            match c {
                's' => {units.second   = amount}
                'm' => {units.metre    = amount}
                'K' => {units.kilogram = amount}
                'a' => {units.ampere   = amount}
                'k' => {units.kelvin   = amount}
                'c' => {units.candela  = amount}
                _ => {}
            }
        }
        return units;
    }

    // Check if number is unitless
    pub fn is_unitless(&self) -> bool {
        return self.second == 0.0 && self.metre == 0.0 && self.kilogram == 0.0 && self.ampere == 0.0 && self.kelvin == 0.0 && self.candela == 0.0;
    }

    pub fn operation<T: Fn(f64) -> f64>(mut unit1: Units, operation: T) -> Units {
        unit1.second = operation(unit1.second);
        unit1.metre = operation(unit1.metre);
        unit1.kilogram = operation(unit1.kilogram);
        unit1.ampere = operation(unit1.ampere);
        unit1.kelvin = operation(unit1.kelvin);
        unit1.candela = operation(unit1.candela);
        return unit1;
    }

    // When we have two Unit 
    pub fn combine<T: Fn(f64, f64) -> f64>(unit1: &Units, unit2: &Units, operation: T) -> Units {
        let mut output_units = Units::new(vec![]);
        output_units.second = operation(unit1.second, unit2.second);
        output_units.metre = operation(unit1.metre, unit2.metre);
        output_units.kilogram = operation(unit1.kilogram, unit2.kilogram);
        output_units.ampere = operation(unit1.ampere, unit2.ampere);
        output_units.kelvin = operation(unit1.kelvin, unit2.kelvin);
        output_units.candela = operation(unit1.candela, unit2.candela);

        return output_units;
    }

    pub fn display(&self) -> String {
        // TODO: Later this needs to adapt to other units (like newton => 1 kilo gram * meter /
        // second^-2)
        let mut output = String::new();
        if self.second != 0.0 {output += &format!{"{}^{} ", "second", self.second}};
        if self.metre != 0.0 {output += &format!{"{}^{} ", "meter", self.metre}};
        if self.kilogram != 0.0 {output += &format!{"{}^{} ", "kilogram", self.kilogram}};
        if self.ampere != 0.0 {output += &format!{"{}^{} ", "ampere", self.ampere}};
        if self.kelvin != 0.0 {output += &format!{"{}^{} ", "kelvin", self.kelvin}};
        if self.candela != 0.0 {output += &format!{"{}^{} ", "candela", self.candela}};
        return output;
    }
}
