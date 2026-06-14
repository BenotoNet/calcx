use crate::utils;
use super::Token;
use std::collections::HashMap;

fn split_chunks(token_chunks: Vec<&str>) -> Vec<Token> {
    let mut tokens = vec![];
    for rough_token in token_chunks {
    }

    tokens
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
    let mut tokens = vec![];

    let rough_tokens = query.split(" ").collect::<Vec<&str>>();

    tokens = split_chunks(rough_tokens);
    tokens = clean_tokens(tokens);

    // Finally, return the list of tokens
    tokens
}
