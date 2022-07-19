use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::parser_data::ASTNode;
use crate::semantic::semantic_data::Symbol;

// -----------------------------------------------------------------------------------------
// FUNCTION DECLARATION HELPERS
// -----------------------------------------------------------------------------------------

// Loop through all variables in a function declaration and assign them a memory address
pub fn declare_variables(node: &mut ASTNode, current_offset: &mut i32) {
    if node.node_type == "parameter" || node.node_type == "varDecl" {
        // Add the local variable's offset to its symbol table entry
        node.get_sym().borrow_mut().addr = Some(*current_offset);

        // Increment the current offset by the size of this local variable for next time
        *current_offset += 4;
    }

    // Visit children
    for child in &mut node.children {
        declare_variables(child, current_offset);
    }
}

// Calculate the number of bytes a function needs to allocate on the stack
pub fn get_func_stack_alloc(node: &ASTNode) -> i32 {
    // Calculate the number of bytes we need to allocate on the stack for local variables
    let mut var_alloc = get_func_var_alloc(node);

    // If the number of bytes isn't at least 16, make it 16
    if var_alloc < 16 {
        var_alloc = 16;
    }

    return var_alloc;
}

// Calculate the number of bytes a function needs to allocate on the stack for its local variables
pub fn get_func_var_alloc(node: &ASTNode) -> i32 {
    let mut num_bytes = 0;

    if node.node_type == "parameter" || node.node_type == "varDecl" {
        num_bytes += 4;
    }

    // Visit children
    for child in &node.children {
        num_bytes += get_func_var_alloc(child);
    }

    return num_bytes;
}

// -----------------------------------------------------------------------------------------
// STACK ALLOCATION HELPERS
// -----------------------------------------------------------------------------------------

// Loop through every variable declaration (including parameters) and increment their memory address by the given amount
pub fn increment_addrs(node: &ASTNode, increment: i32, already_incremented: &mut Vec<Rc<RefCell<Symbol>>>) {
    if node.node_type == "varDecl" || node.node_type == "parameter" {
        match &node.sym {
            None => {}
            Some(sym) => {
                if !already_incremented.contains(sym) {
                    already_incremented.push(Rc::clone(sym));
    
                    match &mut sym.borrow_mut().addr {
                        None => {}
                        Some(addr) => {
                            *addr += increment;
                        }
                    }
                }
            }
        }
    }

    // Visit children
    for child in &node.children {
        increment_addrs(child, increment, already_incremented);
    }
}