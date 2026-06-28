// Internal Libraries
mod ui;
mod calc;
mod utils;

use calc::num::Num;
use rug::Float;

use std::env::args;
use ui::Option;

const PRECISION: u32 = 4096;

fn main() {
    let args = args().collect::<Vec<String>>();

    let options = utils::parse_cli_args(args);

    println!{"{}", Float::with_val(PRECISION, 1.02)}

    // All options are set, therefore, we can start the interactive session with the options
    ui::UI::new(options).interactive();
}
