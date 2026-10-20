use crate::calcx_core::Calc;
use crate::calcx_core::Expr;

use colored_print::cformat;
use directories::ProjectDirs;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

mod autocomplete;
use rustyline::{Config, Editor, history::DefaultHistory};

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

pub struct Paths {
    pub base: PathBuf,
    pub config: PathBuf,
    pub history: PathBuf,
}

// NOTE: Getting the paths for saving history and config
pub fn paths() -> Result<Paths, String> {
    let dirs = match ProjectDirs::from("dev", "benoto", "calcx") {
        Some(v) => v,
        _ => return Err(String::from("There was an Error getting the Paths")),
    };

    Ok(Paths {
        base: dirs.config_dir().to_path_buf(),
        config: dirs.config_dir().join("config.json"),
        history: dirs.config_dir().join("history.txt"),
    })
}

pub struct UI {
    calc: Calc,
    stdout: Editor<autocomplete::AutoComplete, DefaultHistory>,
    persistent: Vec<Setting>,
}

impl UI {
    pub fn new(options: Vec<Setting>) -> Result<UI, String> {
        // Default Precision
        let default_precision = 15;

        // change config of rustyline:
        let config = Config::builder().build();
        let mut rl = Editor::with_config(config).unwrap();

        let helper = autocomplete::AutoComplete::build();
        rl.set_helper(Some(helper));

        // Load previous settings, history, and variables
        if Path::new(&paths()?.history).exists() {
            // NOTE: Load previous history from history path
            rl.load_history(&paths()?.history)
                .expect("Could not load history");
        }

        let mut ui = UI {
            calc: Calc::new(default_precision),
            stdout: rl,
            persistent: vec![Setting::AutoSave],
        };

        if Path::new(&paths()?.config).exists() {
            // NOTE: If we have a config file present, we want to load it
            match calcx_core::persistence::load_from_file(&paths()?.config) {
                Ok(v) => {
                    // there is actual useful data here, set calculator to the loaded one
                    ui.add_autocompletes(v.get_customs());
                    ui.calc = v;
                }
                _ => {
                    println!(
                        "There was an error reading the config file, consider deleting or overwriting."
                    )
                }
            };
        }

        let mut exit_after_single_queries = false;
        for option in options {
            match option {
                Setting::SingleQuery(query) => {
                    ui.run_query(&query, true);
                    exit_after_single_queries = true;
                }
                Setting::Precision(precision) => {
                    ui.calc.change_precision(precision);
                }
                var => ui.persistent.push(var),
            }
        }
        if exit_after_single_queries {
            exit(0)
        };
        return Ok(ui);
    }

    fn add_autocompletes(&mut self, autocompletes: Vec<String>) {
        for word in autocompletes {
            self.stdout
                .helper_mut()
                .expect("could not get helper")
                .add_word(word);
        }
    }

    fn exit(&mut self, output_code: i32) -> Result<(), String> {
        if self.persistent.contains(&Setting::AutoSave) {
            self.save()?;
        };
        exit(output_code);
    }

    fn save(&mut self) -> Result<(), String> {
        // Check if the parent dir exists
        if !Path::new(&paths()?.base).exists() {
            // There is no config home folder ($HOME/.config/calcx), therefore, we want to make it
            match fs::create_dir(&paths()?.base) {
                Ok(_) => {}
                Err(e) => {
                    panic! {"There is no configuration folder present on this device?! Error: {e}"}
                }
            }
        }
        // Save config
        calcx_core::persistence::save_to_file(&self.calc, &paths()?.config).unwrap();

        // Save History
        self.stdout
            .save_history(&paths()?.history)
            .expect("Could not save history");

        self.output("Done");
        Ok(())
    }

