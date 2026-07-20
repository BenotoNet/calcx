// Internal Libraries
mod ui;
mod utils;

// Core Library
use calcx_core;

use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>();

    let options = utils::parse_cli_args(args);

    // All options are set, therefore, we can start the interactive session with the options
    ui::UI::new(options).interactive();
}
