use crate::calc::Calc;

use std::process::exit;

use rustyline::DefaultEditor;
use crate::utils;

// NOTE: UI Library

// Enums for setting Options
#[derive(PartialEq)]
pub enum Setting {
    SingleQuery(String),
    Precision(usize),
    OutputOnly,
}

pub struct UI {
    calc: Calc,
    stdout: DefaultEditor,
    persistent: Vec<Setting>
}

impl UI {
    pub fn new(options: Vec<Setting>) -> UI {
        // Default Precision
        let default_precision = 15;
        let rl = DefaultEditor::new().unwrap();

        let mut ui = UI { calc: Calc::new(default_precision), stdout: rl, persistent: vec![] };
        let mut exit_after_single_queries = false;
        for option in options {
            match option {
                Setting::SingleQuery(query) => {ui.run_query(&query); exit_after_single_queries = true;},
                Setting::Precision(precision) => {ui.calc.change_precision(precision);},
                Setting::OutputOnly => ui.persistent.push(Setting::OutputOnly),
            }
        }
        if exit_after_single_queries {exit(0)};
        return ui;
    }

    pub fn interactive(mut self) {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        loop {
            // Old Way of getting input via CliClack, Deprecated because of History management
            // let query: String = input("Calcxulate!").autocomplete(self.history.clone()).interact().expect("Could not get input...");
            // New: rustyline, less styling but more useful
            let query: String = match self.stdout.readline("Calcxulate >> ") {
                Ok(input) => {input},
                Err(_) => {return},
            };

            match query.as_str() {
                "quit"|"Quit"|"QUIT"|"exit"|"Exit"|"EXIT" => {return}
                "clear" => {self.stdout.clear_screen().expect("Failed to clear screen..."); return self.interactive();}
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
            self.stdout.add_history_entry(&query).expect("Could not add query to history...?");

            self.run_query(&query);

            // Deprecated, from CliClack
            // if !self.history.contains(&query) {
            //     self.history.push(query);
            // }
        }
    }

    pub fn run_query(&mut self, query: &str) {
        if self.persistent
            .contains(
                &Setting::OutputOnly
                ) {
            println!{"{}", self.calc.run_ouput(&query)};
        } else {
            // Normal output with nice formatting
            UI::output(&self.calc.run_ouput(&query));
        }
    }

    pub fn output(output_string: &str) {
        utils::success(output_string);
    }
}
