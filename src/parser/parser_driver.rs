use crate::scanner::scanner_data::Token;
use crate::parser::parser_data::*;
use crate::parser::parser_grammar::*;

// -----------------------------------------------------------------
// PARSER
// -----------------------------------------------------------------

pub fn parser(tokens: &Vec<Token>) -> ASTNode {
    start_(tokens, &mut 0)
}

// -----------------------------------------------------------------
// MISC FUNCTIONS
// -----------------------------------------------------------------

pub fn consume_token(current: &mut usize) {
    *current += 1;
}