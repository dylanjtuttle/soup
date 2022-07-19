use crate::parser::parser_data::ASTNode;
use crate::semantic::semantic_utils::{is_binary, is_unary};

use crate::code_gen::code_gen_data::*;
use crate::code_gen::code_gen_utils::*;

pub fn gen_asm_main(writer: &mut ASMWriter) {
    // Write ASM main routine (not to be confused with the compilee's main function)
    writer.write("\n        .global _start");
    writer.write("        .balign 4");
    writer.write("_start: stp     x29, x30, [sp, -16]!");
    writer.write("        mov     x29, sp");

    // Branch and link to the compilee's main function
    writer.write("        bl      main1");

    writer.write("end:    ldp     x29, x30, [sp], 16");

    // Exit the program
    writer.write("        mov     x0, 0  // Return code 0");
    writer.write("        mov     x16, 1  // Sys call code to terminate program");
    writer.write("        svc     0x80  // Make system call");
}

pub fn gen_expr(writer: &mut ASMWriter, node: &ASTNode) -> i32 {
    if is_binary(node) {
        // Generate the expressions on either side of the operator, each returned in a register
        let lhs = gen_expr(writer, &node.children[0]);
        let rhs = gen_expr(writer, &node.children[1]);
        let dest = writer.alloc_reg();

        if node.node_type == "=" {
            writer.free_reg(lhs);
            writer.free_reg(dest);
            return rhs;

        } else if node.node_type == "+" {
            writer.write(&format!("        add     w{}, w{}, w{}", dest, lhs, rhs));
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;

        } else if node.node_type == "+=" {
            writer.write(&format!("        add     w{}, w{}, w{}", lhs, lhs, rhs));
            writer.free_reg(dest);
            writer.free_reg(rhs);
            return lhs;

        } else if node.node_type == "-" {
            writer.write(&format!("        sub     w{}, w{}, w{}", dest, lhs, rhs));
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;

        } else if node.node_type == "-=" {
            writer.write(&format!("        sub     w{}, w{}, w{}", lhs, lhs, rhs));
            writer.free_reg(dest);
            writer.free_reg(rhs);
            return lhs;

        } else if node.node_type == "*" {
            writer.write(&format!("        mul     w{}, w{}, w{}", dest, lhs, rhs));
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;

        } else if node.node_type == "*=" {
            writer.write(&format!("        mul     w{}, w{}, w{}", lhs, lhs, rhs));
            writer.free_reg(dest);
            writer.free_reg(rhs);
            return lhs;

        } else if node.node_type == "/" {
            gen_division(writer, node, dest, lhs, rhs);
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;

        } else if node.node_type == "/=" {
            gen_division(writer, node, lhs, lhs, rhs);
            writer.free_reg(dest);
            writer.free_reg(rhs);
            return lhs;

        }

    } else if is_unary(node) {
        // Generate the expression on the right side of the operator, returned in a register
        let rhs = gen_expr(writer, &node.children[0]);

        if node.node_type == "u-" {
            writer.write(&format!("        neg     w{}, w{}", rhs, rhs));
            return rhs;

        } else if node.node_type == "!" {
            writer.write(&format!("        not     w{}, w{}", rhs, rhs));
            return rhs;
        }

    } else if node.node_type == "number" {
        // Allocate a register, move the number into it, and return it
        let reg = writer.alloc_reg();
        writer.write(&format!("        mov     w{}, {}", reg, node.get_attr()));
        return reg;

    } else if node.node_type == "true" {
        let reg = writer.alloc_reg();
        writer.write(&format!("        mov     w{}, 1", reg));
        return reg;

    } else if node.node_type == "false" {
        let reg = writer.alloc_reg();
        writer.write(&format!("        mov     w{}, 0", reg));
        return reg;
        
    } else if node.node_type == "id" {
        // We have a local variable
        let reg = writer.alloc_reg();
        let addr = node.get_sym().borrow().get_addr();
        writer.write(&format!("        ldr     w{}, [sp, {}]", reg, addr));
        return reg;

    } else if node.node_type == "funcCall" {
        gen_func_call(writer, &mut node.clone());
        let reg = writer.alloc_reg();
        writer.write(&format!("        mov     w{}, w0", reg));
        return reg;
    }

    return 0;
}

