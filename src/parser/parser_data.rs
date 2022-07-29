use std::io;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use crate::semantic::semantic_data::Symbol;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct ASTNode {
    pub node_type: String,
    pub attr: Option<String>,
    pub line_num: Option<i32>,
    pub type_sig: Option<String>,
    pub sym: Option<Rc<RefCell<Symbol>>>,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(node_type: &str, attr: Option<String>, line_num: Option<i32>) -> ASTNode {
        return ASTNode {
            node_type: String::from(node_type),
            attr: attr,
            line_num: line_num,
            type_sig: None,
            sym: None,
            children: vec![],
        };
    }

    // ---------------------------------------------------------------------------------------
    // Adders
    // ---------------------------------------------------------------------------------------

    pub fn add_child(&mut self, new_node: ASTNode) {
        self.children.push(new_node);
    }
  
    pub fn add_children(&mut self, new_nodes: Vec<ASTNode>) {
        for node in new_nodes {
            self.children.push(node);
        }
    }

    pub fn add_child_to_front(&mut self, new_node: ASTNode) {
        // Get the current vector of children
        let children = &self.children;

        // Create a new vector for children
        let mut new_children = Vec::new();

        // Insert the new node at the beginning of this new vector
        new_children.push(new_node);

        // Add all of the old children after the new node
        for child in children {
            new_children.push(child.clone());
        }

        // Replace the old children vector with the new one
        self.children = new_children;
    }

    pub fn add_sym(&mut self, new_sym: Rc<RefCell<Symbol>>) {
        self.sym = Some(new_sym);
    }

    // ---------------------------------------------------------------------------------------
    // Getters
    // ---------------------------------------------------------------------------------------

    pub fn get_attr(&self) -> String {
        match &self.attr {
            None => String::from("ATTR"), // Should never happen, indicates an error on my end
            Some(attr) => {
                attr.clone()
            }
        }
    }

    pub fn get_line_num(&self) -> i32 {
        match self.line_num {
            None => 0,  // Should never happen, indicates an error on my end
            Some(line_num) => {
                line_num
            }
        }
    }

    pub fn get_type(&self) -> String {
        match &self.type_sig {
            None => {
                match &self.sym {
                    None => {
                        if self.node_type == "int" || self.node_type == "bool" || self.node_type == "string" {
                            self.node_type.clone()
                        } else {
                            String::from("NO TYPE")  // Should never happen, indicates an error on my end
                        }
                    }
                    Some(sym) => {sym.borrow().returns.clone()}
                }
            }
            Some(type_sig) => {
                type_sig.clone()
            }
        }
    }

    pub fn get_func_name(&self) -> String {
        return match &self.sym {
            None => String::from("FUNC"),  // Should never happen, indicates an error on my end
            Some(sym) => sym.borrow().name.clone()
        }
    }

    pub fn get_sym(&self) -> Rc<RefCell<Symbol>> {
        return match &self.sym {
            // Should never happen, indicates an error on my end
            None => Rc::new(RefCell::new(Symbol::new(String::from("SYMBOL"), String::from("SYMBOL"), String::from("SYMBOL")))),
            Some(sym) => Rc::clone(sym)
        }
    }

    // ---------------------------------------------------------------------------------------
    // Misc
    // ---------------------------------------------------------------------------------------

    // Generate the function signature for a function node
    pub fn get_func_sig(&self) -> String {
        // Open func sig
        let mut func_sig = String::from("f(");
    
        // Loop through parameters
        let mut param_num = 0;
        for param in &self.children[1].children {
            param_num += 1;
    
            // Function parameters must be comma separated, so any parameter after the first must be prefixed by ", "
            if param_num > 1 {
                func_sig.push_str(", ");
            }
    
            // Add parameter/argument type to func sig
            func_sig.push_str(&&param.children[0].get_type());
        }
    
        // Close func sig
        func_sig.push_str(")");
    
        return func_sig;
    }

    // Check if the current node or any of its children are a return node
    pub fn has_nonempty_return(&self) -> bool {
        // If the current node is a return node, return true
        if self.node_type == "return" && self.get_type() != "void" {
            return true;

        } else {
            // Otherwise, if any of the children are or have a return node, return true
            for child in &self.children {
                if child.has_nonempty_return() {
                    return true;
                }
            }

            // If none of the children are or have a return node, return false
            return false;
        }
    }

    // Format the data contained in this node
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

        // Only print type sig if it exists
        let type_sig = match &self.type_sig {
            None => String::from(""),
            Some(type_sig) => format!(", type: '{}'", type_sig),
        };

        display_string.push_str(&type_sig);

        // Only print symbol table entry if it exists
        let sym = match &self.sym {
            None => String::from(""),
            Some(symbol_entry) => {
                let mut sym_string = format!(", sym: {{name: {}, sig: {}, returns: {}",
                        symbol_entry.borrow().name,
                        symbol_entry.borrow().type_sig,
                        symbol_entry.borrow().returns);

                // Add label if it exists
                let label_string = match &symbol_entry.borrow().label {
                    None => String::from(""),
                    Some(label) => {
                        format!(", label: {}", label)
                    }
                };

                sym_string.push_str(&label_string);

                // Add memory address if it exists
                let addr_string = match &symbol_entry.borrow().addr {
                    None => String::from(""),
                    Some(addr) => {
                        format!(", addr: {}", addr)
                    }
                };

                sym_string.push_str(&addr_string);

                sym_string.push_str("}");

                sym_string
            }
        };

        display_string.push_str(&sym);

        // Print node close brace
        display_string.push_str("}");

        return display_string;
    }
}

// Print the current node, called by print_ast
fn print_node(node: &ASTNode, num_tabs: i32) {
    // Add the correct indentation by adding num_tabs tabs
    for _i in 0..num_tabs {
        print!("\t");                   // Print a tab without a newline at the end
        io::stdout().flush().unwrap();  // 
    }

    // Print current node
    println!("{}", node.display_string());

    // Call recursively on the nodes children
    for child in &node.children {
        print_node(child, num_tabs + 1);
    }
}

// Small wrapper to improve the printing quality of the AST print
// and abstract away the need to explicitly give the initial tab level
pub fn print_ast(node: &ASTNode) {
    println!("\n--------------------------------------------------------------------------------------------------------------------------------------------------------------");
    println!("AST: beginning from {{{}}} node", node.node_type);
    println!("--------------------------------------------------------------------------------------------------------------------------------------------------------------");

    print_node(node, 0);

    println!("--------------------------------------------------------------------------------------------------------------------------------------------------------------\n");
}