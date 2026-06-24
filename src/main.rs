// Internal Libraries
mod ui;
mod calc;
mod utils;

use calc::num::Num;
use calc::units::Units;

use std::env::args;
use ui::Option;

fn main() {
    let args = args().collect::<Vec<String>>();

    let options = utils::parse_cli_args(args);

    println!{"{}", calc::conversion::can_be_converted(Num::new(1.0, vec![('s', 1)]), "second")};

    // All options are set, therefore, we can start the interactive session with the options
    ui::UI::new(options).interactive();
}
