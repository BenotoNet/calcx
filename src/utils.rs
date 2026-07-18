use crate::ui::Setting;
use crate::Float;
use crate::calc::num::Num;

pub fn is_number<T: AsRef<str>>(test_string: T) -> bool {
    match test_string.as_ref().parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_int(num: &Num) -> bool {
    num.round_to(3).unwrap() == num.round().unwrap()
}

// Clean Output without CliClack
pub fn success(output: &str) {
    println!{"> {output}\n"};
}

pub fn info(output: &str) {
    println!{"{output}"}
}

// Function to parse "--help" into printing help screen; Everything that is seperated by spaces and
// does not contain -- or - will be added to the exec-once query => no interactive shell
pub fn parse_cli_args(args: Vec<String>) -> Vec<Setting> {
    let args = &args[1..];
    let mut option_queue = vec![];

    let mut index = 0;
    while index < args.len() {
        let opt = args.get(index).unwrap();

        // Command line argument
        if opt.starts_with("-") {
            match opt.as_str() {
                "--help"|"-h" => {help_menu(); std::process::exit(0)},
                "--precision"|"-p" => {
                    // new precision
                    index += 1;
                    let precision: usize = args.get(index).unwrap().parse().expect("Not a valid new Precision Value!");
                    option_queue.push(Setting::Precision(precision));
                }
                "--output-only"|"-o" => {
                    option_queue.push(Setting::OutputOnly);
                }
                _ => {},
            }
        }

        // Otherwise, we have a normal exec-once command / commands
        else {
            if opt.contains(";") {
                // Multiple Commands in one
                for cmd in opt.split(";") {
                    option_queue.push(Setting::SingleQuery(cmd.to_string()))
                }
            }
            else {
                option_queue.push(Setting::SingleQuery(opt.clone()));
            }
        }
        index += 1;
    }

    // Return the entire list of options
    return option_queue;
}

// Printing the entire help menu for convinience purposes as a function
pub fn help_menu() {
    let output = "
  Help Menu  
-------------
This program is a calculator with advanced features like units, functions, variables and many more to come. 
Most things are designed as intuitively as possible (at least from a programmer's perspective)

Command Line Interface options are: 
    -o / --output-only => output each result of each query on its own line without any formatting (useful for piping)
    -h / --help => printing this help menu
    -p / --precision <Integer> => The number of digits to show (correct up to ~1200 decimal digits; 4096 bits of precision)

Additionally, you can append calculations after the options in quotation marks which well then be executed (after that the program will exit, also to allow piping of commands)

Example command:
$ ./calcx --output-only --precision 32 \"sqrt(2)\" \"-tan(pi)\" > output.txt

When the program is run without any single-execute commands, you are thrown into an interactive session. 
Possible Meta-Keywords:
    - clear => clears the screen
    - quit / exit / CTRL-C => exits the interactive session
    - help => displays this help screen

To run a query, simply type the query and press enter. 
Example queries to get to know the calculator and its capabilities:
    - 60 + 4.5 * (3+-1)^2 - 9
    - 5 meters / second + 5 kilo meters / hour to miles / day
    - sin((sqrt(log(10, 1000000000)) + root(3, 27)) * 10 degrees)
    - my_var = 50 meters
    - my_var^2

Thank you for using this calculator despite it being unfinished and still rather buggy :)
I would love to have at least one program be any use to anybody other than me
";
    // Print Help menu and exit
    info(output);
}

// Written by AI, sorry...
pub fn format_float(x: &Float, max_decimals: usize) -> String {
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

// Old Version
// pub fn format_float(num: &Float, max_decimals: usize) -> String {
//     let mut s = format!{"{:.*}", max_decimals, num};
//
//     if s.contains('.') {
//         s = s.trim_end_matches('0')
//             .trim_end_matches('.')
//             .to_string();
//     }
//
//     return s;
// }
