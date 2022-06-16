use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::throw_warning;

pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
    pub line_num: i32,
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
    MAIN,
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
    COMMA,
    EOF
}

pub fn scanner(code_file: &str) -> Vec<Token> {
    // Get a vector of characters from the file
    let chars = get_chars(code_file);

    let mut tokens = Vec::new();

    // Loop through the characters
    let mut i = 0;
    while i < chars.len() {
        // Get the current character
        let ch = chars[i].char_val;
        let line_num = chars[i].line_num;

        // Let's check our cases:

        match ch {
            ' ' | '\t' | '\n' => {
                // Ignore whitespace
                i += 1;
            }
            '(' => {
                // Push an 'open parenthesis' token into the vector of tokens
                tokens.push(Token {name: TokenName::OPENPAR, lexeme: String::from("("), line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            ')' => {
                // Push a 'close parenthesis' token into the vector of tokens
                tokens.push(Token {name: TokenName::CLOSEPAR, lexeme: String::from(")"), line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            '{' => {
                // Push an 'open brace' token into the vector of tokens
                tokens.push(Token {name: TokenName::OPENBRACE, lexeme: String::from("{"), line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            '}' => {
                // Push an 'close brace' token into the vector of tokens
                tokens.push(Token {name: TokenName::CLOSEBRACE, lexeme: String::from("}"), line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            ';' => {
                // Push a 'semicolon' token into the vector of tokens
                tokens.push(Token {name: TokenName::SEMICOLON, lexeme: String::from(";"), line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            ',' => {
                // Push a 'comma' token into the vector of tokens
                tokens.push(Token {name: TokenName::COMMA, lexeme: String::from(","), line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            '+' => {
                // Initialize a 'plus' token
                let mut token = Token {name: TokenName::PLUS, lexeme: String::from("+"), line_num: line_num};

                // Check to see if token is '+=', not just '+'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::MINUS, lexeme: String::from("-"), line_num: line_num};

                // Check to see if token is '-=', not just '-'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::MULT, lexeme: String::from("*"), line_num: line_num};

                // Check to see if token is '*=', not just '*'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::DIV, lexeme: String::from("/"), line_num: line_num};

                // Check to see if token is '/=', not just '/'
                if chars[i + 1].char_val == '=' {
                    // Update token information
                    token.name = TokenName::DIVEQ;
                    token.lexeme = String::from("/=");
                    
                    // Skip the next char, since it is a part of our current token
                    i += 2;

                    // Push the token into the vector of tokens
                    tokens.push(token);
                } else if chars[i + 1].char_val == '/' {
                    // We have a comment, ignore until a newline character
                    println!("Comment found!");

                    // Loop until we find a newline character
                    let mut comment_char = chars[i].char_val;
                    while comment_char != '\n' {
                        i += 1;
                        comment_char = chars[i].char_val;
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
                let mut token = Token {name: TokenName::MOD, lexeme: String::from("%"), line_num: line_num};

                // Check to see if token is '%=', not just '%'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::POWER, lexeme: String::from("^"), line_num: line_num};

                // Check to see if token is '%=', not just '%'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::LT, lexeme: String::from("<"), line_num: line_num};

                // Check to see if token is '<=', not just '<'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::GT, lexeme: String::from(">"), line_num: line_num};

                // Check to see if token is '>=', not just '>'
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::ASSIGN, lexeme: String::from("="), line_num: line_num};

                // Check to see if token is '==', not just '='
                if chars[i + 1].char_val == '=' {
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
                let mut token = Token {name: TokenName::NOT, lexeme: String::from("!"), line_num: line_num};

                // Check to see if token is '!=', not just '!'
                if chars[i + 1].char_val == '=' {
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
                if chars[i + 1].char_val == '&' {
                    // Push an 'and' token into the vector of tokens
                    tokens.push(Token {name: TokenName::AND, lexeme: String::from("&&"), line_num: line_num});

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
                if chars[i + 1].char_val == '|' {
                    // Push an 'or' token into the vector of tokens
                    tokens.push(Token {name: TokenName::OR, lexeme: String::from("||"), line_num: line_num});

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

                if is_reserved(vec![chars[i].char_val,
                                            chars[i + 1].char_val,
                                            chars[i + 2].char_val], "if") {
                    // Push an 'if' token into the vector of tokens
                    tokens.push(Token {name: TokenName::IF, lexeme: String::from("if"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 2;
                } else if is_reserved(vec![chars[i].char_val,
                                                   chars[i + 1].char_val,
                                                   chars[i + 2].char_val,
                                                   chars[i + 3].char_val], "int") {
                    // Push an 'int' token into the vector of tokens
                    tokens.push(Token {name: TokenName::INT, lexeme: String::from("int"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 3;
                } else if is_reserved(vec![chars[i].char_val,
                                                   chars[i + 1].char_val,
                                                   chars[i + 2].char_val,
                                                   chars[i + 3].char_val,
                                                   chars[i + 4].char_val], "true") {
                    // Push a 'true' token into the vector of tokens
                    tokens.push(Token {name: TokenName::TRUE, lexeme: String::from("true"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val], "bool") {
                    // Push a 'bool' token into the vector of tokens
                    tokens.push(Token {name: TokenName::BOOL, lexeme: String::from("bool"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val], "void") {
                    // Push a 'void' token into the vector of tokens
                    tokens.push(Token {name: TokenName::VOID, lexeme: String::from("void"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val], "else") {
                    // Push a 'else' token into the vector of tokens
                    tokens.push(Token {name: TokenName::ELSE, lexeme: String::from("else"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val], "func") {
                    // Push a 'func' token into the vector of tokens
                    tokens.push(Token {name: TokenName::FUNC, lexeme: String::from("func"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val], "main") {
                    // Push a 'func' token into the vector of tokens
                    tokens.push(Token {name: TokenName::MAIN, lexeme: String::from("main"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 4;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val,
                                                    chars[i + 5].char_val], "false") {
                    // Push a 'false' token into the vector of tokens
                    tokens.push(Token {name: TokenName::FALSE, lexeme: String::from("false"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 5;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val,
                                                    chars[i + 5].char_val], "while") {
                    // Push a 'while' token into the vector of tokens
                    tokens.push(Token {name: TokenName::WHILE, lexeme: String::from("while"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 5;
                } else if is_reserved(vec![chars[i].char_val,
                                        chars[i + 1].char_val,
                                        chars[i + 2].char_val,
                                        chars[i + 3].char_val,
                                        chars[i + 4].char_val,
                                        chars[i + 5].char_val], "break") {
                    // Push a 'break' token into the vector of tokens
                    tokens.push(Token {name: TokenName::BREAK, lexeme: String::from("break"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 5;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val,
                                                    chars[i + 5].char_val,
                                                    chars[i + 6].char_val], "return") {
                    // Push a 'return' token into the vector of tokens
                    tokens.push(Token {name: TokenName::RETURN, lexeme: String::from("return"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 6;
                } else if is_reserved(vec![chars[i].char_val,
                                                    chars[i + 1].char_val,
                                                    chars[i + 2].char_val,
                                                    chars[i + 3].char_val,
                                                    chars[i + 4].char_val,
                                                    chars[i + 5].char_val,
                                                    chars[i + 6].char_val,
                                                    chars[i + 7].char_val], "returns") {
                    // Push a 'returns' token into the vector of tokens
                    tokens.push(Token {name: TokenName::RETURNS, lexeme: String::from("returns"), line_num: line_num});

                    // Skip the chars comprising the reserved word, since they're a part of our current token
                    i += 7;
                } else {
                    // We have an identifier
                    let mut id_char = chars[i].char_val;
                    let mut id_vec = Vec::new();
                    id_vec.push(id_char);

                    // Loop until we've found a non-id character
                    while is_id_char(id_char) {
                        i += 1;
                        id_char = chars[i].char_val;
                        id_vec.push(id_char);
                    }

                    // Now that we've found the end of the identifier, turn the slice into a string
                    let id_lexeme: String = id_vec[0..id_vec.len() - 1].iter().collect();

                    // Push an 'identifier' token into the vector of tokens, with the newly created lexeme
                    tokens.push(Token {name: TokenName::ID, lexeme: id_lexeme, line_num: line_num});
                }

                // Move along to the next char
                i += 1;
            }
            '0'..='9' => {
                // Integer literal, we have to check for multiple digit literals
                let mut int_lit_char = chars[i].char_val;
                let mut int_lit_vec = Vec::new();
                int_lit_vec.push(int_lit_char);

                // Loop until we've found a non-digit character
                while is_digit(int_lit_char) {
                    i += 1;
                    int_lit_char = chars[i].char_val;
                    int_lit_vec.push(int_lit_char);
                }

                // Now that we've found the end of the integer literal, turn the slice into a string
                let int_lit_lexeme: String = int_lit_vec[0..int_lit_vec.len() - 1].iter().collect();

                // Push an 'integer literal' token into the vector of tokens, with the newly created lexeme
                tokens.push(Token {name: TokenName::INTLIT, lexeme: int_lit_lexeme, line_num: line_num});

                // Move along to the next char
                i += 1;
            }
            '"' => {
                // We have a string literal
                let mut string_vec = Vec::new();
                i += 1;
                let mut string_char = chars[i].char_val;

                // Loop until we find another quotation mark
                while string_char != '"' {
                    string_vec.push(string_char);

                    // If we find a backslash, the user is trying to create an escape character,
                    // so add another one to escape the backslash
                    if string_char == '\\' {
                        string_vec.push('\\');
                    }

                    i += 1;
                    string_char = chars[i].char_val;
                }

                // Now that we've found the end of the integer literal, turn the slice into a string
                let string_lexeme: String = string_vec.iter().collect();

                // Push an 'integer literal' token into the vector of tokens, with the newly created lexeme
                tokens.push(Token {name: TokenName::STRLIT, lexeme: string_lexeme, line_num: line_num});

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

    // Once we've gone through the whole file, add an EOF token at the end
    tokens.push(Token {name: TokenName::EOF, lexeme: String::from("EOF"), line_num: 0});

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

struct Char {
    char_val: char,
    line_num: i32,
}

// Loops through a file and returns a vector containing each of its characters
fn get_chars(file: &str) -> Vec<Char> {
    // Initialize an empty vector to hold characters
    let mut char_vec = Vec::new();

    if let Ok(lines) = read_lines(file) {
        // Loop through the lines of the file, storing each line as a string
        let mut line_num = 0;
        for line in lines {
            if let Ok(line_str) = line {
                line_num += 1;
                // Loop through each character in the line
                for ch in line_str.chars() {
                    // Add the character to the vector
                    char_vec.push(Char{char_val: ch, line_num: line_num});
                }

                // Make sure a newline character is included in the vector at the end of each line
                char_vec.push(Char{char_val: '\n', line_num: line_num});
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