use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::parser_data::ASTNode;
use crate::semantic::semantic_data::Symbol;

use crate::code_gen::code_gen_data::ASMWriter;

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

    // Make sure the amount of bytes is quad-word aligned
    while var_alloc % 16 != 0 {
        var_alloc += 4;
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

// Allocate/deallocate the requested amount of space on the stack
// (positive for allocating, negative for deallocating)
pub fn allocate_stack(writer: &mut ASMWriter, mut allocate: i32) {
    // We always have to allocate an amount which is quad-word aligned
    while allocate % 16 != 0 {
        if allocate < 0 {
            allocate -= 4;
        } else {
            allocate += 4;
        }
    }

    // Move the stack pointer to allocate/deallocate the requested amount of space
    if allocate < 0 {
        writer.write(&format!("        add     sp, sp, {}", -allocate));
    } else if allocate > 0 {
        writer.write(&format!("        sub     sp, sp, {}", allocate));
    } // else (allocate = 0), do nothing

    // Correct the addresses of all local variables in the current function now that we've moved the stack pointer
    increment_addrs(&writer.get_current_func(), allocate, &mut vec![]);
}

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