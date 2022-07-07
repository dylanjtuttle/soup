use std::collections::HashMap;

use crate::parser::{ASTNode, print_ast};
use crate::throw_error;

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

    // Return a mutable reference to the top scope in the stack, or None if the scope stack is empty
    fn peek(&mut self) -> Option<&mut HashMap<String, Symbol>> {
        self.stack.last_mut()
    }

    // Open up a new scope by creating a new symbol table and pushing it to the top of the scope stack
    fn open_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    // Close the topmost scope by popping it and allowing it to go out of (this function's) scope
    fn close_scope(&mut self) {
        _ = self.stack.pop();
    }

    // Insert a new symbol into the topmost scope of the scope stack
    fn insert_symbol(&mut self, name: String, new_symbol: Symbol) {
        match self.peek() {
            None => {
                throw_error("Empty scope stack");
            }
            Some(top) => {
                top.insert(name, new_symbol);
            }
        };
    }

    // Attempt to find a symbol somewhere in the scope stack
    fn find_symbol(&self, search_name: &str) -> Option<&Symbol> {
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
        None
    }

    // Attempt to find a symbol only in the current topmost scope
    fn find_in_scope(&mut self, search_name: &str) -> Option<&Symbol> {
        match self.peek() {
            // If the scope stack is empty, we obviously won't be able to find the symbol
            None => {None}
            Some(symbol_table) => {
                // Search through each entry in the symbol table for the given name
                for (name, symbol) in symbol_table {
                    // If we find a symbol with that name, return a reference to it
                    if name == search_name {
                        return Some(symbol);
                    }
                }

                // Otherwise, we weren't able to find a symbol with the given name, so return None
                None
            }
        }
    }

    // Return the level of the scope (the length of the list)
    fn scope_level(&self) -> usize {
        self.stack.len()
    }
}

// -----------------------------------------------------------------
// SYMBOL
// -----------------------------------------------------------------

#[derive(Clone)]
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

pub fn semantic_checker(ast: &mut ASTNode) {
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

    // Open a new scope for the runtime library
    scope_stack.open_scope();

    // Add a symbol for everything in the runtime library
    scope_stack.insert_symbol(String::from("getchar"), Symbol::new(String::from("getchar"), String::from("f()"), String::from("int")));
    scope_stack.insert_symbol(String::from("halt"), Symbol::new(String::from("halt"), String::from("f()"), String::from("void")));
    scope_stack.insert_symbol(String::from("printbool"), Symbol::new(String::from("printbool"), String::from("f(bool)"), String::from("void")));
    scope_stack.insert_symbol(String::from("printchar"), Symbol::new(String::from("printchar"), String::from("f(int)"), String::from("void")));
    scope_stack.insert_symbol(String::from("printint"), Symbol::new(String::from("printint"), String::from("f(int)"), String::from("void")));
    scope_stack.insert_symbol(String::from("printstr"), Symbol::new(String::from("printstr"), String::from("f(string)"), String::from("void")));

    // Open a new scope for the global symbols in anticipation of the first pass
    scope_stack.open_scope();

    // Begin first pass
    let mut num_main_decls = 0;
    pass1(ast, &mut scope_stack, &mut num_main_decls);

    // Check for incorrect number of main declarations
    if num_main_decls == 0 {
        throw_error("Program must contain a main function declaration");
    } else if num_main_decls > 1 {
        throw_error("Program cannot contain more than one main function declaration")
    }

    // Begin second pass
    pass2(ast, &mut scope_stack);

    // Begin third pass
    pass3(ast, &mut scope_stack);

    print_ast(ast);
}


// -----------------------------------------------------------------
// AST TRAVERSALS
// -----------------------------------------------------------------

fn pass1(node: &mut ASTNode, scope_stack: &mut ScopeStack, num_main_decls: &mut i32) {
    // Call recursively on the current node's children
    for child in &mut node.children {
        pass1(child, scope_stack, num_main_decls);
    }

    // Execute pass1 function
    pass1_post(node, scope_stack, num_main_decls);
}

