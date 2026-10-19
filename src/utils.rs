use crate::ui::Setting;

// Function to parse "--help" into printing help screen; Everything that is seperated by spaces and
// does not contain -- or - will be added to the exec-once query => no interactive shell
pub fn parse_cli_args(args: Vec<String>) -> Vec<Setting> {
    let args = &args[1..];
    let mut option_queue = vec![];

    let mut index = 0;
    while index < args.len() {
        let opt = &args[index];

        // Command line argument
        if opt.starts_with("-") {
            match opt.as_str() {
                "--help" | "-h" => {
                    help_menu();
                    std::process::exit(0)
                }
                "--precision" | "-p" => {
                    // new precision
                    index += 1;
                    let precision: usize = opt.parse().expect("Not a valid new Precision Value!");
                    option_queue.push(Setting::Precision(precision));
                }
                "--output-only" | "-o" => {
                    option_queue.push(Setting::OutputOnly);
                }
                "--autosave" | "-s" => {
                    option_queue.push(Setting::AutoSave);
                }
                _ => {}
            }
        }
        // Otherwise, we have a normal exec-once command / commands
        else {
            if opt.contains(";") {
                // Multiple Commands in one
                for cmd in opt.split(";") {
                    option_queue.push(Setting::SingleQuery(cmd.to_string()))
                }
            } else {
                option_queue.push(Setting::SingleQuery(opt.clone()));
            }
        }
        index += 1;
    }

    // Return the entire list of options
    return option_queue;
}

pub fn success(output: &str) {
    println! {"            = {output}\n"};
}

pub fn info(output: &str) {
    println! {"{output}"}
}

// Printing the entire help menu for convinience purposes as a function
pub fn help_menu() {
    let output = "
Tutorial:
This program is a calculator with advanced features like units, constants, and user defined variables and functions. 
There are more features still on the way, look at the [todo page ](./todo.md) for more information

Command Line Interface options are: 

    -o / --output-only => output each result of each query on its own line without any formatting (useful for piping)
    -h / --help => printing this help menu / tutorial
    -p / --precision=<Integer> => The number of digits to show (correct up to ~1200 decimal digits; 4096 bits of precision)
    -s / --auto-save => automatically saves variables, functions, and history when exiting

Additionally, you can append calculations after the options in quotation marks which well then be executed (after that the program will exit, also to allow piping of commands)
Multiple commands / calculations within a single string can be seperated with semicolons (";
    "), they will be executed sequentually before the next string.

Example command:
$ ./calcx --output-only --precision=32 \"my_var = 1/2*pi\" \"sin(my_var)\" > output.txt

When the program is run without any single-execute commands, you are put into an interactive session. 
Possible Meta-Keywords:

    - clear => clears the screen
    - quit / exit / CTRL-C => exits the interactive session (CTRL-C however does not save history)
    - help => displays this help screen
    - PRECISION:{{ new precision }}
    - OUTPUT_ONLY:{{ bool 1 or 0 (on/off) }}
    - AUTOSAVE:{{ bool 1 or 0 (on/off) }}

To run a query, simply type what you want to calculate and press enter. 
Example queries to get to know the calculator and its capabilities:

    - 60 + 4.5 * (3+-1)^2 - 9
    - 5 meters / second + 5 kilo meters / hour to miles / day
    - sin((sqrt(log(10, 1000000000)) + root(3, 27)) * 10 degrees)
    - my_var = 50 meters
    - my_var^2
    - add_one(x) = x+1
    - multi_var(x, y, z) = x*y^z

Thank you for using this calculator despite of it still containing some bugs; if you find some, let me know ;)
I would love this project to have at least some use other than purely personally for me!
";
    // Print Help menu and exit
    info(output);
}
