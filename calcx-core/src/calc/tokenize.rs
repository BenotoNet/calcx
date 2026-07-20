use crate::utils;
use super::Token;
use crate::calc::num::Num;

pub mod misc_units;

fn split_into_unknowns(query: &str) -> Vec<Token> {
    let splitters = String::from("+*/-%!^()= ,");
    let mut output = vec![];
    let mut partial = String::new();
    for c in query.chars() {
        // Very Hacky, might need to change...
        // if c == '-' {
        //     // When we can, we try to not append the minus as an operation, but rather a sign of a
        //     // number
        //     // -> Leads to being able to do operations like -1+2 while still allowing 5-1 by adding
        //     // Add Operation between two numbers (when the second number is negative)
        //     if partial != "" {
        //         output.push(Token::Unknown(partial.clone()));
        //     }
        //     partial = String::new();
        // }
        if splitters.contains(c) {
            // We want to make 05 into 0 5
            while partial.starts_with("0") {
                output.push(Token::Unknown(String::from("0")));
                partial.remove(0);
            }
            if partial != "" {
                output.push(Token::Unknown(partial.clone()));
            }
            if c != ' ' {
                output.push(Token::Unknown(c.to_string()));
            }
            partial = String::new();
        } else {
            partial += &c.to_string();
        }
    }
    // We want to make 05 into 0 5
    while partial.starts_with("0") {
        output.push(Token::Unknown(String::from("0")));
        partial.remove(0);
    }
    if partial != "" {
        output.push(Token::Unknown(partial));
    }

    return output;
}

fn is_keyword(unknown_token: &str) -> bool {
    // checking if the token is one of the reserved keywords, then it is a keyword, otherwise, it's
    // a variable!

    // Check if it is a unit:
    match misc_units::unit_to_num(unknown_token) {
        Ok(_) => return true,
        _ => {},
    };

    // Check if it's a reserved keyword:
    let temp_calc = crate::calc::Calc::new(1);
    match temp_calc.eval_keyword(unknown_token, Ok(&Num::unitless("1.0")), Ok(&Num::unitless("1.0"))) {
        Err(v) => {
            return !(v.as_str() == "Unknown Keyword or not enough Arguments! (Maybe not implemented yet?)")
        },
        _ => true,
    }
}

fn categorize(tokens: Vec<Token>) -> Vec<Token> {
    tokens.iter().map(|token| {
        match token {
            Token::Unknown(token_string) => {
                let token_str = token_string.as_str();
                if utils::is_number(token_str) {return Token::Number(Num::unitless(token_str));}
                match token_str {
                    "+" => {Token::Add}
                    "-" => {Token::Sub}
                    "*" => {Token::Mul}
                    "/" => {Token::Div}
                    "%" => {Token::Mod}
                    "(" => {Token::LBrac}
                    ")" => {Token::RBrac}
                    "^" => {Token::Pow}
                    "=" => {Token::Assign}
                    "," => {Token::Seperator}
                    _ => {
                        // No Operator Matches
                        // Test for Keyword:
                        if is_keyword(token_string) {
                            return Token::Keyword(token_string.clone());
                        }
                        if super::funcs::is_function(token_string) {
                            return Token::Func(token_string.clone())
                        }
                        else {
                            return Token::Var(token_string.clone());
                        }
                    }
                }
            },
            _ => {panic!()}
        }
    }).collect()
}

fn match_units(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut index = 0; 
    while index < tokens.len() {
        match tokens.get(index) {
            // Checking the keywords for scientific notion / powers of ten / additional units like
            // volts, farad, etc
            Some(Token::Keyword(var)) => {
                match misc_units::unit_to_num(var.as_str()) {
                    Ok(num) => {
                        tokens[index] = Token::RBrac; 
                        tokens.insert(index, Token::Number(num));
                        tokens.insert(index, Token::LBrac);
                    },
                    _ => {},
                }
            }
            _ => {},
        }
        index += 1;
    }
    return tokens;
}