fn pass1_post(node: &mut ASTNode, scope_stack: &mut ScopeStack, num_main_decls: &mut i32) {
    let node_type = &node.node_type.clone();

    if node_type == "mainFuncDecl" {
        // Create a symbol for the main declaration
        let main_symbol = Symbol::new(String::from("main"), String::from("f()"), String::from("void"));

        // Clone that symbol to keep a copy for the AST
        let main_symbol_ast = main_symbol.clone();

        // Insert new symbol into scope stack
        scope_stack.insert_symbol(String::from("main"), main_symbol);

        // Add symbol table entry to AST node
        node.add_sym(main_symbol_ast);

        // Keep track of the number of main declarations
        *num_main_decls += 1;

    } else if node_type == "funcDecl" {
        // Get fields from the AST
        let func_name = get_attr(&node.children[0]);
        let func_sig = get_func_sig(node);
        let func_returns = node.children[2].children[0].node_type.clone();

        // Create a symbol for the function declaration
        let func_symbol = Symbol::new(func_name.clone(), func_sig, func_returns);

        // Clone that symbol to keep a copy for the AST
        let func_symbol_ast = func_symbol.clone();

        // Insert new symbol into scope stack
        scope_stack.insert_symbol(func_name.clone(), func_symbol);

        // Copy same symbol into AST node
        node.add_sym(func_symbol_ast);

    } else if node_type == "globVarDecl" {
        // Get fields from the AST
        let var_name = get_attr(&node.children[1]);
        let var_type = node.children[0].node_type.clone();
        let var_returns = var_type.clone();

        // Create a symbol for the variable declaration
        let var_symbol = Symbol::new(var_name.clone(), var_type, var_returns);

        // Clone that symbol to keep a copy for the AST
        let var_symbol_ast = var_symbol.clone();

        // Insert new symbol into scope stack
        scope_stack.insert_symbol(var_name.clone(), var_symbol);

        // Copy same symbol into AST node
        node.add_sym(var_symbol_ast);
    }
}

fn pass2(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    // Execute pass2 function before checking node children
    pass2_pre(node, scope_stack);

    // Call recursively on the current node's children
    for child in &mut node.children {
        pass2(child, scope_stack);
    }

    // Execute pass2 function after checking node children
    pass2_post(node, scope_stack);
}

fn pass2_pre(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    if node.node_type == "funcDecl" ||
       node.node_type == "mainFuncDecl" ||
       node.node_type == "if" ||
       node.node_type == "ifElse" ||
       node.node_type == "while" {
        // Open up a new scope
        scope_stack.open_scope();

    } else if node.node_type == "varDecl" {
        // Variables can only be defined in the global or function scopes (scope levels 2 and 3)
        if scope_stack.scope_level() > 3 {
            throw_error(&format!("Line {}: Variables can only be defined in the outermost scope of a function or globally (i.e. not in an if statement, while loop, etc.)",
                                      get_line_num(node)))
        }

        // Check if a variable with this name has already been defined in this scope
        match scope_stack.find_in_scope(&get_attr(&node.children[1])) {
            None => {
                // This variable hasn't been defined yet in this scope, so we can proceed to define it in our symbol table
                let var_name = get_attr(&node.children[1]);
                let var_type = node.children[0].node_type.clone();
                
                let var_symbol = Symbol::new(var_name.clone(),
                                                     var_type.clone(),
                                                     var_type);
                
                // Insert symbol into symbol table
                scope_stack.insert_symbol(var_name, var_symbol.clone());

                // Insert symbol table entry into AST
                node.sym = Some(var_symbol);
            }
            Some(_) => {
                // A variable with this name has been defined already in this scope
                throw_error(&format!("Line {}: Variable illegally redefined within the same scope",
                                      get_line_num(node)))
            }
        }

    } else if node.node_type == "parameter" {
        // Parameters are essentially identical to local variables
        let param_name = get_attr(&node.children[1]);
        let param_type = node.children[0].node_type.clone();
        
        let param_symbol = Symbol::new(param_name.clone(),
                                               param_type.clone(),
                                               param_type);
        
        // Insert symbol into symbol table
        scope_stack.insert_symbol(param_name, param_symbol.clone());

        // Insert symbol table entry into AST
        node.sym = Some(param_symbol);

    } else if node.node_type == "id" {
        match scope_stack.find_symbol(&get_attr(&node)) {
            // If we can't find the identifier, we haven't defined it yet
            None => {
                throw_error(&format!("Line {}: Unknown identifier '{}'",
                                      get_line_num(node), get_attr(&node)))
            }
            Some(symbol) => {
                // This identifier exists already, so we already know what it returns and what its symbol table is
                node.type_sig = Some(symbol.type_sig.clone());
                node.sym = Some(symbol.clone());
            }
        }

    } else if node.node_type == "number" {
        node.type_sig = Some(String::from("int"));

    } else if node.node_type == "true" || node.node_type == "false" {
        node.type_sig = Some(String::from("bool"));
    }
}

