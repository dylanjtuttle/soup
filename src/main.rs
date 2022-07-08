use std::process;
use std::env;

use crate::scanner::TokenName;
use crate::parser::print_ast;

pub mod scanner;
pub mod parser;
pub mod semantic;
pub mod code_gen;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        throw_error("No file given to compile, exiting now");
    }

    let code_file = &args[1];

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

    let mut ast = parser::parser(&tokens);

    print_ast(&ast);

    println!("\nBEGIN SEMANTIC CHECKING:\n");

    semantic::semantic_checker(&mut ast);

    println!("\nBEGIN CODE GENERATION:\n");

    code_gen::code_gen("asm/test.asm", &mut ast);
}

pub fn throw_warning(msg: &str) {
    eprintln!("Warning: {}", msg);
}

pub fn throw_error(msg: &str) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
