use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::parser_data::ASTNode;
use crate::semantic::semantic_data::Symbol;
use crate::throw_error;

use crate::code_gen::code_gen_data::*;
use crate::code_gen::code_gen_generators::*;

use super::code_gen_traversals::traverse_prune;

pub fn traverse_pre(writer: &mut ASMWriter, node: &mut ASTNode) -> bool {
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        gen_func_enter(writer, node);
        writer.enter_func(node);
    }

    if node.node_type == "="
    || node.node_type == "+="
    || node.node_type == "-="
    || node.node_type == "*="
    || node.node_type == "/="
    || node.node_type == "%=" {
        // Get the value of the expression on the right hand side of this assignment in a register
        let rhs_reg = gen_expr(writer, node);

        // The left hand side of this assignment is either a local variable or a global variable
        // To check which one, we can simply find out if the variable's symbol table entry has an addr or a label
        match node.children[0].get_sym().borrow().addr {
            Some(addr) => {
                // We have a local variable, so we can store the result of the expression at its address
                writer.write(&format!("        str     w{}, [sp, {}]", rhs_reg, addr));
            }
            None => {
                // We have a global variable, so we can store the result of the expression at its label
                writer.write(&format!("        adrp    x8, {}@PAGE", node.children[0].get_sym().borrow().get_label()));
                writer.write(&format!("        add     x8, x8, {}@PAGEOFF", node.children[0].get_sym().borrow().get_label()));
                writer.write(&format!("        str     w{}, [x8]", rhs_reg));
                writer.free_reg(rhs_reg);
            }
        }

        // Free the register holding the result of the expression on the right hand side of the assignment
        writer.free_reg(rhs_reg);

        return true;
    }

    if node.node_type == "funcCall" {
        gen_func_call(writer, node);
    }

    if node.node_type == "return" {
        if node.children.len() > 0 {
            // If we have a non-empty return statement, generate the expression and store it in the function return register
            let expr = gen_expr(writer, &mut node.children[0]);

            writer.write(&format!("        mov     w0, w{}", expr));
            writer.free_reg(expr);

            // Jump to the function exit
            writer.write(&format!("        b       {}2", writer.get_current_func_name()));
            return true;
        }
    }

    if node.node_type == "if" {
        // Generate label to jump to if condition is not satisfied
        let after_label = writer.new_label();

        // First, we need to evaluate the expression
        let expr_reg = gen_expr(writer, &mut node.children[0]);

        // Branch if equal to zero (false)
        writer.write(&format!("        cmp     w{}, wzr", expr_reg));
        writer.write(&format!("        b.eq    {}", after_label));

        // Evaluate if block
        traverse_prune(writer, &mut node.children[1]);

        // Evaluate stuff after the if block (by exiting out of this traversal)
        writer.write(&format!("        {}:", after_label));  // Write after label
        return true;
    }

    if node.node_type == "ifElse" {
        // Generate else and after labels to jump to
        let else_label = writer.new_label();
        let after_label = writer.new_label();

        // First, we need to evaluate the expression
        let expr_reg = gen_expr(writer, &mut node.children[0]);

        // Branch to else block if equal to zero (false)
        writer.write(&format!("        cmp     w{}, wzr", expr_reg));
        writer.write(&format!("        b.eq    {}", else_label));

        // Evaluate if block
        traverse_prune(writer, &mut node.children[1]);
        
        // Branch past else block to after label
        writer.write(&format!("        b       {}", after_label));

        // Evaluate else block
        writer.write(&format!("        {}:", else_label));  // Write else label
        traverse_prune(writer, &mut node.children[2]);

        // Evaluate stuff after the if-else block (by exiting out of this traversal)
        writer.write(&format!("        {}:", after_label));  // Write after label
        return true;
    }

    if node.node_type == "while" {
        // Generate labels for while condition test and after loop
        let test_label = writer.new_label();
        let after_label = writer.new_label();

        // First, we need to evaluate the expression
        writer.write(&format!("        {}:", test_label));
        let expr_reg = gen_expr(writer, &mut node.children[0]);

        // Branch to after loop if equal to zero (false)
        writer.write(&format!("        cmp     w{}, wzr", expr_reg));
        writer.write(&format!("        b.eq    {}", after_label));

        // We are about to start evaluating the body of the while loop,
        // so store the current after label so we can jump to it if we find a break statement
        writer.while_labels.push(after_label.clone());

        // Evaluate the body of the while loop
        traverse_prune(writer, &mut node.children[1]);

        // Loop back up to the while condition
        writer.write(&format!("        b       {}", test_label));

        // Evaluate stuff after the while (by exiting out of this traversal)
        writer.while_labels.pop();
        writer.write(&format!("        {}:", after_label));  // Write after label
        return true;
    }

    if node.node_type == "break" {
        // Branch to the after label of the current while loop
        let while_labels = writer.while_labels.clone();

        let after_label = match while_labels.last() {
            None => {String::from("")}
            Some(after_label) => {after_label.clone()}
        };

        writer.write(&format!("        b       {}", after_label));
    }

    return false;
}

