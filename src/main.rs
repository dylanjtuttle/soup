use std::process;
use std::env;

pub mod scanner;
pub mod parser;
pub mod semantic;
pub mod code_gen;

use crate::scanner::scanner_driver::scanner;
use crate::parser::parser_driver::parser;
use crate::semantic::semantic_driver::semantic_checker;
use crate::code_gen::code_gen_driver::code_gen;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        throw_error("No file given to compile, exiting now");
    }

    let code_file = &args[1];
    let asm_file = &args[2];

    // Scanner
    let tokens = scanner(code_file);

    // Parser
    let mut ast = parser(&tokens);

    // Semantic checker
    semantic_checker(&mut ast);

    // Code generation
    code_gen(&asm_file, &mut ast);
}

pub fn throw_warning(msg: &str) {
    eprintln!("Warning: {}", msg);
}

pub fn throw_error(msg: &str) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
