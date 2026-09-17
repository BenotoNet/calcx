pub fn get_options() -> Vec<String> { "meter, meters, metre, metres, second, seconds, secs, sec, gram, grams, kilogram, kilograms, ampere, amperes, kelvin, candela, candelas, exa, peta, tera, giga, mega, kilo, hecto, deca, deci, centi, milli, micro, nano, pico, ton, kibi, mebi, gibi, tebi, pebi, exbi, minute, minutes, hour, hours, hr, hrs, day, days, week, weeks, year, years, decade, decades, century, centuries, byte, bytes, octet, bit, bits, newton, newtons, pascal, pascals, joule, joules, british_thermal_units, british_thermal_unit, btu, btus, cal, calorie, kcal, watt, watts, horsepower, coulomb, volt, volts, ohm, ohms, siemens, sievert, gray, farad, weber, hertz, hz, bar, bars, litre, liter, liters, litres, atomic_mass_unit, u, amu, planck, placks_constant, electron_volt, eV, elementary_charge, lumen, lumens, lux, astronomical_unit, astronomical_units, au, gravity, gravity_constant, atm, atmosphere, atmospheric_pressure, mach, room_temperature, standard_temperature, degree, degrees, micron, microns, circle, circles, pi, golden_ratio, e, inf, lightspeed, light, speed_of_light, percent, %, percents, ppm, partspermillion, ppb, partsperbillion, ppt, partspertrillion, karat, inch, inches, foot, feet, yard, yards, mile, miles, cup, cups, tablespoon, tbl, tbsp, tblsp, teaspoon, tsp, pound, pounds, lb, lbs, grain, grains, ounce, ounces, oz, gallon_UK, gallon_US, knot, knots,
add_one, root, nth_root, n_root, log, sqrt, square_root, 2root, root2, sin, sine, cos, cosine, tan, tangent, arcsine, arcsin, asin, asine, arccosine, arccos, arcos, acos, acosine, arctan, arctangent, atan, atangent, ln, natural_log, natural_ln, log_natural, lg, log10, log_base_10, log_base_ten, log2, 2log, log_base_2, log_base_two, exp, round_down, floor, rdown, roundd, round_up, ceil, rup, roundu, ceiling, round, abs, absolute, absol, absolutes, to, in, convert, convert_to, ans, last, answer, last_answer, nth_last, nth_ans, clip, copy, clipboard
PRECISION:, help, exit, quit, clear, OUTPUT_ONLY:,
eq, equal, is_equal, is_equal_to, equal_to, gt, greater, greater_than, lt, less, less_than,
c_to_k, celsius_to_kelvin, celsius, units, get_units, unit, save, reset
"
        .split(&[' ', ',', '\n'])
        .into_iter()
        .map(|v| {v.to_string()})
        .collect()
}

use std::borrow::Cow;
use rustyline::{
    Helper, 
    completion::{
        Completer, 
        Pair
    }, 
    highlight::Highlighter, 
    hint::Hinter, 
    validate::Validator,
};

pub struct AutoComplete {
    options: Vec<String>
}

impl AutoComplete {
    pub fn build() -> AutoComplete {
        let options = get_options();
        AutoComplete { options }
    }

    fn get_current_word(line: &str, pos: usize) -> String {
        let start = &line[..pos];
        start.split(&[' ', '\n', '.', ',']).last().expect("Somehow the Autocomplete Line was completely empty").to_string()
    }

    fn match_possible_word_completions(word_list: &Vec<String>, word_part: &str) -> Vec<Pair> {
        let mut output = vec![];
        for word in word_list {
            if word.starts_with(word_part) {
                let word_ending_to_completion = &word[word_part.len()..];
                output.push(word_ending_to_completion.to_string());
            }
        }
        let mut output: Vec<Pair> = output.iter()
            .map(|v| {Pair { display: v.clone(), replacement: v.clone()}})
            .collect();

        // Sort pairs by length before returning the vector
        output.sort_by(|a, b| {a.display.len().cmp(&b.display.len())});
        return output;
    }


    fn match_possible_words(word_list: &Vec<String>, word_part: &str) -> Vec<Pair> {
        let mut output = vec![];
        for word in word_list {
            if word.starts_with(word_part) {
                output.push(word.clone())
            }
        }
        let mut output: Vec<Pair> = output.iter()
            .map(|v| {Pair { display: v.clone(), replacement: v.clone()}})
            .collect();

        // Sort pairs by length before returning the vector
        output.sort_by(|a, b| {a.display.len().cmp(&b.display.len())});
        return output;
    }
}

impl Validator for AutoComplete {}
impl Helper for AutoComplete {}

impl Highlighter for AutoComplete {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // e.g. dim cyan hint
        Cow::Owned(format!("\x1b[38;5;244m{hint}\x1b[0m"))
    }
}

impl Hinter for AutoComplete {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        let word = AutoComplete::get_current_word(line, pos);
        if word != "" {
            return Some(AutoComplete::match_possible_word_completions(&self.options, &word)
                .get(0)?
                .replacement
                .clone())
        }
        return None;
    }
}

impl Completer for AutoComplete {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)>
    {
        let word = AutoComplete::get_current_word(line, pos);
        if word != "" {
            return Ok((pos-word.len(), AutoComplete::match_possible_words(&self.options, &word)))
        }
        return Ok((0, vec![]));
    }
}
