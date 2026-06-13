use super::Token;

const OPERATORS: [char; 8] = ['*', '^', '+', '-', '(', ')', '/', '%'];

fn clean_tokens(tokens: Vec<Token>) -> Vec<Token> {
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
            Token::Expression(ex) => {
                // Check if Token is empty -> Remove Token
                if ex == "" {
                    tokens.remove(index);
                    index = 0;
                }

                else {
                    index += 1;
                }
            }
        }
    }
    tokens
}

fn split_chunks(token_chunks: Vec<&str>) -> Vec<Token> {
    let tokens = vec![];
    for rough_token in token_chunks {
        let mut part_token = String::new();
        for c in rough_token.chars() {
            if OPERATORS.contains(&c) {
                // If it can be parsed as f64, append it as num, otherwise expression
                match part_token.parse::<f64>() {
                    Ok(num) => {tokens.push(Token::Number(num));},
                    Err(_) => {tokens.push(Token::Expression(part_token));}
                }

                // Append the Operator as expression
                tokens.push(Token::Expression(String::from(c)));

                // Reset the appending token
                part_token = String::new();

            } else {
                // Append current character because the token is not done yet
                part_token += &c.to_string();
            }
        }
        // Append the last bit as expression or number
        match part_token.parse::<f64>() {
            Ok(num) => tokens.push(Token::Number(num)),
            Err(_) => tokens.push(Token::Expression(part_token)),
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
