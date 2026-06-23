use crate::Option;
use cliclack::log;

pub fn is_number<T: AsRef<str>>(test_string: T) -> bool {
    match test_string.as_ref().parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Function to parse "--help" into printing help screen; Everything that is seperated by spaces and
// does not contain -- or - will be added to the exec-once query => no interactive shell
pub fn parse_cli_args(args: Vec<String>) -> Vec<Option> {
    let args = &args[1..];
    let mut option_queue = vec![];

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

        // Parsing Exec-Once
        else {
            option_queue.push(Option::SingleQuery(opt.clone()))
        }
    }

    // Return the entire list of options
    return option_queue;
}

// Printing the entire help menu for convinience purposes as a function
// TODO: finishing the Help Menu
pub fn help_menu() {
    let output = "
  Help Menu  
-------------
";
    log::info(output).unwrap();
}

pub fn error(error_message: &str) {}
