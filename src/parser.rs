use std::io;
use std::io::Write;

use crate::scanner::Token;

pub struct ASTNode {
    node_type: String,
    attr: Option<String>,
    children: Vec<ASTNode>
}

pub fn parser(tokens: Vec<Token>) -> ASTNode {
    start(tokens, 0)
}


// -----------------------------------------------------------------
// GRAMMAR NON-TERMINAL FUNCTIONS
// -----------------------------------------------------------------

// start            : /* empty */
//                  | globaldeclarations
//                  ;
fn start(tokens: Vec<Token>, current: i32) -> ASTNode {
    // Create a root 'program' node
    ASTNode{node_type: String::from("program"), attr: None, children: Vec::new()}
}


// -----------------------------------------------------------------
// MISC FUNCTIONS
// -----------------------------------------------------------------

// Pretty-print an AST
pub fn print_ast(node: ASTNode, num_tabs: i32) {
    // Add the correct indentation by adding num_tabs tabs
    for _i in 0..num_tabs {
        print!("\t");                   // Print a tab without a newline at the end
        io::stdout().flush().unwrap();  // 
    }

    // Print node information
    print!("{{node: {}", node.node_type);
    io::stdout().flush().unwrap();

    // Only print attr if it exists
    match node.attr {
        None => {
            print!("");
            io::stdout().flush().unwrap();
        }
        Some(attr) => {
            print!(", attr: '{}'", attr);
            io::stdout().flush().unwrap();
        }
    }

    // Print node close brace
    println!("}}");
    
    // Call print_ast on each child of the node
    for child in node.children {
        print_ast(child, num_tabs + 1);
    }
}