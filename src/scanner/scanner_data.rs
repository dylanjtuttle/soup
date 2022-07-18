// -------------------------------------------------------------------------------------------------------------
// This file contains data structures used in scanning the compilee file for tokens, the first step of compiling
// -------------------------------------------------------------------------------------------------------------

// Struct to hold character data along with the line of the file the character is on
pub struct Char {
    pub char_val: char,
    pub line_num: i32,
}

// Struct to hold information about a token, like its type, its lexeme, and the line of the file it is found on
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_num: i32,
}

// An enumeration to define Token types for easy comparison
#[derive(PartialEq)]  // Allow TokenTypes to be checked for equality
#[derive(Clone, Copy)] // Allow TokenTypes to be copied implicitly
pub enum TokenType {
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