// -----------------------------------------------------------------------------------------------------------
// This file contains the main logic involved in scanning the compilee for tokens, the first step of compiling
// -----------------------------------------------------------------------------------------------------------

use crate::scanner::scanner_data::{Token, TokenType, Char};
use crate::scanner::scanner_utils::*;
use crate::throw_error;

// --------------------------------------------------------------------------------------
// SCANNER
// --------------------------------------------------------------------------------------

// Main scanner function, returns the vector of tokens scanned from the compilee file
pub fn scanner(code_file: &str) -> Vec<Token> {
    // Get a vector of characters from the file
    let chars = get_chars(code_file);

    // Create a vector to add tokens to
    let mut tokens = Vec::new();

    // Loop through the characters
    let mut i = 0;
    while i < chars.len() {
        // Try to get a token, and push it to the list if you get one
        match get_token(&chars, &mut i) {
            None => {}
            Some(token) => {tokens.push(token)}
        }
    }

    // Once we've gone through the whole file, add an EOF token at the end
    tokens.push(Token {token_type: TokenType::EOF, lexeme: String::from("EOF"), line_num: chars[i - 1].line_num});

    // Return vector of tokens
    tokens
}

// --------------------------------------------------------------------------------------
// GET TOKEN
// --------------------------------------------------------------------------------------

// Tries to get and return one token from the file, starting from the character at index i
fn get_token(chars: &Vec<Char>, i: &mut usize) -> Option<Token> {
    match chars[*i].char_val {
        ' ' | '\t' | '\n' | '\r' => {
            // Ignore whitespace
            *i += 1;
            return None;
        }
        '(' | ')' | '{' | '}' | ';' | ',' => {
            return get_separators(chars, i);
        }
        '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '!' => {
            return get_binary_ops(chars, i);
        }
        '&' => {
            return get_and_or(chars, i, TokenType::AND, "&&");
        }
        '|' => {
            return get_and_or(chars, i, TokenType::OR, "||");
        }
        'A'..='Z' | 'a'..='z' | '_' => {
            // Possible identifier, but we have to check for reserved words first
            match get_reserved(chars, i) {
                // If we find a reserved word token, return it
                Some(reserved) => {return Some(reserved)}

                // Otherwise, we have an identifier
                None => {return Some(get_identifier(chars, i))}
            }
        }
        '0'..='9' => {
            // We have an integer literal
            return Some(get_int_lits(chars, i));
        }
        '"' => {
            // We have a string literal
            return Some(get_str_lits(chars, i));
        }
        unrecognized => {
            // If we haven't matched any tokens, throw an error
            throw_error(&format!("Unrecognized token '{}'", unrecognized));
            return None;
        }
    }
}