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
    let mut num_str = format!{"{:.*}", max_decimals, num};

    // Check for non scientific notation
    if !num_str.contains("e") && !num_str.contains("E") {
        // We do not have scientific notation, therefore, we want to strip the . in the case that we
        // don't need it
        
        // 1. Remove Trailing 0s if there is a dot
        while num_str.ends_with("0") && num_str.contains(".") {
            num_str.pop();
        }
        // 2. Remove trailing dot
        if num_str.ends_with(".") {num_str.pop();}
        return num_str
    }

    return num_str;
}

// Written by AI, sorry...
pub fn format_float_ai(x: &Float, max_decimals: usize) -> String {
    let s = format!("{:.*}", max_decimals, x);

    // If it's already not scientific notation, just trim it.
    if !s.contains('e') && !s.contains('E') {
        return trim(s);
    }

    let (mantissa, exp) = s
        .split_once(['e', 'E'])
        .unwrap();

    let exp: i32 = exp.parse().unwrap();

    // Keep scientific notation for very large/small numbers.
    if exp >= 6 || exp <= -6 {
        return trim_scientific(s);
    }

    // Expand scientific notation.
    let negative = mantissa.starts_with('-');
    let mantissa = mantissa.trim_start_matches('-');

    let mut digits: String = mantissa.chars().filter(|&c| c != '.').collect();
    let decimal_pos = mantissa.find('.').unwrap_or(mantissa.len()) as i32;
    let new_decimal = decimal_pos + exp;

    let result = if new_decimal <= 0 {
        format!("0.{}{}", "0".repeat((-new_decimal) as usize), digits)
    } else if new_decimal >= digits.len() as i32 {
        digits.push_str(&"0".repeat((new_decimal as usize) - digits.len()));
        digits
    } else {
        digits.insert(new_decimal as usize, '.');
        digits
    };

    let result = trim(result);

    if negative {
        format!("-{}", result)
    } else {
        result
    }
}

fn trim(mut s: String) -> String {
    if let Some(dot) = s.find('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.len() == dot {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    s
}

fn trim_scientific(s: String) -> String {
    let (mantissa, exp) = s.split_once(['e', 'E']).unwrap();
    format!("{}e{}", trim(mantissa.to_string()), exp)
}
