use std::io;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use crate::{scanner::{Token, TokenName}, throw_error};


// -----------------------------------------------------------------
// AST
// -----------------------------------------------------------------

pub struct ASTNode {
    node_type: String,
    attr: Option<String>,
    line_num: Option<i32>,
    children: Vec<Rc<RefCell<ASTNode>>>,
}

impl ASTNode {
    pub fn new(node_type: &str, attr: Option<String>, line_num: Option<i32>) -> ASTNode {
        return ASTNode {
            node_type: String::from(node_type),
            attr: attr,
            line_num: line_num,
            children: vec![],
        };
    }
  
    pub fn add_child(&mut self, new_node: Rc<RefCell<ASTNode>>) {
      self.children.push(new_node);
    }

    pub fn add_children(&mut self, new_nodes: Vec<Rc<RefCell<ASTNode>>>) {
        for node in new_nodes {
            self.children.push(node);
        }
    }

    pub fn print_tree(&self, node: Rc<RefCell<ASTNode>>, num_tabs: i32) {
        // Add the correct indentation by adding num_tabs tabs
        for _i in 0..num_tabs {
            print!("\t");                   // Print a tab without a newline at the end
            io::stdout().flush().unwrap();  // 
        }

        // Print current node
        println!("{}", node.borrow_mut().display_string());

        // Call recursively on the nodes children
        for child in &node.borrow_mut().children {
            self.print_tree(Rc::clone(child), num_tabs + 1);
        }
    }
  
    pub fn display_string(&self) -> String {
        let mut display_string = format!("{{{}", self.node_type);

        // Only print attr if it exists
        let attr = match &self.attr {
            None => String::from(""),
            Some(attr) => format!(", attr: '{}'", attr),
        };

        display_string.push_str(&attr);

        // Only print line number if it exists
        let line_num = match self.line_num {
            None => String::from(""),
            Some(line_num) => format!(", line {}", line_num),
        };

        display_string.push_str(&line_num);

        // Print node close brace
        display_string.push_str("}");

        return display_string;
    }
  }


fn new_node(node_type: &str, attr: Option<String>, line_num: Option<i32>) -> Rc<RefCell<ASTNode>> {
    let node = Rc::new(RefCell::new(ASTNode::new(node_type, attr, line_num)));

    return node;
}

pub fn print_ast(node: Rc<RefCell<ASTNode>>, num_tabs: i32) {
    // Add the correct indentation by adding num_tabs tabs
    for _i in 0..num_tabs {
        print!("\t");                   // Print a tab without a newline at the end
        io::stdout().flush().unwrap();  // 
    }

    // Print current node
    println!("{}", node.borrow_mut().display_string());

    // Call recursively on the nodes children
    for child in &node.borrow_mut().children {
        print_ast(Rc::clone(child), num_tabs + 1);
    }
}


// -----------------------------------------------------------------
// PARSER
// -----------------------------------------------------------------

pub fn parser(tokens: &Vec<Token>) -> Rc<RefCell<ASTNode>> {
    start_(tokens, &mut 0)
}


// -----------------------------------------------------------------
// GRAMMAR NON-TERMINAL FUNCTIONS
// -----------------------------------------------------------------


// start		: {globaldeclarations}
// 			    ;
fn start_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Create the root program node for this code file
    let ast_root = new_node("program", None, None);

    if tokens[0].name != TokenName::EOF {
        // If this was an empty file, the first (and only) token would be EOF,
        // in which case we would just return the program node. However, since this file
        // is non-empty, we can parse through it and create our AST:

        ast_root.borrow_mut().add_child(variabledeclaration_(tokens, current));

        // ast_root.borrow_mut().add_children(globaldeclarations_(tokens, current));
    }

    return ast_root;
}


// literal     : INTLIT
//             | STRLIT
//             | TRUE
//             | FALSE
//             ;
fn literal_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // Create AST leaf node for literal
    let literal_node = new_node("literal",
                                                      Some(current_token.lexeme.clone()),
                                                      Some(current_token.line_num));

    // Update the literal node type to correspond to the token we see
    match current_token.name {
        TokenName::INTLIT => {literal_node.borrow_mut().node_type = String::from("number");}
        TokenName::STRLIT => {literal_node.borrow_mut().node_type = String::from("string");}
        TokenName::TRUE => {literal_node.borrow_mut().node_type = String::from("true");}
        TokenName::FALSE => {literal_node.borrow_mut().node_type = String::from("false");}
        _ => {
            throw_error(&format!("Syntax Error on line {}: literal must be an integer, string, \"true\", or \"false\"",
                        tokens[*current + 1].line_num));
        }
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return the literal AST node
    return literal_node;
}


fn type_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // Create AST leaf node for type
    let type_node = new_node("type",
                                                   Some(current_token.lexeme.clone()),
                                                   Some(current_token.line_num));

    // Update the type node type to correspond to the token we see
    match current_token.name {
        TokenName::INT => {type_node.borrow_mut().node_type = String::from("int");}
        TokenName::BOOL => {type_node.borrow_mut().node_type = String::from("bool");}
        _ => {
            throw_error(&format!("Syntax Error on line {}: type must be one of \"int\", \"bool\"",
            tokens[*current + 1].line_num));
        }
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return the type AST node
    return type_node;
}


// variabledeclaration     : type identifier SEMICOLON
//                         ;
fn variabledeclaration_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    let mut current_token = &tokens[*current];

    // Create variable declaration node
    let var_decl_node = new_node("varDecl",
                                                       None,
                                                       Some(current_token.line_num));
    
    // Add child for the variable type
    var_decl_node.borrow_mut().add_child(type_(tokens, current));

    // Add child for the variable identifier
    var_decl_node.borrow_mut().add_child(identifier_(tokens, current));

    // Check to see if current token is a semicolon - if not, throw syntax error
    current_token = &tokens[*current];
    if current_token.name != TokenName::SEMICOLON {
        throw_error(&format!("Syntax Error on line {}: variable declaration must end with a semicolon \";\"",
                    current_token.line_num));
    }

    // If we made it to here, we must have successfully parsed the variable declaration,
    // so return the newly created node!
    return var_decl_node;
}


// identifier              : ID
//                         ;
fn identifier_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    if current_token.name != TokenName::ID {
        throw_error(&format!("Syntax Error on line {}: expected an identifier",
                    current_token.line_num));
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return an identifier AST node corresponding to the ID token
    return new_node("id",
                    Some(current_token.lexeme.clone()),
                    Some(current_token.line_num));
}


// -----------------------------------------------------------------
// MISC FUNCTIONS
// -----------------------------------------------------------------

fn consume_token(current: &mut usize) {
    *current += 1;
}
