use crate::calcx_core::Calc;

use std::process::exit;
use std::borrow::Cow;

use rustyline::{
    Editor, 
    Helper, 
    completion::{
        Completer, 
        Pair
    }, 
    highlight::Highlighter, 
    hint::Hinter, 
    validate::Validator,
    history::DefaultHistory,
};

mod autocomplete;
pub struct AutoComplete {
    options: Vec<String>
}

impl AutoComplete {
    fn build() -> AutoComplete {
        let options = autocomplete::get_options();
        AutoComplete { options }
    }

    fn get_current_word(line: &str, pos: usize) -> String {
        let start = &line[..pos];
        start.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace()).last().unwrap().to_string()
    }

    fn match_possible_word_completions(word_list: &Vec<String>, word_part: &str) -> Vec<Pair> {
        let mut output = vec![];
        for word in word_list {
            if word.starts_with(word_part) {
                let word_ending_to_completion = &word[word_part.len()..];
                output.push(word_ending_to_completion.to_string());
            }
        }
        return output.iter().map(|v| {Pair { display: v.clone(), replacement: v.clone()}}).collect();
    }


    fn match_possible_words(word_list: &Vec<String>, word_part: &str) -> Vec<Pair> {
        let mut output = vec![];
        for word in word_list {
            if word.starts_with(word_part) {
                output.push(word.clone())
            }
        }
        return output.iter().map(|v| {Pair { display: v.clone(), replacement: v.clone()}}).collect();
    }
}

impl Validator for AutoComplete {}
impl Helper for AutoComplete {}

impl Highlighter for AutoComplete {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // e.g. dim cyan hint
        Cow::Owned(format!("\x1b[38;5;244m{hint}\x1b[0m"))
    }
}

impl Hinter for AutoComplete {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        let word = AutoComplete::get_current_word(line, pos);
        if word != "" {
            return Some(AutoComplete::match_possible_word_completions(&self.options, &word)
                .get(0)?
                .replacement
                .clone())
        }
        return None;
    }
}

impl Completer for AutoComplete {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)>
    {
        let word = AutoComplete::get_current_word(line, pos);
        if word != "" {
            return Ok((pos-word.len(), AutoComplete::match_possible_words(&self.options, &word)))
        }
        return Ok((0, vec![]));
    }
}

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
    stdout: Editor<AutoComplete, DefaultHistory>,
    persistent: Vec<Setting>
}

impl UI {
    pub fn new(options: Vec<Setting>) -> UI {
        // Default Precision
        let default_precision = 15;
        let mut rl = Editor::new().unwrap();

        let helper = AutoComplete::build();
        rl.set_helper(Some(helper));

        let mut ui = UI { calc: Calc::new(default_precision), stdout: rl, persistent: vec![] };
        let mut exit_after_single_queries = false;
        for option in options {
            match option {
                Setting::SingleQuery(query) => {ui.run_query(&query); exit_after_single_queries = true;},
                Setting::Precision(precision) => {ui.calc.change_precision(precision);},
                var => ui.persistent.push(var),
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

            // We can have multiple queries at once seperated by semicolons
            for query in query.split(";") {
                let compact_query = query.replace(" ", "");
                // Change settings inside the calc:
                if compact_query.contains("PRECISION:") {
                    // FIX: REMOVE UNWRAP
                    self.calc.change_precision(query.split(":").nth(1).unwrap().parse::<usize>().unwrap());
                    // Skip rest of for loop iteration
                    continue;
                }
                match compact_query.as_str() {
                    "quit"|"Quit"|"QUIT"|"exit"|"Exit"|"EXIT" => {exit(0)}
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
                self.stdout.add_history_entry(query).expect("Could not add query to history...?");

                self.run_query(&query);
            }
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
