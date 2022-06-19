use std::{rc::Rc, cell::RefCell};
use std::collections::HashMap;

use crate::parser::ASTNode;

// -----------------------------------------------------------------
// SCOPE STACK
// -----------------------------------------------------------------

struct ScopeStack {
    stack: Vec<HashMap<String, Symbol>>
}

impl ScopeStack {
    // Create a new scope stack
    fn new() -> Self {
        ScopeStack{stack: Vec::new()}
    }

    // Pop and return the top scope in the stack, or None if the scope stack is empty
    fn pop(&mut self) -> Option<HashMap<String, Symbol>> {
        self.stack.pop()
    }

    // Return a reference to the top scope in the stack, or None if the scope stack is empty
    fn peek(&mut self) -> Option<&HashMap<String, Symbol>> {
        self.stack.last()
    }

    // Push a new scope to the top of the scope stack
    fn push(&mut self, new_scope: HashMap<String, Symbol>) {
        self.stack.push(new_scope);
    }

    // Get the current scope level
    fn scope_level(&mut self) -> usize {
        // Identical to the size of the scope stack
        self.stack.len()
    }
}

// -----------------------------------------------------------------
// SYMBOL
// -----------------------------------------------------------------

struct Symbol {
    type_sig: String,
    returns: String,
}

impl Symbol {
    // Create a new symbol
    fn new(type_sig: String, returns: String,) -> Self {
        Symbol{type_sig: type_sig, returns: returns}
    }
}

// -----------------------------------------------------------------
// SEMANTIC CHECKER
// -----------------------------------------------------------------

pub fn semantic_checker(ast: &mut Rc<RefCell<ASTNode>>) {
    // This semantic checker will perform four traversals of the AST:
    // 
    // pass 1 - post-order - collects information about global declarations
    // 
    // pass 2 - pre/post-order - figures out what all the identifiers refer to
    //                           deals with the scope stack, opens up a new scope
    //                           in the pre pass and closes it in the post pass
    // 
    // pass 3 - post-order - full type checking
    // 
    // pass 4 - pre/post-order - miscellaneous grab bag of everything else

    // Begin by creating the scope stack, this will hold a symbol table for each level of scope:
    // level 1 - runtime library
    // level 2 - global
    // level 3 - function
    let mut scope_stack = ScopeStack::new();
    
    // Then create the first symbol table, for the runtime library
    let mut runtime_sym = HashMap::new();

    // Add a symbol for everything in the runtime library
    runtime_sym.insert(String::from("getchar"), Symbol::new(String::from("f()"), String::from("int")));
    runtime_sym.insert(String::from("halt"), Symbol::new(String::from("f()"), String::from("void")));
    runtime_sym.insert(String::from("printbool"), Symbol::new(String::from("f(bool)"), String::from("void")));
    runtime_sym.insert(String::from("printchar"), Symbol::new(String::from("f(int)"), String::from("void")));
    runtime_sym.insert(String::from("printint"), Symbol::new(String::from("f(int)"), String::from("void")));
    runtime_sym.insert(String::from("printstr"), Symbol::new(String::from("f(string)"), String::from("void")));

    // Add symbol table to scope stack
    scope_stack.push(runtime_sym);

    // Begin first pass
    pass1(ast, &mut scope_stack);
}

// -----------------------------------------------------------------
// AST TRAVERSALS
// -----------------------------------------------------------------

fn pass1(node: &mut Rc<RefCell<ASTNode>>, scope_stack: &mut ScopeStack) {
    // Call recursively on the current node's children
    for child in &node.borrow_mut().children {
        pass1(&mut Rc::clone(&child), scope_stack);
    }

    // Execute pass1 function
    pass1_post(node, scope_stack);
}

fn pass1_post(node: &mut Rc<RefCell<ASTNode>>, scope_stack: &mut ScopeStack) {
    println!("pass1_post (s{}): line {}, node {}",
             scope_stack.scope_level(),
             match node.borrow().line_num {
                 None => String::from(""),
                 Some(line) => format!("{}", line),
             },
             node.borrow().node_type
    )
}