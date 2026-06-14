use crate::utils;
use super::Token;
use std::collections::HashMap;

fn split_into_unknowns(query: &str) -> Vec<Token> {
    let splitters = String::from("+-*/%!^()= ");
    let mut output = vec![];
    let mut partial = String::new();
    for c in query.chars() {
        if splitters.contains(c) {
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
    if partial != "" {
        output.push(Token::Unknown(partial));
    }

    return output;
}

fn is_keyword(unknown: &str) -> bool {
    // TODO: Later, actually add the logic for handling keywords differently from e.g. variables
    true
}

fn categorize(tokens: Vec<Token>) -> Vec<Token> {
    tokens.iter().map(|token| {
        match token {
            Token::Unknown(token_string) => {
                let STR = token_string.as_str();
                if utils::is_number(STR) {return Token::Number(STR.parse().unwrap());}
                match STR {
                    "+" => {Token::Add}
                    "-" => {Token::Sub}
                    "*" => {Token::Mul}
                    "/" => {Token::Div}
                    "%" => {Token::Mod}
                    "(" => {Token::LBrac}
                    ")" => {Token::RBrac}
                    "^" => {Token::Pow}
                    "=" => {Token::Assign}
                    _ => {
                        // No Operator Matches
                        // Test for Keyword:
                        if is_keyword(token_string) {
                            return Token::Keyword(token_string.clone());
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

fn clean(tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens;
    let mut index: usize = 0;

    while index < tokens.len() {
        // If current index and next one are numbers, combine them
        let current = &tokens[index];
        match (current, tokens.get(index + 1)) {
            (Token::Number(num1), Some(Token::Number(num2))) => {
                tokens[index] = Token::Number(format!{"{num1}{num2}"}.parse().unwrap());
                tokens.remove(index+1);
                index = 0;
            },
            _ => {index += 1;}
        }
    }

    return tokens;
}

fn clean_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
    // Cleaning up: Edge cases
    let mut index = 0;
    while index < tokens.len() {
        let current_token = &tokens[index];

        match current_token {
            Token::Number(num1) => {
                match tokens.get(index+1) {
                    Some(Token::Number(num2)) => {
                        // at current index and next, there are two numbers, therefore combine
                        // them:
                        let combined_string = format!{"{num1}{num2}"};
                        // Unwrap should be fine here...
                        tokens[index] = Token::Number(combined_string.parse::<f64>().unwrap());

                        tokens.remove(index+1);
                        index = 0;
                    },
                    // Advance the global index
                    _ => index += 1,
                }
            }
            Token::Keyword(ex) => {
                // Check if Token is empty -> Remove Token
                if ex == "" {
                    tokens.remove(index);
                    index = 0;
                }

                else {
                    index += 1;
                }
            }
            _ => {index += 1}
        }
    }
    tokens
}

// Tokenize (Parse a given Query into Tokens)
pub fn tokenize(query: &str) -> Vec<Token> {
    let tokens = clean(categorize(split_into_unknowns(query)));

    // tokens = split_chunks(rough_tokens);
    // tokens = clean_tokens(tokens);

    // Finally, return the list of tokens
    tokens
}
