use std::rc::Rc;

use crate::parser::parser_data::ASTNode;
use crate::semantic::semantic_data::*;
use crate::semantic::semantic_utils::{is_binary, is_unary};
use crate::throw_error;

// ----------------------------------------------------------------------------------------------------
// PASS 1
// ----------------------------------------------------------------------------------------------------

pub fn pass1_post(node: &mut ASTNode, scope_stack: &mut ScopeStack, num_main_decls: &mut i32) {
    let node_type = &node.node_type.clone();

    if node_type == "mainFuncDecl" {
        // Create a symbol for the main declaration
        let main_symbol = Symbol::new(String::from("main"), String::from("f()"), String::from("void"));

        // Insert symbol into scope stack and AST node
        insert_symbol(main_symbol, scope_stack, node);

        // Keep track of the number of main declarations
        *num_main_decls += 1;

    } else if node_type == "funcDecl" {
        // Get fields from the AST
        let func_name = &node.children[0].get_attr();
        let func_sig = node.get_func_sig();
        let func_returns = node.children[2].children[0].node_type.clone();

        // Create a symbol for the function declaration
        let func_symbol = Symbol::new(func_name.clone(), func_sig, func_returns);

        // Insert symbol into scope stack and AST node
        insert_symbol(func_symbol, scope_stack, node);

    } else if node_type == "globVarDecl" {
        // Get fields from the AST
        let var_name = &node.children[1].get_attr();
        let var_type = node.children[0].node_type.clone();
        let var_returns = var_type.clone();

        // Create a symbol for the variable declaration
        let var_symbol = Symbol::new(var_name.clone(), var_type, var_returns);

        // Insert symbol into scope stack and AST node
        insert_symbol(var_symbol, scope_stack, node);
    }
}

// ----------------------------------------------------------------------------------------------------
// PASS 2
// ----------------------------------------------------------------------------------------------------

pub fn pass2_pre(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    if node.node_type == "funcDecl" ||
       node.node_type == "mainFuncDecl" ||
       node.node_type == "if" ||
       node.node_type == "ifElse" ||
       node.node_type == "while" {
        // Open up a new scope
        scope_stack.open_scope();

    } else if node.node_type == "varDecl" {
        // Variables can only be defined in the global or function scopes (scope levels 2 and 3)
        if scope_stack.scope_level() > 3 {
            throw_error(&format!("Line {}: Variables can only be defined in the outermost scope of a function or globally (i.e. not in an if statement, while loop, etc.)",
                                      node.get_line_num()))
        }

        // Check if a variable with this name has already been defined in this scope
        if scope_stack.find_in_scope(&&node.children[1].get_attr()) {
            // A variable with this name has been defined already in this scope
            throw_error(&format!("Line {}: Variable illegally redefined within the same scope",
                                      node.get_line_num()));
        } else {
            // This variable hasn't been defined yet in this scope, so we can proceed to define it in our symbol table
            let var_name = node.children[1].get_attr();
            let var_type = node.children[0].get_type();
            
            let var_symbol = Symbol::new(var_name.clone(),
                                                 var_type.clone(),
                                                 var_type);
            
            // Insert symbol into scope stack and AST node
            insert_symbol(var_symbol, scope_stack, node);
        }

    } else if node.node_type == "parameter" {
        // Parameters are essentially identical to local variables
        let param_name = node.children[1].get_attr();
        let param_type = node.children[0].node_type.clone();
        
        let param_symbol = Symbol::new(param_name.clone(),
                                               param_type.clone(),
                                               param_type);

        // Insert symbol into scope stack and AST node
        insert_symbol(param_symbol, scope_stack, node);

    } else if node.node_type == "id" {
        match scope_stack.find_symbol(&node.get_attr()) {
            // If we can't find the identifier, we haven't defined it yet
            None => {
                throw_error(&format!("Line {}: Unknown identifier '{}'",
                                          node.get_line_num(), node.get_attr()))
            }
            Some(symbol) => {
                // This identifier exists already, so we already know what it returns and what its symbol table is
                node.type_sig = Some(symbol.borrow().type_sig.clone());
                node.sym = Some(Rc::clone(&symbol));
            }
        }

    } else if node.node_type == "number" {
        node.type_sig = Some(String::from("int"));

    } else if node.node_type == "true" || node.node_type == "false" {
        node.type_sig = Some(String::from("bool"));
    }
}