fn clean(tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens;
    let mut index: usize = 0;

    while index < tokens.len() {
        // If current index and next one are numbers
        let current = &tokens[index];
        match (current, tokens.get(index + 1)) {
            (Token::Number(num1), Some(Token::Number(num2))) => {
                // Check if the numbers are both unitless. If they are, we can combine, otherwise,
                // we have to multiply
                if num1.is_unitless() && num2.is_unitless() {
                    // When we have a negative number behind a positive one, we do not combine, but add
                    // "Add" Operation between them e.g. 4-2 => 4 + -2
                    if num2.get_quant() < 0.0 {
                        tokens.insert(index+1, Token::Add);
                        index = 0
                    }
                    
                    else {
                        // Combine the numbers by putting strings right next to each other. (If Unitless)
                        // e.g. 5 5 => 55
                        tokens[index] = Token::Number(num1.append(&num2));
                        tokens.remove(index+1);
                        index = 0;
                    }
                }

                // Numbers are not unitless, therefore multiply (Add Multiplication Token between
                // them)
                else {
                    tokens.insert(index+1, Token::Mul)
                }
            },
            // When we have a number after a closing bracket, it is implied that we want to multiply
            // (only if number is positive)
            (Token::RBrac, Some(Token::Number(num))) => {
                if num.get_quant() > 0.0 {
                    tokens.insert(index+1, Token::Mul);
                    index = 0;
                }
                // Otherwise, we are trying to subtract here, therefore add an "Add" Operation
                else {
                    tokens.insert(index+1, Token::Add);
                    index = 0;
                }
            }
            // Replace double Signs with a single one
            (Token::Sub, Some(Token::Sub))|(Token::Add, Some(Token::Add)) => {
                tokens.remove(index);
                tokens.remove(index);
                tokens.insert(index, Token::Add);
                index = 0;
            }
            (Token::Sub, Some(Token::Add))|(Token::Add, Some(Token::Sub)) => {
                tokens.remove(index);
                tokens.remove(index);
                tokens.insert(index, Token::Sub);
                index = 0;
            }
            // If we have two variables right next to each other, insert a multiplication, OR if we have
            // a number and then a var or the other way around
            (Token::Var(_), Some(Token::Var(_)))
                |(Token::Var(_), Some(Token::Number(_)))
                |(Token::Number(_), Some(Token::Var(_))) => {
                tokens.insert(index+1, Token::Mul);
            }

            // Adding Brackets around "ans" keyword to allow automatic multiplication & inserting into
            // functions, etc
            (Token::Keyword(key), _) => {
                if key == "ans" {
                    tokens.insert(index, Token::LBrac);
                    tokens.insert(index+2, Token::RBrac);
                    index += 1;
                }
                index += 1;
            }

            // If we have a function, we want to add brackets around it
            (Token::Func(_), _) => {
                // We have a function and want to turn it from func(a, b) into (func(a, b))
                // But save the original index (if we have a double number in the arguments, for
                // example)
                let original_index = index;

                // We count brackets
                let mut brackets = 0;

                // we add the starting brackets
                tokens.insert(index, Token::LBrac);
                index += 1;
                while index < tokens.len() {
                    match tokens.get(index) {
                        // Increase counter on LBrackets
                        Some(Token::LBrac) => {brackets += 1; index += 1;},
                        // and decrease on Rbrackets
                        Some(Token::RBrac) => {brackets -= 1; 
                            // If we finished (last bracket)
                            if brackets <= 0 {
                                // We break the loop and are done (insert the last RBracket)
                                tokens.insert(index, Token::RBrac);
                                index = original_index+2; // The plus two comes from the original
                                                          // bracket and function being where the
                                                          // index points
                                break
                            }
                            index += 1;
                        }
                        _ => {index += 1},
                    }
                }
            }
            _ => {index += 1;}
        }
    }

    return tokens;
}

// Tokenize (Parse a given Query into Tokens)
pub fn tokenize(query: &str) -> Vec<Token> {
    // First, we split the query into semantic blocks (uncategorized) then, we categorize each block
    // into a token and finally clean up the list of tokens (combine two adjecent Numbers)
    let mut tokens = split_into_unknowns(query);
    tokens = categorize(tokens);
    tokens = match_units(tokens);
    tokens = clean(tokens);

    // Finally, return the list of tokens
    tokens
}
