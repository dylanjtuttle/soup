use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::throw_warning;

pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
}

#[derive(PartialEq)]  // Allow TokenNames to be checked for equality
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
    POWER,
    POWEREQ,
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
            '^' => {
                // Initialize a 'to the power of' token
                let mut token = Token {name: TokenName::POWER, lexeme: String::from("^")};

                // Check to see if token is '%=', not just '%'
                if chars[i + 1] == '=' {
                    // Update token information
                    token.name = TokenName::POWEREQ;
                    token.lexeme = String::from("^=");
                    
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

                if is_reserved(chars[i..i + 3].to_vec(), "if") {
                    // Push an 'if' token into the vector of tokens
                    tokens.push(Token {name: TokenName::IF, lexeme: String::from("if")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 2;
                } else if is_reserved(chars[i..i + 4].to_vec(), "int") {
                    // Push an 'int' token into the vector of tokens
                    tokens.push(Token {name: TokenName::INT, lexeme: String::from("int")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 3;
                } else if is_reserved(chars[i..i + 5].to_vec(), "true") {
                    // Push a 'true' token into the vector of tokens
                    tokens.push(Token {name: TokenName::TRUE, lexeme: String::from("true")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(chars[i..i + 5].to_vec(), "bool") {
                    // Push a 'bool' token into the vector of tokens
                    tokens.push(Token {name: TokenName::BOOL, lexeme: String::from("bool")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(chars[i..i + 5].to_vec(), "void") {
                    // Push a 'void' token into the vector of tokens
                    tokens.push(Token {name: TokenName::VOID, lexeme: String::from("void")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(chars[i..i + 5].to_vec(), "else") {
                    // Push a 'else' token into the vector of tokens
                    tokens.push(Token {name: TokenName::ELSE, lexeme: String::from("else")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(chars[i..i + 5].to_vec(), "func") {
                    // Push a 'func' token into the vector of tokens
                    tokens.push(Token {name: TokenName::FUNC, lexeme: String::from("func")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(chars[i..i + 6].to_vec(), "false") {
                    // Push a 'false' token into the vector of tokens
                    tokens.push(Token {name: TokenName::FALSE, lexeme: String::from("false")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 5;
                } else if is_reserved(chars[i..i + 6].to_vec(), "while") {
                    // Push a 'while' token into the vector of tokens
                    tokens.push(Token {name: TokenName::WHILE, lexeme: String::from("while")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 5;
                } else if is_reserved(chars[i..i + 6].to_vec(), "break") {
                    // Push a 'break' token into the vector of tokens
                    tokens.push(Token {name: TokenName::BREAK, lexeme: String::from("break")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 5;
                } else if is_reserved(chars[i..i + 7].to_vec(), "return") {
                    // Push a 'return' token into the vector of tokens
                    tokens.push(Token {name: TokenName::RETURN, lexeme: String::from("return")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 6;
                } else if is_reserved(chars[i..i + 8].to_vec(), "returns") {
                    // Push a 'returns' token into the vector of tokens
                    tokens.push(Token {name: TokenName::RETURNS, lexeme: String::from("returns")});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 7;
                } else {
                    // We have an identifier
                    let id_begin = i;
                    let mut id_char = chars[i];

                    // Loop until we've found a non-id character
                    while is_id_char(id_char) {
                        i += 1;
                        id_char = chars[i];
                    }

                    // Now that we've found the end of the identifier, turn the slice into a string
                    let id_lexeme: String = chars[id_begin..i].iter().collect();

                    // Push an 'identifier' token into the vector of tokens, with the newly created lexeme
                    tokens.push(Token {name: TokenName::ID, lexeme: id_lexeme});
                }

                // Move along to the next char
                i += 1;
            }
            '0'..='9' => {
                // Integer literal, we have to check for multiple digit literals
                let int_lit_begin = i;
                let mut int_lit_char = chars[i];

                // Loop until we've found a non-digit character
                while is_digit(int_lit_char) {
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
            '"' => {
                // We have a string literal
                let mut string_vec = Vec::new();
                i += 1;
                let mut string_char = chars[i];

                // Loop until we find another quotation mark
                while string_char != '"' {
                    string_vec.push(string_char);

                    // If we find a backslash, the user is trying to create an escape character,
                    // so add another one to escape the backslash
                    if string_char == '\\' {
                        string_vec.push('\\');
                    }

                    i += 1;
                    string_char = chars[i];
                }

                // Now that we've found the end of the integer literal, turn the slice into a string
                let string_lexeme: String = string_vec.iter().collect();

                // Push an 'integer literal' token into the vector of tokens, with the newly created lexeme
                tokens.push(Token {name: TokenName::STRLIT, lexeme: string_lexeme});

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

// Returns true if a vector of characters equals a reserved word + a non-id character, and false otherwise
fn is_reserved(actual: Vec<char>, reserved: &str) -> bool {
    let actual_str: String = actual[0..actual.len() - 1].iter().collect();
    let next_char = actual[actual.len() - 1];

    // If the string equals the reserved word and the next character is not alphanumeric or an underscore, this is a reserved word
    actual_str == reserved && !is_id_char(next_char)
}

// Returns true if a character is in a..z, A..Z, or is an underscore, and false otherwise
fn is_id_char(id_char: char) -> bool {
    (id_char >= 'a' && id_char <= 'z') || (id_char >= 'A' && id_char <= 'Z') || (id_char == '_')
}

// Returns true if a character is in 0..9
fn is_digit(digit_char: char) -> bool {
    digit_char >= '0' && digit_char <= '9'
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