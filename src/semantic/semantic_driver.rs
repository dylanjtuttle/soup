use std::cell::RefCell;
use std::rc::Rc;

use crate::parser::parser_data::ASTNode;
use crate::semantic::semantic_callbacks::*;
use crate::semantic::semantic_data::*;
use crate::throw_error;

// -----------------------------------------------------------------
// SEMANTIC CHECKER
// -----------------------------------------------------------------

pub fn semantic_checker(ast: &mut ASTNode) {
    // This semantic checker will perform five traversals of the AST:
    //
    // pass 1 - post-order - collects information about global declarations
    //
    // pass 2 - pre/post-order - figures out what all the identifiers refer to
    //                           deals with the scope stack, opens up a new scope
    //                           in the pre pass and closes it in the post pass
    //
    // pass 3 - post-order - full type checking
    //
    // pass 4 - pre/post-order - ensure break statements are inside while loops
    //                           and if/while conditions are of boolean type
    // pass 5 - pre/post-order - various checks of return statements and their functions

    // Begin by creating the scope stack, this will hold a symbol table for each level of scope:
    // level 1 - runtime library
    // level 2 - global
    // level 3 - function
    let mut scope_stack = ScopeStack::new();

    // Open a new scope for the runtime library
    scope_stack.open_scope();

    // Add a symbol for everything in the runtime library
    scope_stack.insert_symbol(
        String::from("exit"),
        Rc::new(RefCell::new(Symbol::new(
            String::from("exit"),
            String::from("f(int)"),
            String::from("void"),
        ))),
    );
    scope_stack.insert_symbol(
        String::from("printf"),
        Rc::new(RefCell::new(Symbol::new(
            String::from("printf"),
            String::from("f(string, ...)"),
            String::from("void"),
        ))),
    );

    // Open a new scope for the global symbols in anticipation of the first pass
    scope_stack.open_scope();

    // Begin first pass
    let mut num_main_decls = 0;
    pass1(ast, &mut scope_stack, &mut num_main_decls);

    // Check for incorrect number of main declarations
    if num_main_decls == 0 {
        throw_error("Program must contain a main function declaration");
    } else if num_main_decls > 1 {
        throw_error("Program cannot contain more than one main function declaration")
    }

    // Begin second pass
    pass2(ast, &mut scope_stack);

    // Begin third pass
    pass3(ast, &mut scope_stack);

    // Begin fourth pass
    pass4(ast, &mut 0);

    // Begin fifth pass
    pass5(ast, &mut String::from("None"));
}

// -----------------------------------------------------------------
// AST TRAVERSALS
// -----------------------------------------------------------------

fn pass1(node: &mut ASTNode, scope_stack: &mut ScopeStack, num_main_decls: &mut i32) {
    // Call recursively on the current node's children
    for child in &mut node.children {
        pass1(child, scope_stack, num_main_decls);
    }

    // Execute pass1 function
    pass1_post(node, scope_stack, num_main_decls);
}

fn pass2(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    // Execute pass2 function before checking node children
    pass2_pre(node, scope_stack);

    // Call recursively on the current node's children
    for child in &mut node.children {
        pass2(child, scope_stack);
    }

    // Execute pass2 function after checking node children
    pass2_post(node, scope_stack);
}

fn pass3(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    // Call recursively on the current node's children
    for child in &mut node.children {
        pass3(child, scope_stack);
    }

    // Execute pass3 function after checking node children
    pass3_post(node, scope_stack);
}

fn pass4(node: &mut ASTNode, while_depth: &mut i32) {
    // Execute pass4 function before checking node children
    pass4_pre(node, while_depth);

    // Call recursively on the current node's children
    for child in &mut node.children {
        pass4(child, while_depth);
    }

    // Execute pass3 function after checking node children
    pass4_post(node, while_depth);
}

fn pass5(node: &mut ASTNode, current_func_returns: &mut String) {
    // Execute pass4 function before checking node children
    pass5_pre(node, current_func_returns);

    // Call recursively on the current node's children
    for child in &mut node.children {
        pass5(child, current_func_returns);
    }

    // Execute pass3 function after checking node children
    pass5_post(node, current_func_returns);
}
