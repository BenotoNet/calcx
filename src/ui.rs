use std::io;
use tui::{backend::CrosstermBackend, Terminal};

// UI Library

// Enums for setting Options -> Every option has its own "type"
pub enum Option {
    SingleQuery(String),
}

// Function to parse "--help" into printing help screen; Everything that is seperated by spaces and
// does not contain -- or - will be added to the exec-once query => no interactive shell
pub fn parse_args(args: Vec<String>) -> Vec<Option> {
    let mut option_queue = vec![];
    let mut query = String::new();

    for opt in args {
        // Parsing all command line arguments with starting with --
        // e.g. --help
        if opt.starts_with("--") {
            // Verbose Opt
            let cmd = &opt[2..];
            match cmd {
                "help" => {help_menu()},
                _ => {}
            }
        }

        // Parsing all shorthands => e.g. -h for --help
        else if opt.starts_with("-") {
            // Short Opt
            let cmd = &opt[1..];
            match cmd {
                "h" => {help_menu()},
                _ => {}
            }
        }

        // if it contains neither -- or -, then add it to the exec-once query
        else {
            query += &opt;
        }
    }

    // If we do have an exec-once query (-> the query string is not empty) then we want to parse
    // that back to the main function
    if query != "" {
        option_queue.push(
            Option::SingleQuery(query)
        )
    }

    // Return the entire list of options
    return option_queue;
}

// Printing the entire help menu for convinience purposes as a function
// TODO: finishing the Help Menu
fn help_menu() {
    let output = "
  Help Menu  
-------------
";
    println!{"{output}"}
}


pub struct UI {

}

impl UI {
    pub fn new() -> UI {
        UI {  }
    }

    pub fn interactive(&self) {
        // Interaction loop: wait for user input -> parse user input -> query -> return output ->
        // ask for new user input

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend);
        loop {
            
        }
    }
}