    fn reset(&mut self) -> Result<(), String> {
        self.calc = Calc::new(15);
        self.persistent = vec![Setting::AutoSave];
        self.output("Done");
        self.stdout
            .clear_history()
            .expect("Could not clear history");
        match std::fs::remove_file(&paths()?.history) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }?;
        Ok(())
    }

    pub fn interactive(mut self) -> Result<(), String> {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        loop {
            let user_query = match cfg![windows] {
                // Disable colors because they create bugs on windows
                true => {format!{"Calcxulate >> "}},

                // Otherwise they can stay on
                false => {cformat!{"%G:%b^Calcxulate >> "}}
            };
            let query: String = match self.stdout.readline(&user_query) {
                Ok(input) => input,
                Err(e) => return Err(e.to_string()),
            };

            // We can have multiple queries at once seperated by semicolons
            for query in query.split(";") {
                let compact_query = query.replace(" ", "");
                // Change settings inside the calc:
                if compact_query.contains("PRECISION:") {
                    // Check if the next thing after ":" is a number:
                    self.calc.change_precision(
                        query
                            .split(":")
                            .nth(1)
                            .ok_or("There is no : in the query")?
                            .parse::<usize>()
                            .map_err(|e| e.to_string())?,
                    );
                    // Add a new line
                    println! {""}
                    // Skip rest of for loop iteration
                    continue;
                }
                if compact_query.contains("OUTPUT_ONLY:1") {
                    // Add the Output Only setting to the persistent storage
                    self.persistent.push(Setting::OutputOnly);

                    self.save()?;

                    // Add a new line
                    println! {""}
                    continue;
                }
                if compact_query.contains("OUTPUT_ONLY:0") {
                    // Remove all occurences of output only in the storage
                    self.persistent.retain(|v| match v {
                        Setting::OutputOnly => false,
                        _ => true,
                    });

                    self.save()?;

                    // Add a new line
                    println! {""}
                    continue;
                }

                if compact_query.contains("AUTOSAVE:1") {
                    // Add the Output Only setting to the persistent storage
                    self.persistent.push(Setting::AutoSave);

                    self.save()?;

                    // Add a new line
                    println! {""}
                    continue;
                }
                if compact_query.contains("AUTOSAVE:0") {
                    // Remove all occurences of output only in the storage
                    self.persistent.retain(|v| match v {
                        Setting::AutoSave => false,
                        _ => true,
                    });

                    self.save()?;

                    // Add a new line
                    println! {""}
                    continue;
                }

                match compact_query.as_str() {
                    "quit" | "Quit" | "QUIT" | "exit" | "Exit" | "EXIT" => {
                        self.exit(0)?;
                    }
                    "clear" => {
                        self.stdout
                            .clear_screen()
                            .expect("Failed to clear screen...");
                        return self.interactive();
                    }
                    "help" => {
                        // Printing Help Menu when typing help into the calc
                        // clear_screen().expect("Failed to clear Screen...");
                        utils::help_menu();
                        return self.interactive();
                    }

                    // NOTE: Logic for saving calculator & resetting to factory defaults
                    "reset" | "RESET" => {
                        self.reset()?;
                        return self.interactive();
                    }
                    "save" | "SAVE" => {
                        self.save()?;
                        return self.interactive();
                    }

                    "" => {
                        return self.interactive();
                    }
                    _ => {}
                }

                // Add to History
                self.stdout
                    .add_history_entry(query)
                    .expect("Could not add query to history...?");

                // to append the unit we convert to, we want to see, if there is a conversion string
                // present and then append a respective setting with appendage
                let mut conversion_present = false;
                for conversion_splitter in &[" in ", " to ", " convert_to ", " convert "] {
                    if query.contains(conversion_splitter) {
                        conversion_present = true;

                        let before = query.split(conversion_splitter).nth(0).unwrap().to_string();
                        let after = query
                            .split(conversion_splitter)
                            .nth(query.split(conversion_splitter).collect::<Vec<_>>().len() - 1)
                            .ok_or("Query could not be split using a query spliter")?
                            .to_string();

                        //  we convert by dividing the input by the wanted units
                        let query = format! {"{} / ({})", before, after};
                        let out = &self.calc.run_output(&query, false);

                        // Save to History as the multiplication of the base and conversion -> true value of conversion
                        let _ = self
                            .calc
                            .run_output(&format! {"({}) * ({})", out, after}, true);

                        // Check if unitless or if query fails:
                        match self.calc.run(&query, false) {
                            Ok(Expr::Number(num)) => {
                                if num.is_unitless() {
                                    // Everything worked and we got good output
                                    self.output(&format! {"{} [{}]", out, after});
                                } else {
                                    // not unitless -> conversion failed
                                    self.output("Conversion not possible");
                                }
                            }
                            Err(e) => {
                                // Some operation failed for some reason
                                self.output(&e);
                            }
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
        if self.persistent.contains(&Setting::OutputOnly) {
            println! {"{}", result};
        } else {
            // Normal output with nice formatting
            UI::format_output(&format! {"{}", result});
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
