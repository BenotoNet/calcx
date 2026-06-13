use crate::calc::Calc;
use cliclack::{input, log};

// UI Library

// Enums for setting Options
pub enum Option {
    SingleQuery(String),
}

pub struct UI {
    calc: Calc,
}

impl UI {
    pub fn new(calc: Calc) -> UI {
        UI { calc }
    }

    pub fn interactive(&mut self) {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        loop {
            let query: String = input("Calcxulate!").interact().expect("Could not get input...");
            if query == String::from("quit") {
                return;
            }
            let _ = log::success(self.calc.run(&query)).expect("Could not write output...");
        }
    }
}
