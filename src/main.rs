use std::io::{self, Write}; //import an input/output library in rust
use std::collections::HashMap; // To count the occurrences of token types

//BLANCO NAVAREZ PANGILINAN
//A222

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenType {//enumerating token types
    Word,
    Whitespace,
    Tab,       
    Operator,
    Number,
    Punctuation,
    Symbol,
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
    } else if token == "\t" {
        TokenType::Tab // Separate tab characters
    } else if token.chars().all(char::is_whitespace) {
        TokenType::Whitespace // Regular spaces
    } else if token.chars().all(|c| "+-*/=<>".contains(c)) {
        TokenType::Operator//an operator
    } else if token.chars().all(char::is_numeric) {
        TokenType::Number // a number
    } else if token.chars().all(char::is_alphanumeric) {
        TokenType::Symbol // Added symbol identification logic
    } else if token.chars().all(|c| c.is_ascii_punctuation()) {
        TokenType::Punctuation// a punctuation 
    } else { // or unknown
        TokenType::Unknown
    }
}

fn tokenize(input: &str) -> Vec<Token> { //tokenizes a string then outputs the vector
    const DELIMITERS: &str = "= \n"; //delimiters we don't want to include as tokens
    let mut tokens: Vec<Token> = Vec::new(); //to store tokens in vectors
    let mut current_token = String::new(); //temporary storage for building tokens

    for c in input.chars() {
        if DELIMITERS.contains(c) { 
            if !current_token.is_empty() { 
                //if not empty, classify tokens
                let token_type = classify_token(&current_token);
                tokens.push(Token {
                    token: current_token.clone(),
                    token_type,
                });
                current_token.clear(); //clear current token after storing
            }
            
            if c == '\n' { // Handle end-of-line characters
                tokens.push(Token {
                    token: c.to_string(),
                    token_type: TokenType::EndOfLine,
                });
            }
            //if token is "=", skip
            continue;
        }

        if c == '\t' { //separating tabs with whitespaces
            if !current_token.is_empty() {
                let token_type = classify_token(&current_token);
                tokens.push(Token {
                    token: current_token.clone(),
                    token_type,
                });
                current_token.clear();
            }
            tokens.push(Token {
                token: c.to_string(),
                token_type: TokenType::Tab,
            });
            continue;
        }

        if c.is_ascii_punctuation() && !current_token.is_empty() {
            //if a punctuation succeeds a token, store token first then the punctuation
            let token_type = classify_token(&current_token);
            tokens.push(Token {
                token: current_token.clone(),
                token_type,
            });
            current_token.clear(); //clearing token for punctuation
        }

        if c.is_ascii_punctuation() {
            //for tokenizing punctuations
            tokens.push(Token {
                token: c.to_string(),
                token_type: classify_token(&c.to_string()), //classifying punctuations
            });
        } else {
            current_token.push(c); //otherwise, keep building the current token
        }
    }

    if !current_token.is_empty() { 
        //if the current token is not empty, classify it and add it to the tokens vector
        let token_type = classify_token(&current_token);
        tokens.push(Token {
            token: current_token,
            token_type,
        });
    }

    tokens 
}

//function to count the occurrences of each token type
fn count_token_types(tokens: &[Token]) -> HashMap<TokenType, usize> {
    let mut token_counts: HashMap<TokenType, usize> = HashMap::new();

    //iterate through the list of tokens and count each type
    for token in tokens {
        *token_counts.entry(token.token_type.clone()).or_insert(0) += 1;
    }

    token_counts //return the counts for each token type
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

    loop {
        //display the menu to the user
        println!("\n1. Tokenize string");
        println!("2. Exit program");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); //flush stdout to ensure prompt is shown

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap(); //read user input for menu choice
        let choice = choice.trim();

        //handle user choice using match
        match choice {
            "1" => { // Tokenize the string
                print!("Enter the string to tokenize: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.is_empty() {
                    println!("No input provided.");
                    continue;
                }

                let tokens = tokenize(input);
                println!("Phase 1 Output:");
                for token in &tokens {
                    let token_type = match &token.token_type {
                        TokenType::Word => "Word",
                        TokenType::Whitespace => "Whitespace",
                        TokenType::Tab => "Tab", //handle Tab type
                        TokenType::Operator => "Operator",
                        TokenType::Number => "Number",
                        TokenType::Punctuation => "Punctuation",
                        TokenType::Symbol => "Symbol", //added Symbol handling
                        TokenType::EndOfLine => "End of Line",
                        TokenType::Unknown => "Unknown",
                    };
                    println!("Token: \"{}\" - Type: {}", token.token, token_type);
                }

                //count how many of each token type occurred
                let token_counts = count_token_types(&tokens);
                println!("\nToken Type Counts:");
                for (token_type, count) in token_counts {
                    let token_type_str = match token_type {
                        TokenType::Word => "Word",
                        TokenType::Whitespace => "Whitespace",
                        TokenType::Tab => "Tab", //handle Tab counting
                        TokenType::Operator => "Operator",
                        TokenType::Number => "Number",
                        TokenType::Punctuation => "Punctuation",
                        TokenType::Symbol => "Symbol",
                        TokenType::EndOfLine => "End of Line",
                        TokenType::Unknown => "Unknown",
                    };
                    println!("{}: {}", token_type_str, count);
                }

                //granular breakdown of tokens (Phase 2)
                let breakdown = gran_breakdown(tokens.clone()); // calls gran to output phase 2 
                println!("\nPhase 2 Output (Granular Breakdown):");
                for (token, breakdown) in breakdown {
                    println!("Token: \"{}\" -> {}", token, breakdown);
                }
            },
            "2" => { //exit the program
                println!("Exiting the program.");
                break;
            },
            _ => { //handle invalid input
                println!("Invalid choice. Please enter 1 or 2.");
            },
        }
    }
}