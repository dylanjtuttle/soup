use std::process;

use crate::scanner::TokenName;

pub mod scanner;
pub mod parser;

fn main() {
    let code_file = "test.txt";

    println!("\nBEGIN SCANNER");

    let tokens = scanner::scanner(code_file);

    for token in tokens {
        if token.name == TokenName::ID {
            println!("Token (ID): {}", token.lexeme);
        } else {
            println!("Token: {}", token.lexeme);
        }
    }

    println!("\nBEGIN PARSER");

    parser::parser();
}

pub fn throw_warning(msg: &str) {
    eprintln!("Warning: {}", msg);
}

pub fn throw_error(msg: &str) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
