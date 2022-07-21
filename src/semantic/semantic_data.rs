use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::parser_data::ASTNode;
use crate::throw_error;

pub struct ScopeStack {
    pub stack: Vec<HashMap<String, Rc<RefCell<Symbol>>>>
}

impl ScopeStack {
    // Create a new scope stack
    pub fn new() -> Self {
        ScopeStack{stack: Vec::new()}
    }

    // Return a mutable reference to the top scope in the stack, or None if the scope stack is empty
    pub fn peek(&mut self) -> Option<&mut HashMap<String, Rc<RefCell<Symbol>>>> {
        self.stack.last_mut()
    }

    // Open up a new scope by creating a new symbol table and pushing it to the top of the scope stack
    pub fn open_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    // Close the topmost scope by popping it and allowing it to go out of (this function's) scope
    pub fn close_scope(&mut self) {
        _ = self.stack.pop();
    }

    // Insert a new symbol into the topmost scope of the scope stack
    pub fn insert_symbol(&mut self, name: String, new_symbol: Rc<RefCell<Symbol>>) {
        match self.peek() {
            None => {
                throw_error("Empty scope stack");
            }
            Some(top) => {
                top.insert(name, new_symbol);
            }
        };
    }

    // Attempt to find a symbol somewhere in the scope stack
    pub fn find_symbol(&self, search_name: &str) -> Option<Rc<RefCell<Symbol>>> {
        // Iterate backwards through the scope stack (i.e. starting at the top scope and moving downwards)
        for symbol_table in self.stack.iter().rev() {

            // Search through each entry in the symbol table for the given name
            for (name, symbol) in symbol_table {

                // If we find a symbol with that name, return a reference to it
                if name == search_name {
                    return Some(Rc::clone(symbol));
                }
            }
        }

        // Otherwise, we weren't able to find a symbol with the given name, so return None
        None
    }

    // Return true if given symbol already exists in current scope, and false otherwise
    pub fn find_in_scope(&mut self, search_name: &str) -> bool {
        match self.peek() {
            // If the scope stack is empty, we obviously won't be able to find the symbol
            None => {false}
            Some(symbol_table) => {
                // Search through each entry in the symbol table for the given name
                for (name, _symbol) in symbol_table {
                    // If we find a symbol with that name, return true
                    if name == search_name {
                        return true;
                    }
                }

                // Otherwise, we weren't able to find a symbol with the given name, so return false
                false
            }
        }
    }

    // Return the level of the scope (the length of the list)
    pub fn scope_level(&self) -> usize {
        self.stack.len()
    }
}

// -----------------------------------------------------------------
// SYMBOL
// -----------------------------------------------------------------

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Symbol {
    pub name: String,
    pub type_sig: String,
    pub returns: String,
    pub label: Option<String>,
    pub addr: Option<i32>,
    pub stored_bytes: i32,
    pub active_callee_saved: Vec<usize>,
}

impl Symbol {
    // Create a new symbol
    pub fn new(name: String, type_sig: String, returns: String) -> Self {
        Symbol{name: name, type_sig: type_sig, returns: returns, label: None, addr: None, stored_bytes: 0, active_callee_saved: vec![]}
    }

    pub fn get_label(&self) -> String {
        return match &self.label {
            None => String::from("LABEL"),  // Should never happen, indicates an error on my end
            Some(label) => label.clone()
        }
    }

    pub fn get_addr(&self) -> i32 {
        return match &self.addr {
            None => -1,  // Should never happen, indicates an error on my end
            Some(addr) => *addr
        }
    }

    pub fn get_active_callees(&self) -> Vec<usize> {
        return self.active_callee_saved.clone();
    }
}

// Insert symbol into scope stack and AST node
pub fn insert_symbol(symbol: Symbol, scope_stack: &mut ScopeStack, ast_node: &mut ASTNode) {
    // Create a smart pointer to the symbol
    let rc_symbol = Rc::new(RefCell::new(symbol));

    // Add symbol to the scope stack
    scope_stack.insert_symbol(rc_symbol.borrow().name.clone(), Rc::clone(&rc_symbol));

    // Add symbol table entry to the AST node
    ast_node.add_sym(Rc::clone(&rc_symbol));
}