// --------------------------------------------------------------------------------------------------------------
// This file contains helper functions to assist in scanning the compilee for tokens, the first step of compiling
// --------------------------------------------------------------------------------------------------------------

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::scanner::scanner_data::{Token, TokenType, Char};
use crate::throw_error;

// --------------------------------------------------------------------------------------
// SCANNING - SEPARATORS
// --------------------------------------------------------------------------------------

pub fn get_separators(chars: &Vec<Char>, i: &mut usize) -> Option<Token> {
    // We know that we've matched a separator, so we already know we can increment to the next character
    *i += 1;

    let char = chars[*i - 1].char_val;
    let line_num = chars[*i - 1].line_num;

    match char {
        '(' => {return Some(Token {token_type: TokenType::OPENPAR, lexeme: String::from("("), line_num: line_num});}
        ')' => {return Some(Token {token_type: TokenType::CLOSEPAR, lexeme: String::from(")"), line_num: line_num});}
        '{' => {return Some(Token {token_type: TokenType::OPENBRACE, lexeme: String::from("{"), line_num: line_num});}
        '}' => {return Some(Token {token_type: TokenType::CLOSEBRACE, lexeme: String::from("}"), line_num: line_num});}
        ';' => {return Some(Token {token_type: TokenType::SEMICOLON, lexeme: String::from(";"), line_num: line_num});}
        ',' => {return Some(Token {token_type: TokenType::COMMA, lexeme: String::from(","), line_num: line_num});}
        // Will never happen since we already matched one of the above separators
        _ => {return None}
    }
}

// --------------------------------------------------------------------------------------
// SCANNING - SINGLE CHARACTER BINARY OPERATORS
// --------------------------------------------------------------------------------------

// Given a character in the character list, knowing it matches one of the 9 single character binary operators,
// return the correct corresponding token (or nothing in the case of a comment, which may look like a division token at first!)
pub fn get_binary_ops(chars: &Vec<Char>, i: &mut usize) -> Option<Token> {
    match chars[*i].char_val {
        '+' => {return get_binary_op(chars, i, TokenType::PLUS, TokenType::PLUSEQ, "+");}
        '-' => {return get_binary_op(chars, i, TokenType::MINUS, TokenType::MINUSEQ, "-");}
        '*' => {return get_binary_op(chars, i, TokenType::MULT, TokenType::MULTEQ, "*");}
        '/' => {return get_binary_op(chars, i, TokenType::DIV, TokenType::DIVEQ, "/");}
        '%' => {return get_binary_op(chars, i, TokenType::MOD, TokenType::MODEQ, "%");}
        '<' => {return get_binary_op(chars, i, TokenType::LT, TokenType::LEQ, "<");}
        '>' => {return get_binary_op(chars, i, TokenType::GT, TokenType::GEQ, ">");}
        '=' => {return get_binary_op(chars, i, TokenType::ASSIGN, TokenType::EQ, "=");}
        '!' => {return get_binary_op(chars, i, TokenType::NOT, TokenType::NEQ, "!");}
        // Will never happen since we already matched one of the above operators
        _ => {return None;}
    }
}

// Given a character in the character list and a certain single character operator, either return a token for that operator,
// the "equals" version of that operator (e.g. += for +), or nothing in the special case of a comment (which
// may look like a division token at first!)
fn get_binary_op(chars: &Vec<Char>, i: &mut usize, op_type: TokenType, alt_type: TokenType, op_lexeme: &str) -> Option<Token> {
    // Initialize a binary operator token of the requested type
    let mut token = Token {token_type: op_type, lexeme: String::from(op_lexeme), line_num: chars[*i].line_num};

    // Check to see if token is 'op=', not just 'op' (for example, '+=' or '<=' instead of just '+' or '<')
    if chars[*i + 1].char_val == '=' {
        // Update token information
        token.token_type = alt_type;
        token.lexeme.push('=');
        
        // Skip the next char, since it is a part of our current token
        *i += 2;

        return Some(token);

    } else if op_type == TokenType::DIV && chars[*i + 1].char_val == '/' {
        // We have a comment, loop until we find a newline character
        let mut comment_char = chars[*i].char_val;
        while comment_char != '\n' {
            *i += 1;
            comment_char = chars[*i].char_val;
        }

        return None;

    } else {
        // Nothing fancy is going on

        // Prepare to move along to the next char
        *i += 1;

        // Return the token
        return Some(token);
    }

}