pub fn traverse_post(writer: &mut ASMWriter, node: &mut ASTNode) -> bool {
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        gen_func_exit(writer, node);
        writer.exit_func();
    }

    return false;
}

pub fn global_data(writer: &mut ASMWriter, node: &mut ASTNode) {
    // Generate code for global variables
    if node.node_type == "globVarDecl" {
        // Define a label for the global variable
        let global_label = writer.new_label();

        // Initialize the global variable to zero (works for ints and bools!)
        writer.write(&format!("{}: .word 0", global_label));

        // Store the label in the variable's symbol table
        node.get_sym().borrow_mut().label = Some(global_label);
    }

    // Generate code, handle errors for strings
    if node.node_type == "funcCall" && node.get_func_name() == "printf" {
        let mut num_formatters = 0;
        let fstring = node.children[1].children[0].children[0].get_attr();

        let mut new_string = String::from("");
        let mut skip = false;

        for (i, char) in node.children[1].children[0].children[0].get_attr().chars().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            // If we find a backslash, we prepare to push an escaped character to the new string
            if char == '\\' {
                let next_char = fstring.as_bytes()[i + 1] as char;
                match next_char {
                    'n' => {new_string.push_str("\\n"); skip = true;}
                    't' => {new_string.push_str("\\t"); skip = true;}
                    'r' => {new_string.push_str("\\r"); skip = true;}
                    '\'' => {new_string.push_str("\\'"); skip = true;}
                    '\"' => {new_string.push_str("\\\""); skip = true;}
                    '\\' => {new_string.push_str("\\\\"); skip = true;}
                    '{' => {new_string.push('{'); skip = true;}
                    '}' => {new_string.push('}'); skip = true;}
                    _ => {throw_error(&format!("Line {}: Invalid escape character '{}'", node.get_line_num(), next_char))}
                }
            } else if char == '{' {
                // We are probably seeing the beginning of a formatter
                if i == fstring.len() - 1 || fstring.as_bytes()[i + 1] as char != '}' {
                    throw_error(&format!("Line {}: Invalid formatter, opening {{ without a closing }}, did you mean \"\\{{\"?",
                                              node.get_line_num()));
                }
                if fstring.as_bytes()[i + 1] as char == '}' {
                    num_formatters += 1;

                    if num_formatters == 6 {
                        throw_error(&format!("Line {}: printf only accepts 5 format arguments",
                                                 node.get_line_num()));
                    }

                    // Now we need to figure out what the type of the value being passed into the formatter is
                    // First check to see if there are enough arguments passed in to match the current amount of formatters
                    if node.children[1].children.len() - 1 < num_formatters {
                        throw_error(&format!("Line {}: {} formatter(s) given to printf, but only {} format argument(s) passed in",
                                                 node.get_line_num(), num_formatters, node.children[1].children.len() - 1));
                    } else {
                        let value = &node.children[1].children[num_formatters].children[0];

                        if value.get_type() == "int" {
                            new_string.push_str("%d");
                            skip = true;
                        } else {
                            throw_error(&format!("Line {}: Invalid format type '{}' passed into printf, must only be int",
                                                      node.get_line_num(), value.get_type()));
                        }
                    }
                }
            } else if char == '}' {
                throw_error(&format!("Line {}: Invalid formatter, closing }} without an opening {{, did you mean \"\\}}\"?",
                                          node.get_line_num()));
            } else {
                new_string.push(char);
            }
        }

        // Check if too many format arguments were passed into printf
        if node.children[1].children.len() - 1 != num_formatters {
            throw_error(&format!("Line {}: {} format argument(s) passed into to printf, but only {} formatter(s) given",
                                                 node.get_line_num(), node.children[1].children.len() - 1, num_formatters));
        }

        // new_string has successfully been formed, so we can store it for printing later
        let label = writer.new_label();
        writer.write(&format!("{}: .string \"{}\"", label, new_string));
        // Update the version in the AST
        node.children[1].children[0].children[0].attr = Some(new_string);
        // Create a symbol table and add it to the string node
        node.children[1].children[0].children[0].add_sym(Rc::new(RefCell::new(Symbol::new(String::from("string"), String::from("string"), String::from("string")))));
        // Keep track of that label for later
        node.children[1].children[0].children[0].get_sym().borrow_mut().label = Some(label);
    }
}