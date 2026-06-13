// Internal Libraries
mod ui;
mod calc;
mod number_struct;
mod utils;

use std::env::args;
use ui::Option;

fn main() {
    let args = args().collect::<Vec<String>>();

    let calculator = calc::Calc::new();

    let options = utils::parse_cli_args(args);

    // All options are set, therefore, we can start the interactive session
    let ui = ui::UI::new(calculator, options);
    ui.interactive();
}
