use crate::Num;
use std::collections::HashMap;

mod storage;

fn parse_prop_format(file_content: String) -> HashMap<String, Num> {
    let mut map: HashMap<String, Num> = HashMap::new();
    let definitions = file_content.split(['\n', ';']);
    for definition in definitions {
        if definition == "" || definition.starts_with("//") {continue};
        let names = definition.split(": ").nth(0).unwrap();
        let names = names.split(", ").collect::<Vec<&str>>();

        let number_and_units = definition.split(": ").nth(1).unwrap().split(" ").collect::<Vec<&str>>();
        let number = number_and_units.first().unwrap();
        let units = &number_and_units[1..];

        for name in names {
            // Get units:
            let mut working_units: [i8; 6] = [0; 6];

            for unit in units {
                let mut unit = *unit;
                let mut amount = 1;
                if unit.contains("^") {
                    amount = unit.split("^").nth(1).unwrap().parse().unwrap();
                    unit = unit.split("^").nth(0).unwrap();
                }
                if unit == "s" {working_units[0] += amount}
                else if unit == "m" {working_units[1] += amount}
                else if unit == "K" {working_units[2] += amount}
                else if unit == "a" {working_units[3] += amount}
                else if unit == "k" {working_units[4] += amount}
                else if unit == "c" {working_units[5] += amount}
                else {
                    // relative definition of units;
                    let num = map.get(&unit.to_string()).expect(&format!{"Could not get relative UNIT, {}", unit}).get_units().get_ordered_list();
                    for (i, sub_unit) in num.iter().enumerate() {
                        working_units[i] += sub_unit
                    }
                }
            }

            // Build Num
            let num = Num::new(number, vec![('s', working_units[0]), 
                                            ('m', working_units[1]), 
                                            ('K', working_units[2]), 
                                            ('a', working_units[3]), 
                                            ('k', working_units[4]), 
                                            ('c', working_units[5]), 
                                            ]);

            // Actual definition now:
            map.insert(name.to_string(), num);
        }
    }
    map
}

fn map_to_vec(map: HashMap<String, Num>) -> Vec<(String, Num)> {
    let keys: Vec<String> = map.keys().into_iter().map(|v| {v.to_string()}).collect();
    keys.into_iter().filter_map(|key| {
        match map.get(&key) {
            Some(v) => {
                return Some((key, v.clone()))
            }
            _ => None
        }
    }).collect()
}

pub fn get_constants() -> Vec<(String, Num)> {
    map_to_vec(parse_prop_format(storage::constants()))
}
