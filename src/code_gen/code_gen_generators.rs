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
    writer.write(".data");
    writer.write("last_newline: .string \"\\n\"");
    writer.write(".align 4");
    writer.write(".text");
    writer.write("        adrp    x0, last_newline@PAGE");
    writer.write("        add     x0, x0, last_newline@PAGEOFF");
    writer.write("        bl      _printf");
    writer.write("        mov     x0, 0  // Return code 0");
    writer.write("        mov     x16, 1  // Sys call code to terminate program");
    writer.write("        svc     0x80  // Make system call");
}

pub fn gen_runtime_lib(writer: &mut ASMWriter) {
    writer.write("\nexit1:");
    writer.write("// Return code is passed into exit and is already in x0");
    writer.write("        mov     x16, 1  // Sys call code to terminate program");
    writer.write("        svc     0x80  // Make system call");
}

pub fn gen_expr(writer: &mut ASMWriter, node: &ASTNode) -> i32 {
    if is_binary(node) {
        // Generate the expressions on either side of the operator, each returned in a register
        let lhs = gen_expr(writer, &node.children[0]);
        let rhs = gen_expr(writer, &node.children[1]);
        let mut dest = writer.alloc_reg();

        if node.node_type == "=" {
            writer.free_reg(lhs);
            writer.free_reg(dest);
            return rhs;
        } else if node.node_type == "+" || node.node_type == "+=" {
            writer.write(&format!("        add     w{}, w{}, w{}", dest, lhs, rhs));
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;
        } else if node.node_type == "-" || node.node_type == "-=" {
            writer.write(&format!("        sub     w{}, w{}, w{}", dest, lhs, rhs));
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;
        } else if node.node_type == "*" || node.node_type == "*=" {
            writer.write(&format!("        mul     w{}, w{}, w{}", dest, lhs, rhs));
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;
        } else if node.node_type == "/" || node.node_type == "/=" {
            gen_division(writer, node, dest, lhs, rhs);
            writer.free_reg(lhs);
            writer.free_reg(rhs);
            return dest;
        } else if node.node_type == "%" || node.node_type == "%=" {
            gen_division(writer, node, dest, lhs, rhs);
            writer.write(&format!(
                "        msub    w{}, w{}, w{}, w{}",
                lhs, rhs, dest, lhs
            ));
            writer.free_reg(dest);
            writer.free_reg(rhs);
            return lhs;
        } else if node.node_type == "&&" {
            // Since these expressions must be short-circuiting, if the left-hand side is false,
            // then no matter what the right hand side is, the expression will be false, so we don't even need to evaluate it
            let after_label = writer.new_label();
            writer.write(&format!("        cmp     w{}, wzr", lhs));
            writer.write(&format!("        b.eq    {}", after_label));
            writer.free_reg(dest);
            dest = lhs;

            // Otherwise, the left hand side is true, so we can evaluate the right hand side
            writer.write(&format!("        and     w{}, w{}, w{}", dest, lhs, rhs));

            // Write the after label
            writer.write(&format!("        {}:", after_label));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == "||" {
            // Since these expressions must be short-circuiting, if the left-hand side is true,
            // then no matter what the right hand side is, the expression will be true, so we don't even need to evaluate it
            let after_label = writer.new_label();
            writer.write(&format!("        cmp     w{}, 1", lhs));
            writer.write(&format!("        b.eq    {}", after_label));
            writer.free_reg(dest);
            dest = lhs;

            // otherwise, the left hand side is false, so we can evaluate the right hand side
            writer.write(&format!("        orr     w{}, w{}, w{}", dest, lhs, rhs));

            // Write the after label
            writer.write(&format!("        {}:", after_label));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == "==" {
            // dest is 1 if lhs = rhs and 0 otherwise
            writer.write(&format!("        cmp     w{}, w{}", lhs, rhs));
            writer.write(&format!("        cset    w{}, EQ", dest));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == "!=" {
            // dest is 1 if lhs = rhs and 0 otherwise
            writer.write(&format!("        cmp     w{}, w{}", lhs, rhs));
            writer.write(&format!("        cset    w{}, NE", dest));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == "<" {
            // dest is 1 if lhs = rhs and 0 otherwise
            writer.write(&format!("        cmp     w{}, w{}", lhs, rhs));
            writer.write(&format!("        cset    w{}, LT", dest));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == ">" {
            // dest is 1 if lhs = rhs and 0 otherwise
            writer.write(&format!("        cmp     w{}, w{}", lhs, rhs));
            writer.write(&format!("        cset    w{}, GT", dest));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == "<=" {
            // dest is 1 if lhs = rhs and 0 otherwise
            writer.write(&format!("        cmp     w{}, w{}", lhs, rhs));
            writer.write(&format!("        cset    w{}, LE", dest));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
        } else if node.node_type == ">=" {
            // dest is 1 if lhs = rhs and 0 otherwise
            writer.write(&format!("        cmp     w{}, w{}", lhs, rhs));
            writer.write(&format!("        cset    w{}, GE", dest));
            writer.free_reg(lhs);
            writer.free_reg(rhs);

            return dest;
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
        // We have either a local variable or a global variable

        // Allocate a register to store the value of the variable in
        let reg = writer.alloc_reg();

        // To check which one, we can simply find out if the variable's symbol table entry has an addr or a label
        match node.get_sym().borrow().addr {
            Some(addr) => {
                // We have a local variable, so we can load the value at its address
                writer.write(&format!("        ldr     w{}, [sp, {}]", reg, addr));
                return reg;
            }
            None => {
                // We have a global variable, so get the value stored at its label
                writer.write(&format!(
                    "        adrp    x8, {}@PAGE",
                    node.get_sym().borrow().get_label()
                ));
                writer.write(&format!(
                    "        add     x8, x8, {}@PAGEOFF",
                    node.get_sym().borrow().get_label()
                ));
                writer.write(&format!("        ldr     w{}, [x8]", reg));
                return reg;
            }
        }
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
    let div_zero_label = writer.new_label();
    writer.write(&format!(
        "{}: .string \"Error: Line {}: Division by zero\\n\"",
        div_zero_label,
        node.get_line_num()
    ));
    writer.write(".align 4");
    writer.write(".text");
    // Call printf
    writer.write(&format!("        adrp    x0, {}@PAGE", div_zero_label));
    writer.write(&format!(
        "        add     x0, x0, {}@PAGEOFF",
        div_zero_label
    ));
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
        let string_label = node.children[1].children[0].children[0]
            .get_sym()
            .borrow()
            .get_label();

        // Generate the printf function call
        func_call_printf(writer, node, &string_label);
    } else {
        // Check how many arguments we want to pass
        let num_args = node.children[1].children.len();

        // There are 8 argument passing registers, r0 - r7, so if there are more arguments than that,
        // we need to allocate extra space on the stack for them
        if num_args > 8 {
            let extra_space = ((num_args - 8) * 4) as i32;
            // Allocate enough space on the stack, and adjust the addresses of the local variables accordingly
            allocate_stack(writer, extra_space);
        }

        // Loop through any arguments and pass them using the correct method
        for (i, arg) in node.children[1].children.iter().enumerate() {
            let expr_reg = gen_expr(writer, &arg.children[0]);

            // If the argument number is less than 8, just put it in the corresponding argument passing register
            if i < 8 {
                writer.write(&format!("        mov     w{}, w{}", i, expr_reg));
            } else {
                // Otherwise, place it on the stack at offset (i - 8) * 4
                // (for example, argument 8 will be stored at sp + 0, argument 9 at sp + 4, etc...)
                writer.write(&format!(
                    "        str     w{}, [sp, {}]",
                    expr_reg,
                    (i - 8) * 4
                ));
            }

            writer.free_reg(expr_reg);
        }

        // If there are any actively allocated caller-stored registers,
        // they could get trampled by the function we are calling, so we have to store them
        let active_caller = writer.get_allocated_caller_saved_registers();
        // Allocate space on the stack and temporarily store the registers on that allocated space
        allocate_stack(writer, (active_caller.len() * 4) as i32);
        for (i, reg) in active_caller.iter().enumerate() {
            writer.write(&format!("        str     w{}, [sp, {}]", reg, i * 4));
        }
        if active_caller.len() > 0 {
            // If we are storing any of these registers, the callee will need to know about it since we've moved the
            // stack pointer away from any possible arguments stored on the stack
            node.get_sym().borrow_mut().stored_bytes = (active_caller.len() * 4) as i32;
        }

        writer.write(&format!(
            "        bl      {}1",
            node.get_sym().borrow().name
        ));

        for (i, reg) in active_caller.iter().enumerate() {
            writer.write(&format!("        ldr     w{}, [sp, {}]", reg, i * 4));
        }
        if active_caller.len() > 0 {
            // If we stored any caller-saved registers, deallocate the space on the stack
            allocate_stack(writer, -((active_caller.len() * 4) as i32));
        }

        // If we cleared extra space, we have to deallocate it after the function call
        if num_args > 8 {
            let extra_space = ((num_args - 8) * 4) as i32;
            // Deallocate space on the stack, and adjust the addresses of the local variables accordingly
            allocate_stack(writer, -extra_space);
        }
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
    if num_bytes != 0 {
        writer.write(&format!("        sub     sp, sp, {}", num_bytes));
    }

    // Store any parameters in their assigned memory locations
    for (i, param) in node.children[1].children.iter().enumerate() {
        // If the parameter number is less than 8, it is stored in an argument passing register
        if i < 8 {
            writer.write(&format!(
                "        str     w{}, [sp, {}]",
                i,
                param.get_sym().borrow().get_addr()
            ));
        } else {
            // Otherwise, it is stored on the stack
            let temp_reg = writer.alloc_reg();
            // Get the amount of space we need to consider that is used to store saved caller-saved registers
            let caller_bytes = node.get_sym().borrow().stored_bytes;
            writer.write(&format!(
                "        ldr     w{}, [sp, {}]",
                temp_reg,
                ((i - 8) * 4) + 16 + ((num_bytes + caller_bytes) as usize)
            ));
            writer.write(&format!(
                "        str     w{}, [sp, {}]",
                temp_reg,
                param.get_sym().borrow().get_addr()
            ));
            writer.free_reg(temp_reg);
        }
    }

    // We no longer need to keep track of the amount of space allocated for caller-saved registers
    node.get_sym().borrow_mut().stored_bytes = 0;

    // Now, if there are any callee-saved registers currently allocated, it is the job of the callee to save them
    let mut active_callee = writer.get_allocated_callee_saved_registers();
    // Keep track of this for when we're exiting the function
    node.get_sym()
        .borrow_mut()
        .active_callee_saved
        .append(&mut active_callee);

    allocate_stack(writer, (active_callee.len() * 4) as i32);

    for (i, reg) in active_callee.iter().enumerate() {
        writer.write(&format!("        str     w{}, [sp, {}]", reg, i * 4));
    }
}

pub fn gen_func_exit(writer: &mut ASMWriter, node: &mut ASTNode) {
    // Generate an error message if function is non-void
    if node.get_sym().borrow().returns != "void" {
        // Define error string
        writer.write(".data");
        let no_ret_label = writer.new_label();
        writer.write(&format!("{}: .string \"Error: Line {}: A control path reaches the end of a non-void function without returning a value\\n\"", no_ret_label, node.get_line_num()));
        writer.write(".align 4");
        writer.write(".text");
        // Call printf
        writer.write(&format!("        adrp    x0, {}@PAGE", no_ret_label));
        writer.write(&format!("        add     x0, x0, {}@PAGEOFF", no_ret_label));
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

    // If there are any callee-saved registers currently saved, we have to restore them
    let active_callee = node.get_sym().borrow().get_active_callees();
    for (i, reg) in active_callee.iter().enumerate() {
        writer.write(&format!("        ldr     w{}, [sp, {}]", reg, i * 4));
    }
    allocate_stack(writer, -((active_callee.len() * 4) as i32));

    if num_bytes != 0 {
        writer.write(&format!("        add     sp, sp, {}", num_bytes));
    }
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
                writer.write(&format!(
                    "        str     w{}, [sp, {}]",
                    expr_reg,
                    (i - 1) * 8
                ));
            }
            writer.free_reg(expr_reg);
        }
    }
    writer.write("        bl      _printf");
    if formatting {
        // Deallocate space on the stack for the printf arguments
        allocate_stack(writer, -32);
    }
}
