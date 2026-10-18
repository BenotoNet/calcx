use crate::calcx_core::Calc;
use crate::calcx_core::Expr;

use colored_print::cformat;

use std::process::exit;
use std::path::Path;
use std::fs;

mod autocomplete;
use rustyline::{Editor, history::DefaultHistory, Config};

use crate::utils;

// NOTE: UI Library

// Enums for setting Options
#[derive(PartialEq, Clone)]
pub enum Setting {
    SingleQuery(String),
    Precision(usize),
    OutputOnly,
    AutoSave,
}

// NOTE: Getting the paths for saving history and config
fn home_path() -> String {format!{"{}/.config/calcx", std::env::home_dir().unwrap().display()} }
fn history_path() -> String {format!{"{}/history.txt", home_path()}}
fn config_path() -> String {format!{"{}/config.json", home_path()}}

pub struct UI {
    calc: Calc,
    stdout: Editor<autocomplete::AutoComplete, DefaultHistory>,
    persistent: Vec<Setting>
}

impl UI {
    pub fn new(options: Vec<Setting>) -> UI {
        // Default Precision
        let default_precision = 15;

        // change config of rustyline: 
        let config = Config::builder()
            .build();
        let mut rl = Editor::with_config(config).unwrap();

        let helper = autocomplete::AutoComplete::build();
        rl.set_helper(Some(helper));

        let mut ui = UI { calc: Calc::new(default_precision), stdout: rl, persistent: vec![Setting::AutoSave] };

        // Load previous settings, history, and variables
        if Path::new(&history_path()).exists() {
            // NOTE: Load previous history from history path
            // FIX: currently not implemented
            // rl.load_history(HISTORY_PATH).unwrap();
        }
        if Path::new(&config_path()).exists() {
            // NOTE: If we have a config file present, we want to load it
            match calcx_core::persistence::load_from_file(&config_path()) {
                Ok(v) => {
                    // there is actual useful data here, set calculator to the loaded one
                    ui.add_autocompletes(v.get_customs());
                    ui.calc = v;
                },
                _ => {println!("There was an error reading the config file, consider deleting or overwriting.")},
            };
        }
        

        let mut exit_after_single_queries = false;
        for option in options {
            match option {
                Setting::SingleQuery(query) => {ui.run_query(&query, true); exit_after_single_queries = true;},
                Setting::Precision(precision) => {ui.calc.change_precision(precision);},
                var => ui.persistent.push(var),
            }
        }
        if exit_after_single_queries {exit(0)};
        return ui;
    }

    fn add_autocompletes(&mut self, autocompletes: Vec<String>) {
        for word in autocompletes {
            self.stdout.helper_mut().expect("could not get helper").add_word(word);
        }
    }

    fn exit(&self, output_code: i32) -> Result<(), String> {
        if self.persistent.contains(&Setting::AutoSave) {
            self.save()?;
        };
        exit(output_code);
    }

    fn save(&self) -> Result<(), String> {
        // Check if the parent dir exists
        if !Path::new(&home_path()).exists() {
            // There is no config home folder ($HOME/.config/calcx), therefore, we want to make it
            match fs::create_dir(home_path()) {
                Ok(_) => {},
                Err(e) => {panic!{"There is no configuration folder present on this device?! Error: {e}"}}
            }
        }
        // Save config
        calcx_core::persistence::save_to_file(&self.calc, &config_path()).unwrap(); 

        // Save History
        // FIX: currently not implemented
        // self.stdout.save_history(HISTORY_PATH).unwrap();

        self.output("Done");
        Ok(())
    }

