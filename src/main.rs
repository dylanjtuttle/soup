use std::process;

pub mod scanner;

fn main() {
    let code_file = "test.txt";
    let tokens = scanner::scanner(code_file);

    for token in tokens {
        println!("Token: {}", token.lexeme);
    }
}

pub fn throw_warning(msg: &str) {
    eprintln!("Warning: {}", msg);
}

pub fn throw_error(msg: &str) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
