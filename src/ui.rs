// UI Library

pub enum Option {
    SingleQuery(String),
}

pub fn parse_args(args: Vec<String>) -> Vec<Option> {
    let mut option_queue = vec![];
    let mut query = String::new();

    for opt in args {
        if opt.starts_with("--") {
            // Verbose Opt
            let cmd = &opt[2..];
            match cmd {
                "help" => {help_menu()},
                _ => {}
            }
        }
        else if opt.starts_with("-") {
            // Short Opt
            let cmd = &opt[1..];
            match cmd {
                "h" => {help_menu()},
                _ => {}
            }
        }
        else {
            query += &opt;
        }
    }

    if query != "" {
        option_queue.push(
            Option::SingleQuery(query)
        )
    }

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
