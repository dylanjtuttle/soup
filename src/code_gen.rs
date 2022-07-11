use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::ASTNode;
use crate::semantic::Symbol;
use crate::throw_error;

// -----------------------------------------------------------------
// CODE GENERATOR
// -----------------------------------------------------------------

pub fn code_gen(asm_filename: &str, ast: &mut ASTNode) {
    // Open the file to put the assembly code in, and panic if it can't be opened
    let mut asm_file = match File::create(asm_filename) {
        Ok(asm_file) => asm_file,
        Err(_) => panic!("Uh Oh, I can't make an assembly file. Oh well, goodbye!")
    };

    let mut label = String::from("L0");

    // First, before we write any code, find all the strings and add them to the top of the file
    gen_strings(&mut asm_file, ast, &mut label);

    // ----------------------------------------------------------------------------------
    // Write ASM main routine (not to be confused with the compilee's main function)
    write_asm(&mut asm_file, String::from("\n        .global _start"));
    write_asm(&mut asm_file, String::from("        .balign 4"));
    write_asm(&mut asm_file, String::from("_start: stp     x29, x30, [sp, -16]!"));
    write_asm(&mut asm_file, String::from("        mov     x29, sp"));

    // Branch and link to the compilee's main function
    write_asm(&mut asm_file, String::from("        bl      main1"));

    write_asm(&mut asm_file, String::from("end:    ldp     x29, x30, [sp], 16"));
    write_asm(&mut asm_file, String::from("        ret"));
    // ----------------------------------------------------------------------------------

    // Begin traversing the AST and generating code
    traverse_prune(&mut asm_file, ast, &mut label);

    // Append the code for the runtime library at the end of the file
    runtime_lib(&mut asm_file);
}

// -----------------------------------------------------------------
// TRAVERSALS
// -----------------------------------------------------------------

fn gen_strings(asm_file: &mut File, node: &mut ASTNode, label: &mut String) {
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
        write_asm(asm_file, format!("{}: .string \"{}\"", get_label(label), new_string));
        // Update the version in the AST
        node.children[1].children[0].children[0].attr = Some(new_string);
        // Create a symbol table and add it to the string node
        node.children[1].children[0].children[0].add_sym(Rc::new(RefCell::new(Symbol::new(String::from("string"), String::from("string"), String::from("string")))));
        // Keep track of that label for later
        node.children[1].children[0].children[0].get_sym().borrow_mut().label = Some(label.clone());
    }

    // Visit children
    for child in &mut node.children {
        gen_strings(asm_file, child, label);
    }
}

fn traverse_prune(asm_file: &mut File, node: &mut ASTNode, label: &mut String) {
    // Do something with the node before you visit its children,
    // and possibly return without visiting children if do_prune = true
    let do_prune = traverse_pre(asm_file, node, label);

    if do_prune {
        return;
    }

    // Visit children
    for child in &mut node.children {
        traverse_prune(asm_file, child, label);
    }

    // Do something again with the node
    traverse_post(asm_file, node, label);
}

fn traverse_pre(asm_file: &mut File, node: &mut ASTNode, _label: &mut String) -> bool {
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        gen_func_enter(asm_file, node);
    }

    if node.node_type == "funcCall" {
        if node.get_func_name() == "printf" {
            // Get label of string
            let string_label = node.children[1].children[0].children[0].get_sym().borrow().get_label();
            // Generate the printf function call
            func_call_printf(asm_file, node, &string_label);
        }
    }

    return false;
}

fn traverse_post(asm_file: &mut File, node: &mut ASTNode, _label: &mut String) -> bool {
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        gen_func_exit(asm_file, node);
    }

    return false;
}

fn declare_variables(node: &mut ASTNode, current_offset: &mut i32) {
    if node.node_type == "parameter" || node.node_type == "varDecl" {
        // Add the local variable's offset to its symbol table entry
        node.get_sym().borrow_mut().addr = Some(format!("[sp, {}]", *current_offset));

        // Increment the current offset by the size of this local variable for next time
        *current_offset += 4;
    }

    // Visit children
    for child in &mut node.children {
        declare_variables(child, current_offset);
    }
}

