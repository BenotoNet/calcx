pub enum Settings {
    Precision(usize),
}

use rug::Float;
use crate::Num;

pub fn is_number<T: AsRef<str>>(test_string: T) -> bool {
    match test_string.as_ref().parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_int(num: &Num) -> bool {
    match (num.round_to(1000), num.round()) {
        (Ok(n), Ok(m)) => {n == m}
        _ => {false}
    }
}

// Rewriting AI code from scratch:
// What this block is supposed to do: 
// Input: Decimal Float with ~1000 decimal digits
// Optimal Output: 
// Either: Decimal with x places (variable max_decimals)
// Or: Scientific Notation for very big or very small numbers (e.g. 5.12 * 10^-21)

pub fn format_float(num: &Float, max_decimals: usize) -> String {
    let remove_zeros = |num: String| -> String {
        let mut num = num.clone();
        if !num.contains(".") {return num}
        while num.ends_with("0") {
            num.pop();
        }
        if num.ends_with(".") {
            num.pop();
        }

        num
    };

    const MAX_EXPONENT: usize = 6;

    let mut num_str = format!{"{:.*}", max_decimals, num};

    // Check scientific
    if num_str.contains(['e', 'E']) {
        // Scientific Notation -> make not scientific
        let (base, exp) = num_str.split_once(['e', 'E']).unwrap();
        let exp = exp.parse::<i32>().unwrap();
        let base = remove_zeros(base.to_string());

        // if exp is too big or small, we just return scientific
        if exp < 0-(MAX_EXPONENT as i32) || exp > MAX_EXPONENT as i32 {
            return format!{"{base} * 10^{exp}"}
        }

        let negative = base.starts_with("-"); let base = base.trim_start_matches("-");

        let decimal_pos = base.find(".").unwrap_or(base.len()) as i32;
        let new_decimal = decimal_pos+exp;
        let decimals = base.replace(".", "");


        let result = if new_decimal <= 0 {
            // Smaller than 0, therefore just add zeros
            format!{"0.{}{decimals}", "0".repeat(new_decimal.abs() as usize)}

        } else if new_decimal > decimals.len() as i32 {
            // Larger than we have digits (e.g. 500 (digits is just 5))
            format!{"{decimals}{}", "0".repeat(new_decimal.abs() as usize - decimals.len())}

        } else {
            // Between, we need to add a . on our own
            let mut decimals = decimals.to_string();
            decimals.insert(new_decimal as usize, '.');
            decimals
        };
        if negative {
            return format!{"-{result}"}
        }
        return result;
    }
    // Non-Scientific
    else {
         num_str = remove_zeros(num_str);
         let negative = num_str.starts_with("-"); let num_str = num_str.trim_start_matches("-");
         let exp = num_str.split(".").nth(0).unwrap().len()-1;
         let digits = num_str.replace(".", "");
         if exp > MAX_EXPONENT {
             // We have more digits than we want -> put into scientific notation
             let mut scientific = (&digits[0..MAX_EXPONENT]).to_string();
             scientific.insert(1, '.');
             if negative {
                 return format!{"-{} * 10^{exp}", remove_zeros(scientific)}
             } else {
                 return format!{"{} * 10^{exp}", remove_zeros(scientific)}
             };
         }

         // Smaller, therefore just return num_str
         return num_str.to_string();
    }
}
