use std::{rc::Rc, cell::RefCell};
use std::collections::HashMap;

use crate::parser::{ASTNode, print_ast};
use crate::throw_error;

// -----------------------------------------------------------------
// SCOPE STACK
// -----------------------------------------------------------------

struct ScopeStack {
    stack: Vec<HashMap<String, Rc<RefCell<Symbol>>>>
}

impl ScopeStack {
    // Create a new scope stack
    fn new() -> Self {
        ScopeStack{stack: Vec::new()}
    }

    // Pop and return the top scope in the stack, or None if the scope stack is empty
    fn pop(&mut self) -> Option<HashMap<String, Rc<RefCell<Symbol>>>> {
        self.stack.pop()
    }

    // Return a mutable reference to the top scope in the stack, or None if the scope stack is empty
    fn peek(&mut self) -> Option<&mut HashMap<String, Rc<RefCell<Symbol>>>> {
        self.stack.last_mut()
    }

    // Push a new scope to the top of the scope stack
    fn push(&mut self, new_scope: HashMap<String, Rc<RefCell<Symbol>>>) {
        self.stack.push(new_scope);
    }

    // Get the current scope level
    fn scope_level(&mut self) -> usize {
        // Identical to the size of the scope stack
        self.stack.len()
    }

    fn insert_symbol(&mut self, name: String, new_symbol: Rc<RefCell<Symbol>>) {
        match self.peek() {
            None => {
                throw_error("Empty scope stack");
            }
            Some(top) => {
                top.insert(name, new_symbol);
            }
        };
    }

    fn find_symbol(&mut self, search_name: &str) -> Option<&Rc<RefCell<Symbol>>> {
        // Iterate backwards through the scope stack (i.e. starting at the top scope and moving downwards)
        for symbol_table in self.stack.iter().rev() {

            // Search through each entry in the symbol table for the given name
            for (name, symbol) in symbol_table {

                // If we find a symbol with that name, return a reference to it
                if name == search_name {
                    return Some(symbol);
                }
            }
        }

        // Otherwise, we weren't able to find a symbol with the given name, so return None
        return None;
    }
}

// -----------------------------------------------------------------
// SYMBOL
// -----------------------------------------------------------------

pub struct Symbol {
    pub name: String,
    pub type_sig: String,
    pub returns: String,
}

impl Symbol {
    // Create a new symbol
    fn new(name: String, type_sig: String, returns: String) -> Self {
        Symbol{name: name, type_sig: type_sig, returns: returns}
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
    runtime_sym.insert(String::from("getchar"), new_symbol(String::from("getchar"), String::from("f()"), String::from("int")));
    runtime_sym.insert(String::from("halt"), new_symbol(String::from("halt"), String::from("f()"), String::from("void")));
    runtime_sym.insert(String::from("printbool"), new_symbol(String::from("printbool"), String::from("f(bool)"), String::from("void")));
    runtime_sym.insert(String::from("printchar"), new_symbol(String::from("printchar"), String::from("f(int)"), String::from("void")));
    runtime_sym.insert(String::from("printint"), new_symbol(String::from("printint"), String::from("f(int)"), String::from("void")));
    runtime_sym.insert(String::from("printstr"), new_symbol(String::from("printstr"), String::from("f(string)"), String::from("void")));

    // Add symbol table to scope stack
    scope_stack.push(runtime_sym);

    // Open a new symbol table for the global symbols in anticipation of the first pass
    scope_stack.push(HashMap::new());

    // Begin first pass
    let mut num_main_decls = 0;
    pass1(ast, &mut scope_stack, &mut num_main_decls);

    // Check for incorrect number of main declarations
    if num_main_decls == 0 {
        throw_error("Program must contain a main function declaration");
    } else if num_main_decls > 1 {
        throw_error("Program cannot contain more than one main function declaration")
    }

    match scope_stack.find_symbol("test_func") {
        None => {
            println!("Symbol not found :(");
        }
        Some(symbol) => {
            println!("Found symbol!! Type sig: {}", symbol.borrow_mut().type_sig);
        }
    }

    print_ast(ast, 0);
}

fn new_symbol(name: String, type_sig: String, returns: String) -> Rc<RefCell<Symbol>> {
    Rc::new(RefCell::new(Symbol::new(name, type_sig, returns)))
}


// -----------------------------------------------------------------
// AST TRAVERSALS
// -----------------------------------------------------------------

fn pass1(node: &mut Rc<RefCell<ASTNode>>, scope_stack: &mut ScopeStack, num_main_decls: &mut i32) {
    // Call recursively on the current node's children
    for child in &node.borrow_mut().children {
        pass1(&mut Rc::clone(&child), scope_stack, num_main_decls);
    }

    // Execute pass1 function
    pass1_post(node, scope_stack, num_main_decls);
}

fn pass1_post(node: &mut Rc<RefCell<ASTNode>>, scope_stack: &mut ScopeStack, num_main_decls: &mut i32) {
    let node_type = &node.borrow().node_type.clone();

    if node_type == "mainFuncDecl" {
        // Create a symbol for the main declaration
        let main_symbol = new_symbol(String::from("main"), String::from("f()"), String::from("void"));

        // Insert new symbol into scope stack
        scope_stack.insert_symbol(String::from("main"), Rc::clone(&main_symbol));

        // Add symbol table entry to AST node
        node.borrow_mut().add_sym(Rc::clone(&main_symbol));

        // Keep track of the number of main declarations
        *num_main_decls += 1;

    } else if node_type == "funcDecl" {
        // Get fields from the AST
        let func_name = get_attr(&node.borrow().children[0]);
        let func_sig = get_func_sig(node);
        let func_returns = node.borrow().children[2].borrow().children[0].borrow().node_type.clone();

        // Create a symbol for the function declaration
        let func_symbol = new_symbol(func_name.clone(), func_sig, func_returns);

        // Insert new symbol into scope stack
        scope_stack.insert_symbol(func_name.clone(), Rc::clone(&func_symbol));

        // Copy same symbol into AST node
        node.borrow_mut().add_sym(Rc::clone(&func_symbol));

    } else if node_type == "globVarDecl" {
        // Get fields from the AST
        let var_name = get_attr(&node.borrow().children[1]);
        let var_type = node.borrow().children[0].borrow().node_type.clone();
        let var_returns = var_type.clone();

        // Create a symbol for the variable declaration
        let var_symbol = new_symbol(var_name.clone(), var_type, var_returns);

        // Insert new symbol into scope stack
        scope_stack.insert_symbol(var_name.clone(), Rc::clone(&var_symbol));

        // Copy same symbol into AST node
        node.borrow_mut().add_sym(Rc::clone(&var_symbol));
    }
}

fn get_attr(node: &Rc<RefCell<ASTNode>>) -> String {
    match &node.borrow().attr {
        None => {
            // Should never happen, indicates an error on my end
            String::from("ATTR")
        }
        Some(attr) => {
            attr.clone()
        }
    }
}

fn get_func_sig(node: &Rc<RefCell<ASTNode>>) -> String {
    // Open func sig
    let mut func_sig = String::from("f(");

    // Loop through parameters
    let mut param_num = 0;
    for param in &node.borrow().children[1].borrow().children {
        param_num += 1;

        // Function parameters must be comma separated, so any parameter after the first must be prefixed by ", "
        if param_num > 1 {
            func_sig.push_str(", ");
        }

        // Add parameter type to func sig
        func_sig.push_str(&get_attr(&param.borrow().children[0]))
    }

    // Close func sig
    func_sig.push_str(")");

    return func_sig;
}