fn pass2_post(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    if node.node_type == "funcDecl" ||
       node.node_type == "mainFuncDecl" ||
       node.node_type == "if" ||
       node.node_type == "ifElse" ||
       node.node_type == "while" {
        // Close the topmost scope
        scope_stack.close_scope();

    }
}

fn pass3(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    // Call recursively on the current node's children
    for child in &mut node.children {
        pass3(child, scope_stack);
    }

    // Execute pass3 function after checking node children
    pass3_post(node, scope_stack);
}

fn pass3_post(node: &mut ASTNode, scope_stack: &mut ScopeStack) {
    if is_binary(node) {
        let left_type = get_type(&node.children[0]);
        let right_type = get_type(&node.children[1]);

        // Both sides of a binary operation must have the same type
        if left_type != right_type {
            throw_error(&format!("Line {}: Type mismatch for {}, operands must have same type",
                                      get_line_num(node), node.node_type))
        } else {
            // Types match, but we need to check if the types (even if they match) make sense with the operation
            if node.node_type == "&&" || node.node_type == "||" {
                // Both operands must be bools, returns a bool
                if left_type == "bool" && right_type == "bool" {
                    // Type check is successful
                    node.type_sig = Some(String::from("bool"));
                } else {
                    throw_error(&format!("Line {}: Type mismatch for {}, operands must be bools",
                                              get_line_num(node), node.node_type))
                }

            } else if node.node_type == "==" || node.node_type == "!=" {
                // Operands can be either ints or bools, returns a bool
                node.type_sig = Some(String::from("bool"));

            } else if node.node_type == "<" || node.node_type == ">" || node.node_type == "<=" || node.node_type == ">=" {
                // Both operands must be ints, returns a bool
                if left_type == "int" && right_type == "int" {
                    // Type check is successful
                    node.type_sig = Some(String::from("bool"));
                } else {
                    throw_error(&format!("Line {}: Type mismatch for {}, operands must be ints",
                                              get_line_num(node), node.node_type))
                }

            } else if node.node_type == "=" {
                // Operands can be either ints or bools, returns whatever type the operands are
                node.type_sig = Some(left_type);

            } else {
                // One of + += - -= * *= / /= % %=
                // Both operands must be ints, returns an int
                if left_type == "int" && right_type == "int" {
                    // Type check is successful
                    node.type_sig = Some(String::from("int"));
                } else {
                    throw_error(&format!("Line {}: Type mismatch for {}, operands must be ints",
                                              get_line_num(node), node.node_type))
                }
            }
        }

    } else if is_unary(&node) {
        let op_type = get_type(&node.children[0]);
        if node.node_type == "u-" {
            // Operand must be int, returns an int
            if op_type == "int" {
                // Type check is successful
                node.type_sig = Some(String::from("int"));
            } else {
                throw_error(&format!("Line {}: Type mismatch for -, operand must be int",
                                          get_line_num(node)))
            }
        } else {
            // !
            // Operand must be bool, returns a bool
            if op_type == "bool" {
                // Type check is successful
                node.type_sig = Some(String::from("bool"));
            } else {
                throw_error(&format!("Line {}: Type mismatch for {}, operand must be bool",
                                          get_line_num(node), node.node_type))
            }
        }

    } else if node.node_type == "funcCall" {
        let func_name = get_attr(&node.children[0]);

        // Get type signature of function call
        let func_sig = get_func_sig(&node);

        // Add func sig to type_sig of ASTNode
        node.type_sig = Some(func_sig.clone());

        // Try to find the function being called
        match scope_stack.find_symbol(&func_name) {
            None => {
                throw_error(&format!("Line {}: Unknown identifier '{}'",
                                          get_line_num(node), func_name))
            }
            Some(symbol) => {
                // Make sure the func sig of the found function matches our function call
                if symbol.type_sig != func_sig {
                    throw_error(&format!("Line {}: Argument(s) for invocation of function '{}' do not match parameter(s)",
                                          get_line_num(node), func_name))
                } else {
                    node.type_sig = Some(symbol.returns.clone());
                    node.sym = Some(symbol.clone());
                }
            }
        }
    } else if node.node_type == "funcDecl" {
        print_ast(node);
    }
}