// --------------------------------------------------------------------------------------
// SCANNING - AND / OR
// --------------------------------------------------------------------------------------

// Given a character in the character list, knowing it is either '&' or '|', make sure
// the next character is also '&' or '|' respectively and return the corresponding token,
// or throw an error otherwise
pub fn get_and_or(chars: &Vec<Char>, i: &mut usize, op_type: TokenType, op_lexeme: &str) -> Option<Token> {
    // Check to see if the token is '&&' or '||' as it should be, depending on what the first character is
    if (op_type == TokenType::AND && chars[*i + 1].char_val == '&')
    || (op_type == TokenType::OR && chars[*i + 1].char_val == '|') {
        // Skip the next char, since it is a part of our current token
        *i += 2;

        // Return the corresponding token
        return Some(Token {token_type: op_type, lexeme: String::from(op_lexeme), line_num: chars[*i].line_num});

    } else {
        // Otherwise, this is an invalid token
        throw_error(&format!("Unrecognized token '{}'", chars[*i].char_val));
        return None;
    }
}

// --------------------------------------------------------------------------------------
// SCANNING - RESERVED WORDS
// --------------------------------------------------------------------------------------

// Given a character in the character list, knowing it is an ID character,
// loop through all of the possible reserved words and check if
// the given character is the start of any of them
pub fn get_reserved(chars: &Vec<Char>, i: &mut usize) -> Option<Token> {
    let reserved_types = vec![TokenType::IF, TokenType::INT,
                                              TokenType::TRUE, TokenType::BOOL,
                                              TokenType::VOID, TokenType::ELSE,
                                              TokenType::FUNC, TokenType::MAIN,
                                              TokenType::FALSE, TokenType::WHILE,
                                              TokenType::BREAK, TokenType::RETURN,
                                              TokenType::RETURNS];
                                            
    let reserved_lexemes = vec!["if", "int", "true", "bool",
                                           "void", "else", "func", "main",
                                           "false", "while", "break",
                                           "return", "returns"];
       
    // Loop through the reserved words and try to match each
    // If one matches, return the corresponding token
    for j in 0..reserved_types.len() {
        match check_reserved(chars, i, reserved_types[j], reserved_lexemes[j]) {
            None => {}
            Some(token) => {return Some(token);}
        }
    }

    // If we've looped through all of the reserved words and haven't found a match,
    // this token is not the start of a reserved word and we can return None
    return None;
}

// Given a character in the character list and a particular reserved word,
// check if the given character is the start of that reserved word
pub fn check_reserved(chars: &Vec<Char>, i: &mut usize, reserved_type: TokenType, reserved: &str) -> Option<Token> {
    // Check if there are enough characters in the list to check for the reserved word
    if has_enough_chars(chars, i, reserved.len()) {
        // Form a list of chars so we can turn it into a string and check if it equals the given reserved word
        let mut char_vec = Vec::new();

        for j in 0..reserved.len() + 1 {
            char_vec.push(chars[*i + j].char_val);
        }

        if is_reserved(char_vec, reserved) {
            // Skip the chars comprising the reserved word, since they're a part of our current token
            *i += reserved.len();

            // Return a token corresponding to the reserved word
            return Some(Token {token_type: reserved_type, lexeme: String::from(reserved), line_num: chars[*i].line_num});
        }

        // If there are enough chars but the chars do not match the reserved word, return None
        return None;
    }

    // If there aren't enough chars to match the reserved word, return None
    return None;
}

// Returns true if a vector of characters equals a reserved word + a non-id character, and false otherwise
pub fn is_reserved(actual: Vec<char>, reserved: &str) -> bool {
    let actual_str: String = actual[0..actual.len() - 1].iter().collect();
    let next_char = actual[actual.len() - 1];

    // If the string equals the reserved word and the next character is not alphanumeric or an underscore, this is a reserved word
    actual_str == reserved && !is_id_char(next_char)
}

