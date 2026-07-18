# Project Description
Hey there!
This is my personal project, so don't expect high code quality or any type of reliability; there are already known bugs I will not bother to fix...
It is simply and solely a hobby project. 

Tutorial:
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


In case someone likes the project idea and sees the vision, you are welcome to submit bug fixes or even code improvements and I might (very probably) take a look at it.
 
The purpose of this Project is to create a personalized calculator in Rust implementing the following functions / requirements / ToDos:
- [x] normal math operations (+, -, *, /, (), ^, etc)
- [x] leaving out * for multiplication
- [x] Getting negative Numbers to work
- [x] Refactoring of Document Structure (more splitting into smaller files)
- [x] add powers of 10 keywords functionality
- [x] Adding all obscure units (volts -> Si units)
- [x] Handling Keywords (e.g. "to" correctly)
- [x] System for unit-conversion (-> 5 gram to kilogram)
- [x] working with units (-> kilo gram to gram)
- [x] Variables & Assignments (-> "hello = 2", "2*hello => 4")
- [x] Variable Multiplication if left out (hello world => hello * world)
- [x] Functions like sqrt() or sin()
- [x] ans / answer keyword for the result of last calculation
- [x] Copy to clipboard function
- [x] high ceiling for overflow (-> e.g. multiplying very large numbers with each other) (-> For that I will need to change from f64 to a custom type (probably external crate...))
- [x] Fix (-3)^5 => NaN
- [x] Fix (-2)-2 != 4 (--> -2 * -2 is what is being calculated; same as above)
- [x] Fix 5 05 => 55 (should be 505)
- [x] FIX 5 10 => 60
- [x] Fix -(5+5) (currently crashes)
- [x] Fix 10^31 planck => Is supposedly not unitless
- [x] high precision (-> depending on user settings, but at least maximum of 1024 decimal digits) (-> See above)
- [x] Make --output-only cli option with -o
- [x] Improve Precision on "Variables (e.g. Pi)"
- [x] user defined settings (precision, etc)
- [x] implement other misc functions: ln, exp, arcsin, arccos, floor, ...
- [x] Functions (with multiple arguments)
- [x] Fix bugs for Functions: add_one(add_one(5)) should be 7
- [x] Fix: sqrt(add_one(8)) should be 3
- [x] Fix 2 add_one(5) & 2 * add_one(5)
- [x] Fix 5-+2 (originally gave error)
- [x] Procedual categorization of functions by analyzing output Errors
- [x] Port simple functions (except "to") to be functions which could take multiple arguments (-> Having everything in one place)
- [x] Fix "hello = 2; add_one(hello)" => Variables as Arguments (-> Don't make a new calculator, use the current one (because variable storage is not transfered over currently), or replicate the entire calculator)
- [x] Add Semicolon to normal queries as well to execute multiple at once (e.g. 4+4; 5+5)
- [x] Fix 2 sqrt(2), 2*sqrt(2), sqrt(2) * 2, etc (-> Maybe with Brackets or by porting to function logic)
- [x] Fractional Units (e.g. Sqrt(second))
- [x] Replace all Option<Expr> with Result<Expr, String> to be able to use ?
- [x] stop frequent crashes and handle errors instead (-> Operation is impossible, might require dimensionless argument, etc)
- [x] Fix 1-2^2 (currently gives answer 5 since 1+(-2)^2 is what is being calculated) -> Fix by using context to switch between - as *-1 and - as binary operation
- [x] Make an extensive testing suite => testing if it gives the same answers as expected (use AI to generate testing suite?)
- [x] Help Menu
- [ ] Nicer display of output units + System for doing to keyword (appending unit string?)
- [ ] live fetching of regularly updated data (e.g. 4 USD to EUR, 0.001 BTC to EUR)
- [ ] Remove all possible Cloning of Floats & Units
<!-- - [ ] working with dates & time (e.g. 14:00 + 5 hours) (-> I need to figure out a good way to do this, as it's not very easy without clipping other features -> 14:00 won't work) -->
- [ ] custom user-defined functions (add_one(number) = number + 1, etc)
- [ ] saving functions to profile (+ settings, Variables (only after save keyword?; Drop Variable keyword? Reset?), history?)
- [ ] HEX to RGB, etc


Wow, the more I thought about this, the more this seems like a *HUGE* project. Let's see how long I will last before I will quit... xD
