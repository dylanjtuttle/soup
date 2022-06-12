use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn scanner(code_file: &str) {
    // Get a vector of characters from the file
    let chars = get_chars(code_file);
    
    // Loop through the characters
    for ch in chars {
        // Print each character
        println!("{}", ch);
    }
}

// Loops through a file and returns a vector containing each of its characters
fn get_chars(file: &str) -> Vec<char> {
    // Initialize an empty vector to hold characters
    let mut char_vec = Vec::new();

    if let Ok(lines) = read_lines(file) {
        // Loop through the lines of the file, storing each line as a string
        for line in lines {
            if let Ok(line_str) = line {
                // Loop through each character in the line
                for ch in line_str.chars() {
                    // Add the character to the vector
                    char_vec.push(ch);
                }
            }
        }
    }

    // Return the vector
    char_vec
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    // Wrap the output in a Result to allow for error checking
    Ok(io::BufReader::new(file).lines())
}