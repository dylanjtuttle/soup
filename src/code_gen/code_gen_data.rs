use std::fs::File;
use std::io::prelude::*;

use crate::parser::parser_data::ASTNode;
use crate::throw_error;

pub struct ASMWriter {
    pub file: File,
    pub label: String,
    pub regs: Vec<i32>,
    pub current_func: Option<ASTNode>,
    pub while_labels: Vec<String>,
}

impl ASMWriter {
    pub fn new(filename: &str) -> ASMWriter {
        // Open up the file with the given filename
        let asm_file = match File::create(filename) {
            Ok(asm_file) => asm_file,
            Err(_) => panic!("Uh Oh, I can't make an assembly file. Oh well, goodbye!"),
        };

        // Initialize label
        let label = String::from("L0");

        // Initialize the initial state of the registers
        //          <---- r9 - r15 --->              <-------- r19 - r28 ------->
        let regs = vec![
            0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        // 0 = unallocated, 1 = allocated, -1 = not allocatable

        return ASMWriter {
            file: asm_file,
            label: label,
            regs: regs,
            current_func: None,
            while_labels: vec![],
        };
    }

    // Write a line to the assembly file
    pub fn write(&mut self, line: &str) {
        // Attempt to write the line (with a bonus newline at the end), and panic if unable to
        match write!(self.file, "{}\n", line) {
            Ok(()) => {}
            Err(_) => panic!("Unable to write to ASM file! Quitting now, sorry!"),
        };
    }

    // Update the current label
    pub fn new_label(&mut self) -> String {
        // Get number of current label
        let mut label_num = self.label[1..].to_string().parse::<u64>().unwrap();

        // Increment label number by one
        label_num += 1;

        // Update label
        self.label = format!("L{}", label_num);

        // Return the label, just for fun
        return self.label.clone();
    }

    pub fn alloc_reg(&mut self) -> i32 {
        // Usable registers are 9 - 15 (not saved), 19 - 28 (saved)
        for (i, reg) in self.regs.iter_mut().enumerate() {
            // If we find an unallocated register,
            if *reg == 0 {
                // Mark it as allocated
                *reg = 1;

                // Since the register at self.regs[0] is r9,
                // the actual register number is i + 9
                return (i + 9) as i32;
            }
        }

        // If we've made it through the whole list and run out of unallocated registers completely, throw an error
        throw_error("Calculation too compilated, ran out of registers");
        return 0;
    }

    pub fn free_reg(&mut self, to_free: i32) {
        // Usable registers are 9 - 15 (not saved), 19 - 28 (saved)

        // Since the register at self.regs[0] is r9,
        // the index of the register we want to free is to_free - 9
        self.regs[(to_free - 9) as usize] = 0;
    }

    pub fn get_allocated_caller_saved_registers(&self) -> Vec<usize> {
        let mut allocated_caller_saved_registers = Vec::new();

        // Loop through caller saved registers (self.regs[0] - self.regs[6])
        for i in 0..7 {
            if self.regs[i] == 1 {
                allocated_caller_saved_registers.push(i + 9);
            }
        }

        return allocated_caller_saved_registers;
    }

    pub fn get_allocated_callee_saved_registers(&self) -> Vec<usize> {
        let mut allocated_callee_saved_registers = Vec::new();

        // Loop through callee saved registers (self.regs[10] - self.regs[19])
        for i in 10..self.regs.len() {
            if self.regs[i] == 1 {
                allocated_callee_saved_registers.push(i + 9);
            }
        }

        return allocated_callee_saved_registers;
    }

    pub fn enter_func(&mut self, func: &mut ASTNode) {
        self.current_func = Some(func.clone());
    }

    pub fn exit_func(&mut self) {
        self.current_func = None;
    }

    pub fn get_current_func(&self) -> ASTNode {
        match &self.current_func {
            None => {
                return ASTNode::new("", None, None);
            } // Will never happen, indicates an error on my part
            Some(func) => {
                return func.clone();
            }
        }
    }

    pub fn get_current_func_name(&self) -> String {
        match &self.current_func {
            None => {
                return String::from("FUNC");
            } // Will never happen, indicates an error on my part
            Some(func) => {
                return func.get_sym().borrow().name.clone();
            }
        }
    }
}
