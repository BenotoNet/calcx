mod ui;
mod parser;
use std::env::args;
use ui::Option;
mod basic_calc;

fn main() {
    let args = args().collect::<Vec<String>>();

    let parser = parser::Parser::new();

    let options = ui::parse_args(args);

    for option in options {
        match option {
            Option::SingleQuery(query) => {parser.parse(&query)},
            _ => {},
        }
    }
}
