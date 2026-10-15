# Project Description
Hey there!
This is my personal project, so don't expect high code quality or any type of reliability; there are already known bugs I will not bother to fix...
It is simply and solely a hobby project. 

Important Links:

- [Link to ToDo-List](./todo.md)
- [AI](./AI.md)

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
    - quit / exit / CTRL-C => exits the interactive session (CTRL-C however does not save history)
    - help => displays this help screen
    - PRECISION:{{new precision}}

To run a query, simply type the query and press enter. 
Example queries to get to know the calculator and its capabilities:

    - 60 + 4.5 * (3+-1)^2 - 9
    - 5 meters / second + 5 kilo meters / hour to miles / day
    - sin((sqrt(log(10, 1000000000)) + root(3, 27)) * 10 degrees)
    - my_var = 50 meters
    - my_var^2

Thank you for using this calculator despite it being unfinished and still rather buggy :)
I would love to have at least one program be any use to anybody other than me...

In case someone likes the project idea and sees the vision, you are welcome to submit bug fixes or even code improvements and I might (very probably) take a look at it.
