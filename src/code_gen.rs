use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::ASTNode;
use crate::semantic::Symbol;

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
    if node.node_type == "string" {
        // Add the string, along with a label, to the top of the screen
        write_asm(asm_file, format!("{}: .string \"{}\"", get_label(label), node.get_attr()));
        // Create a symbol table and add it to the string node
        node.add_sym(Rc::new(RefCell::new(Symbol::new(String::from("string"), String::from("string"), String::from("string")))));
        // Keep track of that label for later
        node.get_sym().borrow_mut().label = Some(label.clone());
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

fn traverse_pre(asm_file: &mut File, node: &mut ASTNode, label: &mut String) -> bool {
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        gen_func_enter(asm_file, node);
    }

    if node.node_type == "funcCall" {
        if node.get_func_name() == "printstr" {
            // Get string
            let string = node.children[1].children[0].children[0].get_attr();
            // Get label of string
            let string_label = node.children[1].children[0].children[0].get_sym().borrow().get_label();
            // Generate the printstr function call
            func_call_printstr(asm_file, &string_label, string.len());
        }
    }

    return false;
}

fn traverse_post(asm_file: &mut File, node: &mut ASTNode, label: &mut String) -> bool {
    if node.node_type == "funcDecl" || node.node_type == "mainFuncDecl" {
        gen_func_exit(asm_file, node);
    }

    return false;
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

    // Write function entry label
    write_asm(asm_file, format!("\n{}1:", node.get_func_name()));
    write_asm(asm_file, format!("        stp     x29, x30, [sp, -{}]!", num_bytes));
    write_asm(asm_file, String::from("        mov     x29, sp"));
}

fn gen_func_exit(asm_file: &mut File, node: &mut ASTNode) {
    // Get number of bytes to allocate on the stack
    let num_bytes = get_func_stack_alloc(node);

    // Write function exit label
    write_asm(asm_file, format!("{}2:", node.get_func_name()));
    write_asm(asm_file, format!("        ldp     x29, x30, [sp], {}", num_bytes));
    write_asm(asm_file, String::from("        ret"));
}

fn func_call_printstr(asm_file: &mut File, string_label: &String, string_len: usize) {
    write_asm(asm_file, format!("        adr     x1, {}", string_label));
    write_asm(asm_file, format!("        mov     x2, {}", string_len + 1));
    write_asm(asm_file, String::from("        bl      printstr1"));
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