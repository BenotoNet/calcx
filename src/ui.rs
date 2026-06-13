use crate::calc::Calc;
use cliclack::{input, log};
use rustyline::DefaultEditor;

// UI Library

// Enums for setting Options
pub enum Option {
    SingleQuery(String),
}

pub struct UI {
    calc: Calc,
    history: Vec<String>,
}

impl UI {
    pub fn new(calc: Calc) -> UI {
        UI { calc, history: vec![] }
    }

    pub fn interactive(mut self) {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        let mut rl = DefaultEditor::new().unwrap();

        loop {
            // Old Way of getting input via CliClack
            // let query: String = input("Calcxulate!").autocomplete(self.history.clone()).interact().expect("Could not get input...");
            // New: rustyline
            let query: String = rl.readline("Calcxulate! ").expect("Could not get Input...");

            if query == String::from("quit") {
                return;
            }
            
            // Add to History
            rl.add_history_entry(&query).expect("Could not add query to history...?");

            log::success(self.calc.run(&query)).expect("Could not write output...");

            // if !self.history.contains(&query) {
            //     self.history.push(query);
            // }

        }
    }
}
