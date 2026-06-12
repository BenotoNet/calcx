// Internal Libraries
mod ui;
mod calc;
mod number_struct;

use std::env::args;
use ui::Option;

fn main() {
    let args = args().collect::<Vec<String>>();

    let mut calculator = calc::Calc::new();

    let options = ui::parse_args(args);

    for option in options {
        match option {
            Option::SingleQuery(query) => {println!("{}", calculator.run(&query));},
            // _ => {},
        }
    }

    // All options
}