pub fn gen_division(writer: &mut ASMWriter, node: &ASTNode, dest: i32, lhs: i32, rhs: i32) {
    // Generate labels
    let div_label = writer.new_label();
    let after_label = writer.new_label();

    // If denominator is zero, jump over division to error call
    writer.write(&format!("        cmp     w{}, wzr", rhs));
    writer.write(&format!("        b.eq    {}", div_label));

    // Otherwise, perform division and jump over error
    writer.write(&format!("        sdiv    w{}, w{}, w{}", dest, lhs, rhs));
    writer.write(&format!("        b       {}", after_label));

    // Define error string
    writer.write(&format!("{}:", div_label));
    writer.write(".data");
    writer.write(&format!("div_zero: .string \"Error: Line {}: Division by zero\\n\"", node.get_line_num()));
    writer.write(".align 4");
    writer.write(".text");
    // Call printf
    writer.write("        adrp    x0, div_zero@PAGE");
    writer.write("        add     x0, x0, div_zero@PAGEOFF");
    writer.write("        bl      _printf");
    // Exit the program
    writer.write("        mov     x0, 1  // Return code 1");
    writer.write("        mov     x16, 1  // Sys call code to terminate program");
    writer.write("        svc     0x80  // Make system call");
    // Move on and free registers
    writer.write(&format!("{}:", after_label));
}

pub fn gen_func_call(writer: &mut ASMWriter, node: &mut ASTNode) {
    if node.get_func_name() == "printf" {
        // Get label of string
        let string_label = node.children[1].children[0].children[0].get_sym().borrow().get_label();
        // Generate the printf function call
        func_call_printf(writer, node, &string_label);
    } else {
        // Loop through any arguments and put them in the argument passing registers
        for (i, arg) in node.children[1].children.iter().enumerate() {
            let expr_reg = gen_expr(writer, &arg.children[0]);
            writer.write(&format!("        mov     w{}, w{}", i, expr_reg));
            writer.free_reg(expr_reg);
        }
        writer.write(&format!("        bl      {}1", node.get_sym().borrow().name));
    }
}

pub fn gen_func_enter(writer: &mut ASMWriter, node: &mut ASTNode) {
    // Get number of bytes to allocate on the stack
    let num_bytes = get_func_stack_alloc(node);

    // Calculate and store memory addresses for all local variables defined in this function
    declare_variables(node, &mut 0);

    // Write function entry label
    writer.write(&format!("\n{}1:", node.get_func_name()));
    writer.write("        stp     x29, x30, [sp, -16]!");
    writer.write("        mov     x29, sp");
    writer.write(&format!("        sub     sp, sp, {}", num_bytes));

    // Store any parameters in their assigned memory locations
    for (i, param) in node.children[1].children.iter().enumerate() {
        writer.write(&format!("        str     x{}, [sp, {}]", i, param.get_sym().borrow().get_addr()));
    }
}

pub fn gen_func_exit(writer: &mut ASMWriter, node: &mut ASTNode) {
    // Generate an error message if function is non-void
    if node.get_sym().borrow().returns != "void" {
        // Define error string
        writer.write(".data");
        writer.write(&format!("no_ret: .string \"Error: Line {}: A control path reaches the end of a non-void function without returning a value\\n\"", node.get_line_num()));
        writer.write(".align 4");
        writer.write(".text");
        // Call printf
        writer.write("        adrp    x0, no_ret@PAGE");
        writer.write("        add     x0, x0, no_ret@PAGEOFF");
        writer.write("        bl      _printf");
        // Exit the program
        writer.write("        mov     x0, 1  // Return code 1");
        writer.write("        mov     x16, 1  // Sys call code to terminate program");
        writer.write("        svc     0x80  // Make system call");
    }

    // Get number of bytes to allocate on the stack
    let num_bytes = get_func_stack_alloc(node);

    // Write function exit label
    writer.write(&format!("{}2:", node.get_func_name()));
    writer.write(&format!("        add     sp, sp, {}", num_bytes));
    writer.write("        ldp     x29, x30, [sp], 16");
    writer.write("        ret");
}

pub fn func_call_printf(writer: &mut ASMWriter, node: &ASTNode, string_label: &String) {
    let mut formatting = false;
    writer.write(&format!("        adrp    x0, {}@PAGE", string_label));
    writer.write(&format!("        add     x0, x0, {}@PAGEOFF", string_label));
    for (i, param) in node.children[1].children.iter().enumerate() {
        if i > 0 {
            formatting = true;
            let expr_reg = gen_expr(writer, &param.children[0]);
            if i == 1 {
                writer.write(&format!("        str     w{}, [sp, -32]!", expr_reg));
                increment_addrs(&writer.get_current_func(), 32, &mut vec![]);
            } else {
                writer.write(&format!("        str     w{}, [sp, {}]", expr_reg, (i - 1) * 8));
            }
            writer.free_reg(expr_reg);
        }
    }
    writer.write("        bl      _printf");
    if formatting {
        writer.write("        add     sp, sp, 32");
        increment_addrs(&writer.get_current_func(), -32, &mut vec![]);
    }
}