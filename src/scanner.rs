use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::throw_warning;

pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
}

pub enum TokenName {
    ID,
    STRLIT,
    INTLIT,
    TRUE,
    FALSE,
    INT,
    BOOL,
    VOID,
    IF,
    ELSE,
    WHILE,
    BREAK,
    RETURN,
    FUNC,
    RETURNS,
    PLUS,
    PLUSEQ,
    MINUS,
    MINUSEQ,
    MULT,
    MULTEQ,
    DIV,
    DIVEQ,
    MOD,
    MODEQ,
    LT,
    GT,
    LEQ,
    GEQ,
    ASSIGN,
    EQ,
    NEQ,
    NOT,
    AND,
    OR,
    OPENPAR,
    CLOSEPAR,
    OPENBRACE,
    CLOSEBRACE,
    SEMICOLON,
    COMMA
}

pub fn scanner(code_file: &str) -> Vec<Token> {
    // Get a vector of characters from the file
    let chars = get_chars(code_file);

    let mut tokens = Vec::new();

    // Loop through the characters
    let mut i = 0;
    while i < chars.len() {
        // Get the current character
        let ch = chars[i];

        // Let's check our cases:

        match ch {
            ' ' | '\t' | '\n' => {
                // Ignore whitespace
                i += 1;
            }
            '(' => {
                // Push an 'open parenthesis' token into the vector of tokens
                tokens.push(Token {name: TokenName::OPENPAR, lexeme: String::from("(")});

                // Move along to the next char
                i += 1;
            }
            ')' => {
                // Push a 'close parenthesis' token into the vector of tokens
                tokens.push(Token {name: TokenName::CLOSEPAR, lexeme: String::from(")")});

                // Move along to the next char
                i += 1;
            }
            '{' => {
                // Push an 'open brace' token into the vector of tokens
                tokens.push(Token {name: TokenName::OPENBRACE, lexeme: String::from("{")});

                // Move along to the next char
                i += 1;
            }
            '}' => {
                // Push an 'close brace' token into the vector of tokens
                tokens.push(Token {name: TokenName::CLOSEBRACE, lexeme: String::from("}")});

                // Move along to the next char
                i += 1;
            }
            ';' => {
                // Push a 'semicolon' token into the vector of tokens
                tokens.push(Token {name: TokenName::SEMICOLON, lexeme: String::from(";")});

                // Move along to the next char
                i += 1;
            }
            ',' => {
                // Push a 'comma' token into the vector of tokens
                tokens.push(Token {name: TokenName::COMMA, lexeme: String::from(",")});

                // Move along to the next char
                i += 1;
            }
            '+' => {
                // Initialize a 'plus' token
                let mut token = Token {name: TokenName::PLUS, lexeme: String::from("+")};

                // Check to see if token is '+=', not just '+'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::PLUSEQ;
                    token.lexeme = String::from("+=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '-' => {
                // Initialize a 'minus' token
                let mut token = Token {name: TokenName::MINUS, lexeme: String::from("-")};

                // Check to see if token is '-=', not just '-'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::MINUSEQ;
                    token.lexeme = String::from("-=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '*' => {
                // Initialize a 'multiplication' token
                let mut token = Token {name: TokenName::MULT, lexeme: String::from("*")};

                // Check to see if token is '*=', not just '*'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::MULTEQ;
                    token.lexeme = String::from("*=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '/' => {
                // Initialize a 'division' token
                let mut token = Token {name: TokenName::DIV, lexeme: String::from("/")};

                // Check to see if token is '/=', not just '/'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::DIVEQ;
                    token.lexeme = String::from("/=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 2;

                    // Push the token into the vector of tokens
                    tokens.push(token);
                } else if chars[i + 1] == '/' {
                    // We have a comment, ignore until a newline character
                    println!("Comment found!");

                    // Loop until we find a newline character
                    let mut comment_char = chars[i];
                    while comment_char != '\n' {
                        i += 1;
                        comment_char = chars[i];
                    }

                } else {
                    // We just have a regular division token
                    // Push the token into the vector of tokens
                    tokens.push(token);

                    // Move along to the next char
                    i += 1;
                }
            }
            '%' => {
                // Initialize a 'modulus' token
                let mut token = Token {name: TokenName::MOD, lexeme: String::from("%")};

                // Check to see if token is '%=', not just '%'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::MODEQ;
                    token.lexeme = String::from("%=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '<' => {
                // Initialize a 'less than' token
                let mut token = Token {name: TokenName::LT, lexeme: String::from("<")};

                // Check to see if token is '<=', not just '<'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::LEQ;
                    token.lexeme = String::from("<=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '>' => {
                // Initialize a 'greater than' token
                let mut token = Token {name: TokenName::GT, lexeme: String::from(">")};

                // Check to see if token is '>=', not just '>'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::GEQ;
                    token.lexeme = String::from(">=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '=' => {
                // Initialize an 'assignment' token
                let mut token = Token {name: TokenName::ASSIGN, lexeme: String::from("=")};

                // Check to see if token is '==', not just '='
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::EQ;
                    token.lexeme = String::from("==");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '!' => {
                // Initialize a 'not' token
                let mut token = Token {name: TokenName::NOT, lexeme: String::from("!")};

                // Check to see if token is '!=', not just '!'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::NEQ;
                    token.lexeme = String::from("!=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 1;
                }

                // Push the token into the vector of tokens
                tokens.push(token);

                // Move along to the next char
                i += 1;
            }
            '&' => {
                // Check to see if token is '&&'
                if chars[i + 1] == '&' {
                    // Push an 'and' token into the vector of tokens
                    tokens.push(Token {name: TokenName::AND, lexeme: String::from("&&")});

                    // Skip the next char, since it is a part of our current token
                    i += 1;
                } else {
                    // Otherwise, this is an invalid token
                    throw_warning("Unrecognized token");
                }

                // Move along to the next char
                i += 1;
            }
            '|' => {
                // Check to see if token is '||'
                if chars[i + 1] == '|' {
                    // Push an 'or' token into the vector of tokens
                    tokens.push(Token {name: TokenName::OR, lexeme: String::from("||")});

                    // Skip the next char, since it is a part of our current token
                    i += 1;
                } else {
                    // Otherwise, this is an invalid token
                    throw_warning("OOO Unrecognized token");
                }

                // Move along to the next char
                i += 1;
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                // Possible identifier, but we have to check for reserved words first
            }
            '0'..='9' => {
                // Integer literal, we have to check for multiple digit literals
                let int_lit_begin = i;
                let mut int_lit_char = chars[i];

                // Loop until we've found a 
                while int_lit_char >= '0' && int_lit_char <= '9' {
                    i += 1;
                    int_lit_char = chars[i];
                }

                // Now that we've found the end of the integer literal, turn the slice into a string
                let int_lit_lexeme: String = chars[int_lit_begin..i].iter().collect();

                // Push an 'integer literal' token into the vector of tokens, with the newly created lexeme
                tokens.push(Token {name: TokenName::INTLIT, lexeme: int_lit_lexeme});

                // Move along to the next char
                i += 1;
            }
            _ => {
                // If we haven't matched any tokens, throw a warning
                throw_warning("Unrecognized token");
                i += 1;
            }
        }
    }

    // Return vector of tokens
    tokens
}

// Loops through a file and returns a vector containing each of its characters
fn get_chars(file: &str) -> Vec<char> {
    // Initialize an empty vector to hold characters
    let mut char_vec = Vec::new();

    if let Ok(lines) = read_lines(file) {
        // Loop through the lines of the file, storing each line as a string
        for line in lines {
            if let Ok(line_str) = line {
                // Loop through each character in the line
                for ch in line_str.chars() {
                    // Add the character to the vector
                    char_vec.push(ch);
                }

                // Make sure a newline character is included in the vector at the end of each line
                char_vec.push('\n');
            }
        }
    }

    // Return the vector
    char_vec
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    // Wrap the output in a Result to allow for error checking
    Ok(io::BufReader::new(file).lines())
}