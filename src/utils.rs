use crate::ui::Setting;

// Function to parse "--help" into printing help screen; Everything that is seperated by spaces and
// does not contain -- or - will be added to the exec-once query => no interactive shell
pub fn parse_cli_args(args: Vec<String>) -> Vec<Setting> {
    let args = &args[1..];
    let mut option_queue = vec![];

    let mut index = 0;
    while index < args.len() {
        let opt = args.get(index).unwrap();

        // Command line argument
        if opt.starts_with("-") {
            match opt.as_str() {
                "--help"|"-h" => {help_menu(); std::process::exit(0)},
                "--precision"|"-p" => {
                    // new precision
                    index += 1;
                    let precision: usize = args.get(index).unwrap().parse().expect("Not a valid new Precision Value!");
                    option_queue.push(Setting::Precision(precision));
                }
                "--output-only"|"-o" => {
                    option_queue.push(Setting::OutputOnly);
                }
                _ => {},
            }
        }

        // Otherwise, we have a normal exec-once command / commands
        else {
            if opt.contains(";") {
                // Multiple Commands in one
                for cmd in opt.split(";") {
                    option_queue.push(Setting::SingleQuery(cmd.to_string()))
                }
            }
            else {
                option_queue.push(Setting::SingleQuery(opt.clone()));
            }
        }
        index += 1;
    }

    // Return the entire list of options
    return option_queue;
}

// Clean Output without CliClack
pub fn success(output: &str) {
    println!{"> {output}\n"};
}

pub fn info(output: &str) {
    println!{"{output}"}
}

// Printing the entire help menu for convinience purposes as a function
pub fn help_menu() {
    let output = "
  Help Menu  
-------------
This program is a calculator with advanced features like units, functions, variables and many more to come. 
Most things are designed as intuitively as possible (at least from a programmer's perspective)

Command Line Interface options are: 
    -o / --output-only => output each result of each query on its own line without any formatting (useful for piping)
    -h / --help => printing this help menu
    -p / --precision <Integer> => The number of digits to show (correct up to ~1200 decimal digits; 4096 bits of precision)

Additionally, you can append calculations after the options in quotation marks which well then be executed (after that the program will exit, also to allow piping of commands)

Example command:
$ ./calcx --output-only --precision 32 \"sqrt(2)\" \"-tan(pi)\" > output.txt

When the program is run without any single-execute commands, you are thrown into an interactive session. 
Possible Meta-Keywords:
    - clear => clears the screen
    - quit / exit / CTRL-C => exits the interactive session
    - help => displays this help screen

To run a query, simply type the query and press enter. 
Example queries to get to know the calculator and its capabilities:
    - 60 + 4.5 * (3+-1)^2 - 9
    - 5 meters / second + 5 kilo meters / hour to miles / day
    - sin((sqrt(log(10, 1000000000)) + root(3, 27)) * 10 degrees)
    - my_var = 50 meters
    - my_var^2

Thank you for using this calculator despite it being unfinished and still rather buggy :)
I would love to have at least one program be any use to anybody other than me
";
    // Print Help menu and exit
    info(output);
}
