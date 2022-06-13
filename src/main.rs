use std::process;

use crate::scanner::TokenName;
use crate::parser::print_ast;

pub mod scanner;
pub mod parser;

fn main() {
    let code_file = "test.txt";

    println!("\nBEGIN SCANNER");

    let tokens = scanner::scanner(code_file);

    for token in &tokens {
        if token.name == TokenName::ID {
            println!("{}: Token (ID): {}", token.line_num, token.lexeme);
        } else {
            println!("{}: Token: {}", token.line_num, token.lexeme);
        }
    }

    println!("\nBEGIN PARSER");

    // Add arguments:
    // tokens: Vec<Token>
    print_ast(parser::parser(), 0);
}

pub fn throw_warning(msg: &str) {
    eprintln!("Warning: {}", msg);
}

pub fn throw_error(msg: &str) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
