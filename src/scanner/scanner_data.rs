// -------------------------------------------------------------------------------------------------------------
// This file contains data structures used in scanning the compilee file for tokens, the first step of compiling
// -------------------------------------------------------------------------------------------------------------

// Struct to hold character data along with the line of the file the character is on
#[derive(Debug, PartialEq)]
pub struct Char {
    pub char_val: char,
    pub line_num: i32,
}

// Struct to hold information about a token, like its type, its lexeme, and the line of the file it is found on
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_num: i32,
}

impl Token {
    pub fn is_type_token(&self) -> bool {
        return self.token_type == TokenType::INT
            || self.token_type == TokenType::FLOAT
            || self.token_type == TokenType::BOOL;
    }
}

// An enumeration to define Token types for easy comparison
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    ID,
    STRLIT,
    INTLIT,
    FLOATLIT,
    TRUE,
    FALSE,
    INT,
    FLOAT,
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
    EOF,
}
