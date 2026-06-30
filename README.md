# Project Description
Hey there!
This is my personal project, so don't expect high code quality or any type of reliability; there are already known bugs I will not bother to fix...
It is simply and solely a hobby project. 

In case someone likes the project idea and sees the vision, you are welcome to submit bug fixes or even code improvements and I might (very probably) take a look at it.
 
The purpose of this Project is to create a personalized calculator in Rust implementing the following functions / requirements:

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
- [ ] ans / answer keyword for the result of last calculation
- [ ] Copy to clipboard function
- [x] high ceiling for overflow (-> e.g. multiplying very large numbers with each other) (-> For that I will need to change from f64 to a custom type (probably external crate...))
- [x] high precision (-> depending on user settings, but at least maximum of 1024 decimal digits) (-> See above)
- [ ] Make --output-only cli option with -o
- [ ] Remove all possible Cloning of Floats & Units
- [ ] Improve Precision on "Variables (e.g. Pi)"
- [ ] implement other misc functions: ln, exp, arcsin, arccos, ...
- [ ] live fetching of regularly updated data (e.g. 4 USD to EUR, 0.001 BTC to EUR)
- [ ] working with dates & time (e.g. 14:00 + 5 hours) (-> I need to figure out a good way to do this, as it's not very easy without clipping other features -> 14:00 won't work)
- [ ] custom user-defined functions (add_one(number) = number + 1, etc)
- [ ] saving functions to profile (& settings)
- [ ] user defined settings (precision, etc)
- [ ] Fractional Units (e.g. Sqrt(second))
- [ ] stop frequent crashes and handle errors instead (-> Operation is impossible, might require dimensionless argument, etc)
- [ ] HEX to RGB, etc
- [ ] Fix 1-2^2 (currently gives answer 5 since 1+(-2)^2 is what is being calculated)
- [ ] Fix (-2)-2 != 4 (--> -2 * -2 is what is being calculated; same as above)
- [ ] FIX 5 10 => 60
- [x] Fix -(5+5) (currently crashes)
- [x] Fix 10^31 planck => Is supposedly not unitless


Wow, the more I thought about this, the more this seems like a *HUGE* project. Let's see how long I will last before I will quit... xD

TODOs for later improvement (optional):
4--2 works, but 5-+2 does not, since plus is not considered the sign of a number
