use crate::parser::parser_data::ASTNode;

use crate::code_gen::code_gen_data::*;
use crate::code_gen::code_gen_generators::*;
use crate::code_gen::code_gen_traversals::*;

// -----------------------------------------------------------------
// CODE GENERATOR
// -----------------------------------------------------------------

pub fn code_gen(asm_filename: &str, ast: &mut ASTNode) {
    // Initialize the ASMWriter
    let mut writer = ASMWriter::new(asm_filename);

    writer.write(".data");
    // First, before we write any code, find all the strings and add them to the top of the file
    gen_strings(&mut writer, ast);
    writer.write(".align 2");
    writer.write(".text");

    // Generate the assembly file main routine (not to be confused with the compilee's main function)
    gen_asm_main(&mut writer);

    // Begin traversing the AST and generating code
    traverse_prune(&mut writer, ast);

    // Finally, generate the runtime library
    gen_runtime_lib(&mut writer);
}
