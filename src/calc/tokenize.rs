use crate::utils;
use super::Token;
use crate::Num;

fn split_into_unknowns(query: &str) -> Vec<Token> {
    let splitters = String::from("+*/%!^()= ");
    let mut output = vec![];
    let mut partial = String::new();
    for c in query.chars() {
        // Very Hacky, might need to change...
        if c == '-' {
            // When we can, we try to not append the minus as an operation, but rather a sign of a
            // number
            // -> Leads to being able to do operations like -1+2 while still allowing 5-1 by adding
            // Add Operation between two numbers (when the second number is negative)
            if partial != "" {
                output.push(Token::Unknown(partial.clone()));
            }
            partial = String::new();
        }
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

#[allow(unused)]
fn is_keyword(unknown_token: &str) -> bool {
    // TODO: Later, actually add the logic for handling keywords differently from e.g. variables
    true
}

fn categorize(tokens: Vec<Token>) -> Vec<Token> {
    tokens.iter().map(|token| {
        match token {
            Token::Unknown(token_string) => {
                let token_str = token_string.as_str();
                if utils::is_number(token_str) {return Token::Number(Num::unitless(token_str.parse::<f64>().unwrap()));}
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

fn match_keywords_units(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut index = 0;
    while index < tokens.len() {
        match tokens.get(index) {
            // Checking unit keywords and replace them with unit-number
            Some(Token::Keyword(var)) => {
                match var.as_str() {
                    "metre"|"meter"|"meters"|"metres" => {tokens[index] = Token::Number(Num::new(1.0, vec![('m', 1)]));}
                    "second"|"seconds"|"secs"|"sec" => {tokens[index] = Token::Number(Num::new(1.0, vec![('s', 1)]));}
                    "gram"|"grams" => {tokens[index] = Token::Number(Num::new(1.0, vec![('g', 1)]));}
                    "ampere"|"amperes" => {tokens[index] = Token::Number(Num::new(1.0, vec![('a', 1)]));}
                    "kelvin" => {tokens[index] = Token::Number(Num::new(1.0, vec![('k', 1)]));}
                    "candela"|"candelas" => {tokens[index] = Token::Number(Num::new(1.0, vec![('c', 1)]));}
                    _ => {}
                }
            }
            _ => {}
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
                // When we have a negative number behind a positive one, we do not combine, but add
                // "Add" Operation between them e.g. 4-2 => 4 + -2
                if num2.get_quant() < 0.0 {
                    tokens.insert(index+1, Token::Add);
                    index = 0
                }

                // Check if the numbers are both unitless. If they are, we can combine, otherwise,
                // we have to multiply
                else if num1.is_unitless() && num2.is_unitless() {
                    // Combine the numbers by putting strings right next to each other. (If Unitless)
                    // e.g. 5 5 => 55
                    tokens[index] = Token::Number(num1.append(num2));
                    tokens.remove(index+1);
                    index = 0;
                }

                // Numbers are not unitless, therefore multiply (Add Multiplication Token between
                // them)
                else {
                    tokens.insert(index+1, Token::Mul)
                }
            },
            (Token::RBrac, Some(Token::Number(num))) => {
                if !num.is_unitless() {tokens.insert(index+1, Token::Mul)}
                index = 0;
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
    tokens = match_keywords_units(tokens);
    tokens = clean(tokens);

    // Finally, return the list of tokens
    tokens
}
