use std::io::{self, Write}; //import an input/output library in rust

//BLANCO NAVAREZ PANGILINAN
//A222

#[derive(Debug, Clone)]
enum TokenType {//enumerating token types
    Word,
    Whitespace,
    Operator,
    Number,
    Punctuation,
    EndOfLine,
    Unknown,
}

#[derive(Debug, Clone)]
struct Token { //identify the data type of each variable
    token: String,
    token_type: TokenType,
}

fn classify_token(token: &str) -> TokenType { //this is where we classify the token to its type
    if token.is_empty() {
        return TokenType::EndOfLine;
    }
    if token.chars().all(char::is_alphabetic) {
        TokenType::Word //if its a word
    } else if token.chars().all(char::is_whitespace) {
        TokenType::Whitespace //an empty space
    } else if token.chars().all(|c| "+-*/=<>".contains(c)) {
        TokenType::Operator//an operator
    } else if token.chars().all(char::is_numeric) {
        TokenType::Number // a number
    } else if token.chars().all(|c| c.is_ascii_punctuation()) {
        TokenType::Punctuation// a punctuation 
    } else { // or unknown
        TokenType::Unknown
    }
}

fn tokenize(input: &str) -> Vec<Token> { // tokenizes a string then outputs the vector
    const DELIMITERS: &str = "= \n";
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();

    for c in input.chars() {
        if DELIMITERS.contains(c) {
            if !current_token.is_empty() {
                let token_type = classify_token(&current_token);
                tokens.push(Token {
                    token: current_token.clone(),
                    token_type,
                });
                current_token.clear();
            }
            if c.is_whitespace() {
                tokens.push(Token {
                    token: c.to_string(),
                    token_type: TokenType::Whitespace,
                });
            } else if c.is_ascii_punctuation() {
                tokens.push(Token {
                    token: c.to_string(),
                    token_type: classify_token(&c.to_string()),
                });
            } else if c == '\n' {
                tokens.push(Token {
                    token: c.to_string(),
                    token_type: TokenType::EndOfLine,
                });
            }
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() { 
        // If the current token is not empty, it is classified, added to the tokens vector, and current_token is cleared.
        let token_type = classify_token(&current_token);
        tokens.push(Token {
            token: current_token,
            token_type,
        });
    }

    tokens 
}

fn gran_breakdown(tokens: Vec<Token>) -> Vec<(String, String)> { 
    //it creates a string showing each character in the token
    let mut breakdown_vec = Vec::new();

    for token in tokens {
        let breakdown: String = token.token.chars()
            .map(|c| format!("\"{}\" = ", c))
            .collect::<Vec<String>>()
            .join("")
            .trim_end_matches(" = ")
            .to_string();
        breakdown_vec.push((token.token.clone(), breakdown));
    }

    breakdown_vec
}

fn main() { 
    //main method
    //ask user input and outputs the tokenized output
    print!("Enter the string to tokenize: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        println!("No input provided.");
        return;
    }

    let tokens = tokenize(input);
    println!("Phase 1 Output:");
    for token in &tokens {
        let token_type = match &token.token_type {
            TokenType::Word => "Word",
            TokenType::Whitespace => "Whitespace",
            TokenType::Operator => "Operator",
            TokenType::Number => "Number",
            TokenType::Punctuation => "Punctuation",
            TokenType::EndOfLine => "End of Line",
            TokenType::Unknown => "Unknown",
        };
        println!("Token: \"{}\" - Type: {}", token.token, token_type);
    }

    let breakdown = gran_breakdown(tokens.clone()); // calls gran to output phase 2 
    println!("\nPhase 2 Output (Granular Breakdown):");
    for (token, breakdown) in breakdown {
        println!("Token: \"{}\" -> {}", token, breakdown);
    }
}