fn get_attr(node: &ASTNode) -> String {
    match &node.attr {
        None => {
            // Should never happen, indicates an error on my end
            String::from("ATTR")
        }
        Some(attr) => {
            attr.clone()
        }
    }
}

fn get_line_num(node: &ASTNode) -> i32 {
    match &node.line_num {
        None => {
            // Should never happen, indicates an error on my end
            0
        }
        Some(line_num) => {
            *line_num
        }
    }
}

fn get_func_sig(node: &ASTNode) -> String {
    // Open func sig
    let mut func_sig = String::from("f(");

    // Loop through parameters
    let mut param_num = 0;
    for param in &node.children[1].children {
        param_num += 1;

        // Function parameters must be comma separated, so any parameter after the first must be prefixed by ", "
        if param_num > 1 {
            func_sig.push_str(", ");
        }

        // Add parameter/argument type to func sig
        func_sig.push_str(&get_type(&param.children[0]));
    }

    // Close func sig
    func_sig.push_str(")");

    return func_sig;
}

fn get_type(node: &ASTNode) -> String {
    match &node.type_sig {
        None => {
            match &node.sym {
                None => {
                    if node.node_type == "int" || node.node_type == "bool" || node.node_type == "string" {
                        node.node_type.clone()
                    } else {
                        String::from("NO TYPE")
                    }
                }
                Some(sym) => {sym.returns.clone()}
            }
        }
        Some(type_sig) => {
            type_sig.clone()
        }
    }
}

fn is_binary(node: &ASTNode) -> bool {
    node.node_type == "+" ||
    node.node_type == "+=" ||
    node.node_type == "-" ||
    node.node_type == "-=" ||
    node.node_type == "*" ||
    node.node_type == "*=" ||
    node.node_type == "/" ||
    node.node_type == "/=" ||
    node.node_type == "%" ||
    node.node_type == "%=" ||
    node.node_type == "+" ||
    node.node_type == "<" ||
    node.node_type == ">" ||
    node.node_type == "<=" ||
    node.node_type == ">=" ||
    node.node_type == "=" ||
    node.node_type == "==" ||
    node.node_type == "!=" ||
    node.node_type == "&&" ||
    node.node_type == "||"
}

fn is_unary(node: &ASTNode) -> bool {
    node.node_type == "u-" ||
    node.node_type == "!"
}