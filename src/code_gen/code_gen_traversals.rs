use crate::parser::parser_data::ASTNode;

use crate::code_gen::code_gen_callbacks::*;
use crate::code_gen::code_gen_data::*;

// -----------------------------------------------------------------
// CODE GENERATION TRAVERSAL
// -----------------------------------------------------------------

pub fn traverse_prune(writer: &mut ASMWriter, node: &mut ASTNode) {
    // Do something with the node before you visit its children,
    // and possibly return without visiting children if do_prune = true
    let do_prune = traverse_pre(writer, node);

    if do_prune {
        return;
    }

    // Visit children
    for child in &mut node.children {
        traverse_prune(writer, child);
    }

    // Do something again with the node
    traverse_post(writer, node);
}

// -----------------------------------------------------------------
// STRING GENERATION TRAVERSAL
// -----------------------------------------------------------------

pub fn gen_strings(writer: &mut ASMWriter, node: &mut ASTNode) {
    global_data(writer, node);

    // Visit children
    for child in &mut node.children {
        gen_strings(writer, child);
    }
}
