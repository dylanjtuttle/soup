use std::io;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use crate::scanner::Token;


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
    let ast_root = new_node("program", None, None);

    println!("First token is: {}", tokens[0].lexeme);

    // Add a test child
    let child_1 = new_node("child 1", Some(String::from("test attr")), Some(1));
    ast_root.borrow_mut().add_child(Rc::clone(&child_1));

    // Add another test child and a grandchild
    let child_2 = new_node("child 2", Some(String::from("another test attr")), Some(2));
    let grandchild = new_node("grandchild", Some(String::from("yet another test attr")), Some(3));
    child_2.borrow_mut().add_child(Rc::clone(&grandchild));
    ast_root.borrow_mut().add_child(Rc::clone(&child_2));

    return Rc::clone(&ast_root);
}