use crate::calc::Calc;

use std::process::exit;

use cliclack::{log, clear_screen};
use rustyline::DefaultEditor;
use crate::utils;

// NOTE: UI Library

// Enums for setting Options
pub enum Option {
    SingleQuery(String),
}

pub struct UI {
    calc: Calc,
}

impl UI {
    pub fn new(options: Vec<Option>) -> UI {
        let mut ui = UI { calc: Calc::new() };
        for option in options {
            match option {
                Option::SingleQuery(query) => {ui.run_query(&query); exit(1);},
            }
        }
        return ui;
    }

    pub fn interactive(mut self) {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        let mut rl = DefaultEditor::new().unwrap();

        loop {
            // Old Way of getting input via CliClack, Deprecated because of History management
            // let query: String = input("Calcxulate!").autocomplete(self.history.clone()).interact().expect("Could not get input...");
            // New: rustyline, less styling but more useful
            let query: String = match rl.readline("Calcxulate >> ") {
                Ok(input) => {input},
                Err(_) => {return},
            };

            match query.as_str() {
                "quit"|"Quit"|"QUIT"|"exit"|"Exit"|"EXIT" => {return}
                "clear" => {clear_screen().expect("Failed to clear screen..."); return self.interactive();}
                "help" => {
                    // Printing Help Menu when typing help into the calc
                    // clear_screen().expect("Failed to clear Screen..."); 
                    utils::help_menu();
                    return self.interactive();
                }
                "" => {return self.interactive();}
                _ => {}
            }
            
            // Add to History
            rl.add_history_entry(&query).expect("Could not add query to history...?");

            self.run_query(&query);

            // Deprecated, from CliClack
            // if !self.history.contains(&query) {
            //     self.history.push(query);
            // }
        }
    }

    pub fn run_query(&mut self, query: &str) {
        log::success(self.calc.run(&query)).expect("Could not write output...");
    }
}
