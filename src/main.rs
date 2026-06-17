// Internal Libraries
mod ui;
mod calc;
mod utils;

use crate::calc::num::Num;

use std::env::args;
use ui::Option;

fn main() {
    let args = args().collect::<Vec<String>>();

    let options = utils::parse_cli_args(args);

    let num1 = Num::new(1.0, vec![('s', 2), ('m', 1)]);
    let num2 = Num::new(1.0, vec![('m', 1), ('s', 2)]);

    println!{"{:?}", num1.add(&num2)}

    // All options are set, therefore, we can start the interactive session with the options
    ui::UI::new(options).interactive();
}