    pub fn interactive(mut self) -> Result<(), String> {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        loop {
            // New: rustyline, less styling but more useful
            let query: String = match self.stdout.readline(&cformat!{"%G:%b^Calcxulate >> "}) {
                Ok(input) => {input},
                Err(e) => {return Err(e.to_string())},
            };

            // We can have multiple queries at once seperated by semicolons
            for query in query.split(";") {
                let compact_query = query.replace(" ", "");
                // Change settings inside the calc:
                if compact_query.contains("PRECISION:") {
                    // Check if the next thing after ":" is a number:
                    self.calc.change_precision(query.split(":").nth(1)
                        .ok_or("There is no : in the query")?
                        .parse::<usize>()
                        .map_err(|e| {e.to_string()})?);
                    // Add a new line
                    println!{""}
                    // Skip rest of for loop iteration
                    continue;
                }
                if compact_query.contains("OUTPUT_ONLY:1") {
                    // Add the Output Only setting to the persistent storage
                    self.persistent.push(Setting::OutputOnly);

                    self.save()?;

                    // Add a new line
                    println!{""}
                    continue;
                }
                if compact_query.contains("OUTPUT_ONLY:0") {
                    // Remove all occurences of output only in the storage
                    self.persistent.retain(|v| {match v {
                        Setting::OutputOnly => false,
                        _ => true,
                    }});

                    self.save()?;

                    // Add a new line
                    println!{""}
                    continue;
                }

                if compact_query.contains("AUTOSAVE:1") {
                    // Add the Output Only setting to the persistent storage
                    self.persistent.push(Setting::AutoSave);

                    self.save()?;

                    // Add a new line
                    println!{""}
                    continue;
                }
                if compact_query.contains("AUTOSAVE:0") {
                    // Remove all occurences of output only in the storage
                    self.persistent.retain(|v| {match v {
                        Setting::AutoSave => false,
                        _ => true,
                    }});

                    self.save()?;

                    // Add a new line
                    println!{""}
                    continue;
                }

                match compact_query.as_str() {
                    "quit"|"Quit"|"QUIT"|"exit"|"Exit"|"EXIT" => {
                        self.exit(0)?;
                    }
                    "clear" => {self.stdout.clear_screen().expect("Failed to clear screen..."); return self.interactive();}
                    "help" => {
                        // Printing Help Menu when typing help into the calc
                        // clear_screen().expect("Failed to clear Screen..."); 
                        utils::help_menu();
                        return self.interactive();
                    }

                    // NOTE: Logic for saving calculator & resetting to factory defaults
                    "reset"|"RESET" => {
                        self.calc = Calc::new(15); 
                        self.persistent = vec![Setting::AutoSave]; 
                        self.output("Done");
                        return self.interactive()
                    },
                    "save"|"SAVE" => {
                        self.save()?;
                        return self.interactive();
                    }

                    "" => {return self.interactive();}
                    _ => {}
                }
                
                // Add to History
                self.stdout.add_history_entry(query).expect("Could not add query to history...?");

                // to append the unit we convert to, we want to see, if there is a conversion string
                // present and then append a respective setting with appendage
                let mut conversion_present = false;
                for conversion_splitter in &[" in ", " to ", " convert_to ", " convert "] {
                    if query.contains(conversion_splitter) {
                        conversion_present = true;

                        let before = query.split(conversion_splitter).nth(0).unwrap().to_string();
                        let after = query.split(conversion_splitter)
                                .nth(
                                    query.split(conversion_splitter).collect::<Vec<_>>().len()-1)
                                .ok_or("Query could not be split using a query spliter")?
                                .to_string();

                        //  we convert by dividing the input by the wanted units
                        let query = format!{"{} / ({})", before, after};
                        let out = &self.calc.run_output(&query, false);

                        // Save to History as the multiplication of the base and conversion -> true value of conversion
                        let _ = self.calc.run_output(&format!{"({}) * ({})", out, after}, true);

                        // Check if unitless or if query fails:
                        match self.calc.run(&query, false) {
                            Ok(Expr::Number(num)) => {
                                if num.is_unitless() {
                                    // Everything worked and we got good output
                                    self.output(&format!{"{} [{}]", out, after});
                                }
                                else {
                                    // not unitless -> conversion failed
                                    self.output("Conversion not possible");
                                }
                            },
                            Err(e) => {
                                // Some operation failed for some reason
                                self.output(&e);
                            },
                            Ok(_) => {
                                self.output("Something strange happened and the query failed...");
                            }
                        }
                    }
                }
                // A normal query without any conversion keyword
                if !conversion_present {
                    self.run_query(&query, true);
                }

                // Append user-defined functions to autocomplete options
                self.add_autocompletes(self.calc.get_customs());
            }
        }
    }

    pub fn output(&self, result: &str) {
        if self.persistent
            .contains(
                &Setting::OutputOnly
                ) {
                    println!{"{}", result};
                } 
        else {
            // Normal output with nice formatting
            UI::format_output(&cformat!{"{}", result});
        }
    }

    pub fn run_query(&mut self, query: &str, save_history: bool) {
        // Have empty append unit string by default
        let out = self.calc.run_output(query, save_history);
        self.output(&out);
    }

    pub fn format_output(output_string: &str) {
        utils::success(output_string);
    }
}
