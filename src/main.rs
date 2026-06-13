// Internal Libraries
mod ui;
mod calc;
mod number_struct;
mod utils;

use std::env::args;
use ui::Option;

fn main() {
    let args = args().collect::<Vec<String>>();

    let mut calculator = calc::Calc::new();

    let options = utils::parse_cli_args(args);

    // TODO: 
    for option in options {
        match option {
            _ => {},
        }
    }

    // All options are set, therefore, we can start the interactive session
    let mut ui = ui::UI::new(calculator);
    ui.interactive();
}