// Calculate the number of bytes a function needs to allocate on the stack
fn get_func_stack_alloc(node: &ASTNode) -> i32 {
    // Calculate the number of bytes we need to allocate on the stack for local variables
    let mut var_alloc = get_func_var_alloc(node);

    // Add 16 bytes for the stack frame
    var_alloc += 16;

    // If the number of bytes isn't doubleword aligned, add 4 pad bytes to align it
    if var_alloc % 8 != 0 {
        var_alloc += 4;
    }

    return var_alloc;
}

// Calculate the number of bytes a function needs to allocate on the stack for its local variables
fn get_func_var_alloc(node: &ASTNode) -> i32 {
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

fn gen_func_enter(asm_file: &mut File, node: &mut ASTNode) {
    // Get number of bytes to allocate on the stack
    let num_bytes = get_func_stack_alloc(node);

    // Calculate and store memory addresses for all local variables defined in this function
    declare_variables(node, &mut 0);

    // Write function entry label
    write_asm(asm_file, format!("\n{}1:", node.get_func_name()));
    write_asm(asm_file, format!("        stp     x29, x30, [sp, -{}]!", num_bytes));
    write_asm(asm_file, String::from("        mov     x29, sp"));

    // Store any parameters in their assigned memory locations
    for (i, param) in node.children[1].children.iter().enumerate() {
        write_asm(asm_file, format!("        str     x{}, {}", i, param.get_sym().borrow().get_addr()));
    }
}

fn gen_func_exit(asm_file: &mut File, node: &mut ASTNode) {
    // Get number of bytes to allocate on the stack
    let num_bytes = get_func_stack_alloc(node);

    // Write function exit label
    write_asm(asm_file, format!("{}2:", node.get_func_name()));
    write_asm(asm_file, format!("        ldp     x29, x30, [sp], {}", num_bytes));
    write_asm(asm_file, String::from("        ret"));
}

fn func_call_printf(asm_file: &mut File, node: &ASTNode, string_label: &String) {
    let mut formatting = false;
    write_asm(asm_file, format!("        adrp    x0, {}@PAGE", string_label));
    write_asm(asm_file, format!("        add     x0, x0, {}@PAGEOFF", string_label));
    for (i, param) in node.children[1].children.iter().enumerate() {
        if i > 0 {
            formatting = true;
            write_asm(asm_file, format!("        mov     w{}, {}", i, param.children[0].get_attr()));
            if i == 1 {
                write_asm(asm_file, format!("        str     w{}, [sp, -32]!", i));
            } else {
                write_asm(asm_file, format!("        str     w{}, [sp, {}]", i, (i - 1) * 8));
            }
        }
    }
    write_asm(asm_file, String::from("        bl      _printf"));
    if formatting {
        write_asm(asm_file, format!("        add     sp, sp, 32"));
    }
}

fn get_label(label: &mut String) -> String {
    // Get number of current label
    let mut label_num = label[1..].to_string().parse::<u64>().unwrap();

    // Increment label number by one
    label_num += 1;

    // Update label
    *label = format!("L{}", label_num);

    // Return the label, just for fun
    return label.clone();
}

fn runtime_lib(asm_file: &mut File) {
    // printstr()
    write_asm(asm_file, String::from("\nprintstr1:"));
    write_asm(asm_file, String::from("        stp     x29, x30, [sp, -16]!"));
    write_asm(asm_file, String::from("        mov     x29, sp"));
    write_asm(asm_file, String::from("        mov     x0, 1  // stdout = 1"));
    write_asm(asm_file, String::from("        mov     x16, 4  // Unix write syscall"));
    write_asm(asm_file, String::from("        svc     0x80  // Execute syscall"));
    write_asm(asm_file, String::from("printstr2:"));
    write_asm(asm_file, String::from("        ldp     x29, x30, [sp], 16"));
    write_asm(asm_file, String::from("        ret"));
}

fn write_asm(asm_file: &mut File, line: String) {
    // Print line as well so we can see the output in stdout
    println!("{}", line);

    // Attempt to write the line (with a bonus newline at the end), and panic if unable to
    match write!(asm_file, "{}\n", line) {
        Ok(()) => {}
        Err(_) => panic!("Unable to write to ASM file! Quitting now, sorry!")
    };
}