pub fn pass2_post(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    if node.node_type == "funcDecl" ||
       node.node_type == "mainFuncDecl" ||
       node.node_type == "if" ||
       node.node_type == "ifElse" ||
       node.node_type == "while" {
        // Close the topmost scope
        scope_stack.close_scope();

    }
}

// ----------------------------------------------------------------------------------------------------
// PASS 3
// ----------------------------------------------------------------------------------------------------

pub fn pass3_post(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    if is_binary(node) {
        let left_type = node.children[0].get_type();
        let right_type = node.children[1].get_type();

        // Both sides of a binary operation must have the same type
        if left_type != right_type {
            throw_error(&format!("Line {}: Type mismatch for {}, operands must have same type",
                                      node.get_line_num(), node.node_type))
        } else {
            // Types match, but we need to check if the types (even if they match) make sense with the operation
            if node.node_type == "&&" || node.node_type == "||" {
                // Both operands must be bools, returns a bool
                if left_type == "bool" && right_type == "bool" {
                    // Type check is successful
                    node.type_sig = Some(String::from("bool"));
                } else {
                    throw_error(&format!("Line {}: Type mismatch for {}, operands must be bools",
                                              node.get_line_num(), node.node_type))
                }

            } else if node.node_type == "==" || node.node_type == "!=" {
                // Operands can be either ints or bools, returns a bool
                node.type_sig = Some(String::from("bool"));

            } else if node.node_type == "<" || node.node_type == ">" || node.node_type == "<=" || node.node_type == ">=" {
                // Both operands must be ints, returns a bool
                if left_type == "int" && right_type == "int" {
                    // Type check is successful
                    node.type_sig = Some(String::from("bool"));
                } else {
                    throw_error(&format!("Line {}: Type mismatch for {}, operands must be ints",
                                              node.get_line_num(), node.node_type))
                }

            } else if node.node_type == "=" {
                // Operands can be either ints or bools, returns whatever type the operands are
                node.type_sig = Some(left_type);

            } else {
                // One of + += - -= * *= / /= % %=
                // Both operands must be ints, returns an int
                if left_type == "int" && right_type == "int" {
                    // Type check is successful
                    node.type_sig = Some(String::from("int"));
                } else {
                    throw_error(&format!("Line {}: Type mismatch for {}, operands must be ints",
                                              node.get_line_num(), node.node_type))
                }
            }
        }

    } else if is_unary(&node) {
        let op_type = node.children[0].get_type();
        if node.node_type == "u-" {
            // Operand must be int, returns an int
            if op_type == "int" {
                // Type check is successful
                node.type_sig = Some(String::from("int"));
            } else {
                throw_error(&format!("Line {}: Type mismatch for -, operand must be int",
                                          node.get_line_num()))
            }
        } else {
            // !
            // Operand must be bool, returns a bool
            if op_type == "bool" {
                // Type check is successful
                node.type_sig = Some(String::from("bool"));
            } else {
                throw_error(&format!("Line {}: Type mismatch for {}, operand must be bool",
                                          node.get_line_num(), node.node_type))
            }
        }

    } else if node.node_type == "funcCall" {
        let func_name = node.children[0].get_attr();

        // Get type signature of function call
        let func_sig = node.get_func_sig();

        // Add func sig to type_sig of ASTNode
        node.type_sig = Some(func_sig.clone());

        // Try to find the function being called
        match scope_stack.find_symbol(&func_name) {
            None => {
                throw_error(&format!("Line {}: Unknown identifier '{}'",
                                          node.get_line_num(), func_name))
            }
            Some(symbol) => {
                // Make sure the func sig of the found function matches our function call
                if symbol.borrow().type_sig != func_sig {
                    // If the function declaration is printf, the func sigs don't have to match as long as...
                    if symbol.borrow().type_sig == "f(string, ...)" {
                        // Our function call sig begins with a string argument
                        if func_sig.contains("f(string") {
                            node.type_sig = Some(symbol.borrow().returns.clone());
                            node.sym = Some(symbol.clone());
                        } else {
                            throw_error(&format!("Line {}: First argument passed into 'printf' must be a string literal",
                                                      node.get_line_num()))
                        }
                    } else {
                        throw_error(&format!("Line {}: Argument(s) for invocation of function '{}' do not match parameter(s)",
                                                  node.get_line_num(), func_name))
                    }
                } else {
                    node.type_sig = Some(symbol.borrow().returns.clone());
                    node.sym = Some(symbol.clone());
                }
            }
        }

    } else if node.node_type == "return" {
        if node.children.len() == 0 {
            // If the return statement is empty, set its type signature to "void"
            node.type_sig = Some(String::from("void"));
        } else {
            // Otherwise, pass the type of the expression being returned up to the return node
            node.type_sig = Some(node.children[0].get_type());
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// PASS 4
// ----------------------------------------------------------------------------------------------------

pub fn pass4_pre(node: &mut ASTNode, while_depth: &mut i32) {
    if node.node_type == "while" {
        *while_depth += 1;
    }

    // Break statement must be within a while loop
    if node.node_type == "break" {
        if *while_depth == 0 {
            throw_error(&format!("Line {}: break statement must be within a while loop",
                                      node.get_line_num()))
        }
    }

    // An if- or while-condition must be of Boolean type
    if node.node_type == "if" || node.node_type == "ifElse" || node.node_type == "while" {
        // The condition is the first child of the if/if-else/while
        if node.children[0].get_type() != "bool" {
            // Simply for the error statement, so that it can specify whether it was
            // an if or while condition that caused the error
            let node_type = match &node.node_type {
                w if w == "while" => "while",
                _ => "if"
            };

            throw_error(&format!("Line {}: {} condition must be of boolean type",
                                      node.get_line_num(), node_type));
        }
    }
}

pub fn pass4_post(node: &mut ASTNode, while_depth: &mut i32) {
    if node.node_type == "while" {
        *while_depth -= 1;
    }
}

// ----------------------------------------------------------------------------------------------------
// PASS 5
// ----------------------------------------------------------------------------------------------------

pub fn pass5_pre(node: &mut ASTNode, current_func_returns: &mut String) {
    // If we're entering into a function, make note of its return type
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        *current_func_returns = node.get_type();

        if node.get_type() != "void" && !node.has_nonempty_return() {
            // If this is a non-void function, it must return a value.
            // Thus, if it does not have a non-empty return statement,
            // that is, a return statement that actually returns a value, that is an error
            throw_error(&format!("Line {}: Non-void function '{}' must return a value",
                                      node.get_line_num(), node.children[0].get_attr()));
        }
    }

    if node.node_type == "return" {
        if node.get_type() != "void" {
            // We have a non-empty return statement
            if current_func_returns == "void" {
                // A void function can't return a value
                throw_error(&format!("Line {}: Void function cannot return a value",
                                         node.get_line_num()));

            } else if *current_func_returns != node.get_type() {
                // If we're in a non-void function, we have to be returning a value with the same type
                throw_error(&format!("Line {}: Function is supposed to return {}, but returns {} instead",
                                         node.get_line_num(), current_func_returns, node.get_type()));
            }
        } else {
            // We have an empty return statement
            if current_func_returns != "void" {
                throw_error(&format!("Line {}: Non-void function must return a value",
                                         node.get_line_num()));
            }
        }
    }
}

pub fn pass5_post(node: &mut ASTNode, current_func_returns: &mut String) {
    // If we're leaving a function, set the return type back to "None"
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        *current_func_returns = String::from("None");
    }
}