// Checks to see if the vector of chars is long enough to check for a reserved word of length num_chars
pub fn has_enough_chars(chars: &Vec<Char>, i: &usize, num_chars: usize) -> bool {
    // For example, if we want to check if a character at index 8 of a length 10 character vector is the
    // first letter of the reserved word "while", we would get an index out of bounds error when checking
    // if the next 5 characters are 'w', 'h', 'i', 'l', and 'e', because there aren't that many characters left!
    chars.len() - i - 1 >= num_chars
}

// --------------------------------------------------------------------------------------
// SCANNING - IDENTIFIERS
// --------------------------------------------------------------------------------------

pub fn get_identifier(chars: &Vec<Char>, i: &mut usize) -> Token {
    let mut id_char = chars[*i].char_val;
    let line_num = chars[*i].line_num;
    let mut id_vec = Vec::new();

    // Loop until we've found a non-id character
    while is_id_char(id_char) {
        // Add the id character to the vector
        id_vec.push(id_char);

        // Move to the next character and jump to the loop test again
        *i += 1;
        id_char = chars[*i].char_val;
    }

    // We incremented one character too far because we had to find a non-id character to exit the loop
    *i -= 1;

    // Now that we've found the end of the identifier, turn the slice into a string
    let id_lexeme: String = id_vec[0..id_vec.len()].iter().collect();

    // Prepare to move along to the next char
    *i += 1;

    // Return an 'identifier' token, with the newly created lexeme
    return Token {token_type: TokenType::ID, lexeme: id_lexeme, line_num: line_num};
}

// --------------------------------------------------------------------------------------
// SCANNING - INTEGER LITERALS
// --------------------------------------------------------------------------------------

pub fn get_int_lits(chars: &Vec<Char>, i: &mut usize) -> Token {
    // We have to check for multiple digit literals
    let mut int_lit_char = chars[*i].char_val;
    let mut int_lit_vec = Vec::new();

    // Loop until we've found a non-digit character
    while is_digit(int_lit_char) {
        // Add digit character to the vector
        int_lit_vec.push(int_lit_char);

        // Move to the next character and jump to the loop test again
        *i += 1;
        int_lit_char = chars[*i].char_val;
    }

    // We incremented one character too far because we had to find a non-digit character to exit the loop
    *i -= 1;

    // Now that we've found the end of the integer literal, turn the slice into a string
    let int_lit_lexeme: String = int_lit_vec[0..int_lit_vec.len()].iter().collect();

    // Prepare to move along to the next char
    *i += 1;

    // Return an 'integer literal' token, with the newly created lexeme
    return Token {token_type: TokenType::INTLIT, lexeme: int_lit_lexeme, line_num: chars[*i].line_num};
}

// --------------------------------------------------------------------------------------
// SCANNING - STRING LITERALS
// --------------------------------------------------------------------------------------

pub fn get_str_lits(chars: &Vec<Char>, i: &mut usize) -> Token {
    let mut string_vec = Vec::new();
    // Skip the open quote
    *i += 1;

    // Loop until we find another quotation mark
    let mut string_char = chars[*i].char_val;
    while string_char != '"' {
        string_vec.push(string_char);

        *i += 1;
        string_char = chars[*i].char_val;
    }

    // Now that we've found the end of the string literal, turn the slice into a string
    let string_lexeme: String = string_vec.iter().collect();

    // Prepare to move along to the next char
    *i += 1;

    // Return a 'string literal' token, with the newly created lexeme
    return Token {token_type: TokenType::STRLIT, lexeme: string_lexeme, line_num: chars[*i].line_num};
}

// --------------------------------------------------------------------------------------
// HELPERS - CHARACTER TYPE CHECKING
// --------------------------------------------------------------------------------------

// Returns true if a character is in a..z, A..Z, 0..9, or is an underscore, and false otherwise
pub fn is_id_char(id_char: char) -> bool {
    (id_char >= 'a' && id_char <= 'z') || (id_char >= 'A' && id_char <= 'Z') || is_digit(id_char) || (id_char == '_')
}

// Returns true if a character is in 0..9
pub fn is_digit(digit_char: char) -> bool {
    digit_char >= '0' && digit_char <= '9'
}

// --------------------------------------------------------------------------------------
// HELPERS - FILE READING
// --------------------------------------------------------------------------------------

// Loops through a file and returns a vector containing each of its characters
pub fn get_chars(file: &str) -> Vec<Char> {
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
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    // Wrap the output in a Result to allow for error checking
    Ok(io::BufReader::new(file).lines())
}