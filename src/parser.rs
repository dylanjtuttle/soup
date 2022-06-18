use std::io;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

use crate::{scanner::{Token, TokenName}, throw_error};


// -----------------------------------------------------------------
// AST
// -----------------------------------------------------------------

pub struct ASTNode {
    node_type: String,
    attr: Option<String>,
    line_num: Option<i32>,
    children: Vec<Rc<RefCell<ASTNode>>>,
}

impl ASTNode {
    pub fn new(node_type: &str, attr: Option<String>, line_num: Option<i32>) -> ASTNode {
        return ASTNode {
            node_type: String::from(node_type),
            attr: attr,
            line_num: line_num,
            children: vec![],
        };
    }
  
    pub fn add_child(&mut self, new_node: Rc<RefCell<ASTNode>>) {
      self.children.push(new_node);
    }

    pub fn add_children(&mut self, new_nodes: Vec<Rc<RefCell<ASTNode>>>) {
        for node in new_nodes {
            self.children.push(node);
        }
    }
  
    pub fn display_string(&self) -> String {
        let mut display_string = format!("{{{}", self.node_type);

        // Only print attr if it exists
        let attr = match &self.attr {
            None => String::from(""),
            Some(attr) => format!(", attr: '{}'", attr),
        };

        display_string.push_str(&attr);

        // Only print line number if it exists
        let line_num = match self.line_num {
            None => String::from(""),
            Some(line_num) => format!(", line {}", line_num),
        };

        display_string.push_str(&line_num);

        // Print node close brace
        display_string.push_str("}");

        return display_string;
    }
  }


fn new_node(node_type: &str, attr: Option<String>, line_num: Option<i32>) -> Rc<RefCell<ASTNode>> {
    let node = Rc::new(RefCell::new(ASTNode::new(node_type, attr, line_num)));

    return node;
}

pub fn print_ast(node: Rc<RefCell<ASTNode>>, num_tabs: i32) {
    // Add the correct indentation by adding num_tabs tabs
    for _i in 0..num_tabs {
        print!("\t");                   // Print a tab without a newline at the end
        io::stdout().flush().unwrap();  // 
    }

    // Print current node
    println!("{}", node.borrow_mut().display_string());

    // Call recursively on the nodes children
    for child in &node.borrow_mut().children {
        print_ast(Rc::clone(child), num_tabs + 1);
    }
}


// -----------------------------------------------------------------
// PARSER
// -----------------------------------------------------------------

pub fn parser(tokens: &Vec<Token>) -> Rc<RefCell<ASTNode>> {
    start_(tokens, &mut 0)
}


// -----------------------------------------------------------------
// GRAMMAR NON-TERMINAL FUNCTIONS
// -----------------------------------------------------------------


// start		: {globaldeclarations}
// 			    ;
fn start_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Create the root program node for this code file
    let ast_root = new_node("program", None, None);

    if tokens[0].name != TokenName::EOF {
        // If this was an empty file, the first (and only) token would be EOF,
        // in which case we would just return the program node. However, since this file
        // is non-empty, we can parse through it and create our AST:

        // ast_root.borrow_mut().add_child(variabledeclaration_(tokens, current));

        ast_root.borrow_mut().add_children(globaldeclarations_(tokens, current));
    }

    return ast_root;
}


// type    	: BOOLEAN
// 	        | INT
// 	        ;
fn type_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // Create AST leaf node for type
    let type_node = new_node("type",
                                                   Some(current_token.lexeme.clone()),
                                                   Some(current_token.line_num));

    // Update the type node type to correspond to the token we see
    match current_token.name {
        TokenName::INT => {type_node.borrow_mut().node_type = String::from("int");}
        TokenName::BOOL => {type_node.borrow_mut().node_type = String::from("bool");}
        _ => {
            throw_error(&format!("Syntax Error on line {}: type must be one of \"int\", \"bool\"",
            tokens[*current + 1].line_num));
        }
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return the type AST node
    return type_node;
}


// globaldeclarations		: [globaldeclaration]+
// 						    ;
fn globaldeclarations_(tokens: &Vec<Token>, current: &mut usize) -> Vec<Rc<RefCell<ASTNode>>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Initialize a vector to hold all of the global declaration nodes so we can return them
    let mut children_vec = Vec::new();

    // Loop until we reach the end of the file
    while current_token.name != TokenName::EOF {
        children_vec.push(globaldeclaration_(tokens, current));
        current_token = &tokens[*current];
    }

    return children_vec;
}


// globaldeclaration       : variabledeclaration
//                         | functiondeclaration
//                         | mainfunctiondeclaration
//                         ;
fn globaldeclaration_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // We have to find out what kind of global declaration this is, or throw an error if our token doesn't match
    if current_token.name == TokenName::FUNC {
        // We have a function declaration, so we just need to find out if it's a main function or just a regular one
        if tokens[*current + 1].name == TokenName::MAIN {
            // We have a main function
            return mainfunctiondeclaration_(tokens, current);
        } else if tokens[*current + 1].name == TokenName::ID {
            // We have a regular function
            return functiondeclaration_(tokens, current);
        } else {
            throw_error(&format!("Syntax Error on line {}: \"func\" keyword must be followed by \"main\" or identifier",
                        tokens[*current + 1].line_num));
        }

    } else if current_token.name == TokenName::INT || current_token.name == TokenName::BOOL {
        // We have a variable declaration
        return variabledeclaration_(tokens, current);

    } else {
        throw_error(&format!("Syntax Error on line {}: global declaration must take the form of a function or variable declaration",
                    tokens[*current + 1].line_num));
    }

    // Return a dummy node, this code is unreachable since throw_error() exits the program
    return new_node("globDecl", None, None);
}


// variabledeclaration     : type identifier SEMICOLON
//                         ;
fn variabledeclaration_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create variable declaration node
    let var_decl_node = new_node("varDecl",
                                                       None,
                                                       Some(current_token.line_num));
    
    // Add child for the variable type
    var_decl_node.borrow_mut().add_child(type_(tokens, current));

    // Add child for the variable identifier
    var_decl_node.borrow_mut().add_child(identifier_(tokens, current));

    // Check to see if current token is a semicolon - if not, throw syntax error
    current_token = &tokens[*current];
    if current_token.name != TokenName::SEMICOLON {
        throw_error(&format!("Syntax Error on line {}: variable declaration must end with a semicolon \";\"",
                    current_token.line_num));
    }

    // Consume the semicolon token and move on to the next one
    consume_token(current);

    // If we made it to here, we must have successfully parsed the variable declaration,
    // so return the newly created node!
    return var_decl_node;
}


// identifier              : ID
//                         ;
fn identifier_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    if current_token.name != TokenName::ID {
        throw_error(&format!("Syntax Error on line {}: expected an identifier",
                    current_token.line_num));
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return an identifier AST node corresponding to the ID token
    return new_node("id",
                    Some(current_token.lexeme.clone()),
                    Some(current_token.line_num));
}


// functiondeclaration     : functionheader block
//                         ;
fn functiondeclaration_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // Create function declaration node
    let new_node = new_node("funcDecl",
                                             None,
                                         Some(current_token.line_num));
    
    // Add child through function header
    new_node.borrow_mut().add_children(functionheader_(tokens, current));

    // Add child for block
    new_node.borrow_mut().add_child(block_(tokens, current));

    // Return function declaration node
    return new_node;
}


// functionheader          : FUNC functiondeclarator RETURNS [type | VOID]
//                         ;
fn functionheader_(tokens: &Vec<Token>, current: &mut usize) -> Vec<Rc<RefCell<ASTNode>>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create a vector to hold the AST nodes
    let mut node_vec = Vec::new();

    // A function header always starts with a "func" keyword, otherwise we have a syntax error
    if current_token.name != TokenName::FUNC {
        throw_error(&format!("Syntax Error on line {}: function declaration must always start with a \"func\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "func" keyword, so we can consume it
    consume_token(current);
    // Add nodes from the function declarator
    for node in functiondeclarator_(tokens, current) {
        node_vec.push(node);
    }

    // Next we should see the "returns" keyword
    current_token = &tokens[*current];
    if current_token.name != TokenName::RETURNS {
        throw_error(&format!("Syntax Error on line {}: expected \"returns\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "returns" keyword, so we can consume it
    consume_token(current);

    // Create a node to hold the return value of the function
    let returns_node = new_node("returns", None, None);

    current_token = &tokens[*current];
    if current_token.name == TokenName::VOID {
        returns_node.borrow_mut().add_child(new_node("void",
                                                     Some(String::from("void")),
                                                     Some(current_token.line_num)));
        
        // Consume void token
        consume_token(current);

    } else {
        // Otherwise we should see a type
        returns_node.borrow_mut().add_child(type_(tokens, current));
    }

    // Add the return node to the list
    node_vec.push(returns_node);

    // Finally we can return our function header nodes
    return node_vec;
}


// functiondeclarator      : identifier OPENPAR {formalparameterlist} CLOSEPAR
//                         ;
fn functiondeclarator_(tokens: &Vec<Token>, current: &mut usize) -> Vec<Rc<RefCell<ASTNode>>> {
    // Create a vector to hold the AST nodes
    let mut node_vec = Vec::new();

    // Add node for function name (identifier)
    node_vec.push(identifier_(tokens, current));

    // Next we should see an open parenthesis:
    let mut current_token = &tokens[*current];
    if current_token.name != TokenName::OPENPAR {
        throw_error(&format!("Syntax Error on line {}: function name must be followed by a parameter list enclosed in parentheses \"(\" \")\"",
                    current_token.line_num));
    }

    // Otherwise we found a "(", so we can consume it
    consume_token(current);
    current_token = &tokens[*current];

    // Now we can start parsing the parameter list
    let param_list = new_node("parameterList",
                                               None,
                                           Some(current_token.line_num));
    
    // Add one child for each parameter in the list
    param_list.borrow_mut().add_children(formalparameterlist_(tokens, current));

    // Add param list to function declarator node
    node_vec.push(param_list);

    // Next we should see an close parenthesis:
    current_token = &tokens[*current];
    if current_token.name != TokenName::CLOSEPAR {
        throw_error(&format!("Syntax Error on line {}: function parameter list must be followed up by a close parenthesis \")\"",
                    current_token.line_num));
    }

    // Otherwise we found a ")", so we can consume it
    consume_token(current);

    // We can now return our list of nodes
    return node_vec;
}


// formalparameterlist     : formalparameter [COMMA formalparameter]*
//                         ;
fn formalparameterlist_(tokens: &Vec<Token>, current: &mut usize) -> Vec<Rc<RefCell<ASTNode>>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create a vector to hold the AST nodes
    let mut param_list = Vec::new();

    if current_token.name == TokenName::CLOSEPAR {
        // If the current token is a close parenthesis, this function has no parameters and we can return an empty list
        return param_list;
    }

    // Otherwise, we have at least one parameter that we need to parse
    param_list.push(formalparameter_(tokens, current));

    // Loop through more parameters until we reach the close parenthesis
    current_token = &tokens[*current];

    while current_token.name != TokenName::CLOSEPAR {

        if current_token.name == TokenName::COMMA {
            // Consume comma token and then parse the following parameter
            consume_token(current);
            param_list.push(formalparameter_(tokens, current));

            // Update current token
            current_token = &tokens[*current];

        } else {
            throw_error(&format!("Syntax Error on line {}: function parameter list must be a comma separated list of parameters",
                        current_token.line_num));
        }
    }

    return param_list;
}


// formalparameter         : type identifier
//                         ;
fn formalparameter_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    let param = new_node("parameter",
                                          None,
                                      Some(current_token.line_num));

    // Add child for parameter type
    param.borrow_mut().add_child(type_(tokens, current));

    // Add child for parameter identifier
    param.borrow_mut().add_child(identifier_(tokens, current));

    return param;
}


// mainfunctiondeclaration : FUNC mainfunctiondeclarator RETURNS VOID block
//                         ;
fn mainfunctiondeclaration_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create function declaration node
    let main_decl_node = new_node("mainFuncDecl",
                                             None,
                                         Some(current_token.line_num));

    // A function declaration always starts with a "func" keyword, otherwise we have a syntax error
    if current_token.name != TokenName::FUNC {
        throw_error(&format!("Syntax Error on line {}: main function declaration must always start with a \"func\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "func" keyword, so we can consume it
    consume_token(current);
    
    // Parse main function declarator
    main_decl_node.borrow_mut().add_child(mainfunctiondeclarator_(tokens, current));

    // Next we should see the "returns" keyword
    current_token = &tokens[*current];
    if current_token.name != TokenName::RETURNS {
        throw_error(&format!("Syntax Error on line {}: expected \"returns\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "returns" keyword, so we can consume it
    consume_token(current);

    // Create a node to hold the return value of the function
    let returns_node = new_node("returns", None, None);

    current_token = &tokens[*current];
    if current_token.name == TokenName::VOID {
        returns_node.borrow_mut().add_child(new_node("void",
                                                     Some(String::from("void")),
                                                     Some(current_token.line_num)));
        
        // Consume void token
        consume_token(current);

    } else {
        throw_error(&format!("Syntax Error on line {}: main function must return \"void\"",
                    current_token.line_num));
    }

    // Add returns node to main declaration node
    main_decl_node.borrow_mut().add_child(returns_node);

    // Add child for block
    main_decl_node.borrow_mut().add_child(block_(tokens, current));

    // Return function declaration node
    return main_decl_node;
}


// mainfunctiondeclarator  : MAIN OPENPAR CLOSEPAR
//                         ;
fn mainfunctiondeclarator_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Main function must be called "main"
    if current_token.name != TokenName::MAIN {
        throw_error(&format!("Syntax Error on line {}: main function must be called \"main\"",
                    current_token.line_num));
    }

    // Otherwise, we found a "main" keyword, so we can consume it
    consume_token(current);
    current_token = &tokens[*current];

    // "main" keyword must be followed by "()"
    if current_token.name != TokenName::OPENPAR || tokens[*current + 1].name != TokenName::CLOSEPAR {
        throw_error(&format!("Syntax Error on line {}: \"main\" keyword must be followed by \"()\"",
                    current_token.line_num));
    }

    // Otherwise, we found a pair of tokens "()", so we can consume them
    consume_token(current);
    consume_token(current);
    current_token = &tokens[*current];

    return new_node("id", Some(String::from("main")), Some(current_token.line_num));
}


// block                   : OPENBRACE {blockstatements} CLOSEBRACE
//                         ;
fn block_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let mut current_token = &tokens[*current];

    let block_node = new_node("block", None, Some(current_token.line_num));

    // A block should always start with an open brace
    if current_token.name != TokenName::OPENBRACE {
        throw_error(&format!("Syntax Error on line {}: expected an open brace \"{{\"",
                    current_token.line_num));
    }

    // Otherwise, we found an open brace token, so we can consume it
    consume_token(current);

    // Add block statements as children to our block node
    block_node.borrow_mut().add_children(blockstatements_(tokens, current));

    // A block should always end with a close brace
    current_token = &tokens[*current];
    if current_token.name != TokenName::CLOSEBRACE {
        throw_error(&format!("Syntax Error on line {}: expected a close brace \"}}\"",
                    current_token.line_num));
    }

    // Otherwise, we found an open brace token, so we can consume it
    consume_token(current);

    // Return the block node
    return block_node;
}


// blockstatements         : [blockstatement]+
//                         ;
fn blockstatements_(tokens: &Vec<Token>, current: &mut usize) -> Vec<Rc<RefCell<ASTNode>>> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create vector to hold block statement nodes
    let mut statement_vec = Vec::new();

    // Blocks cannot be empty, so if the first token we see is a close brace, we have a syntax error:
    if current_token.name == TokenName::CLOSEBRACE {
        throw_error(&format!("Syntax Error on line {}: block cannot be empty",
                    current_token.line_num));
    }

    // Otherwise, we have a non-empty block, so we can loop until we find that close brace
    while current_token.name != TokenName::CLOSEBRACE {
        statement_vec.push(blockstatement_(tokens, current));
        current_token = &tokens[*current];
    }

    return statement_vec;
}


// blockstatement          : variabledeclaration
//                         | statement
//                         ;
fn blockstatement_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // A block statement can either be a variable declaration or a statement
    // If it is a variable declaration, the first token we will find is a type (int or bool)
    if current_token.name == TokenName::INT || current_token.name == TokenName::BOOL {
        return variabledeclaration_(tokens, current);
    } else {
        // Otherwise, it is a statement, and if the first token doesn't match any of those options,
        // we will deal with the syntax error in there
        return statement_(tokens, current);
    }
}


// statement               : block
//                         | SEMICOLON
//                         | statementexpression SEMICOLON
//                         | BREAK SEMICOLON
//                         | RETURN expression SEMICOLON
//                         | RETURN SEMICOLON
//                         | IF expression statement
//                         | IF expression statement ELSE statement
//                         | WHILE expression statement
//                         ;
fn statement_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let mut current_token = &tokens[*current];

    match current_token.name {
        // If the statement is a block, the first token we see is an open brace
        TokenName::OPENBRACE => {return block_(tokens, current);}

        // If the statement is a void statement, the first token we see is a semicolon
        TokenName::SEMICOLON => {
            // Consume semicolon token
            consume_token(current);
            current_token = &tokens[*current];

            return new_node("voidStmt", None, Some(current_token.line_num));
        }

        // If the statement is a statement expression (which can be either an assignment or a function call),
        // the first token we see is an identifier
        TokenName::ID => {return statementexpression_(tokens, current);}

        // If the statement is a break statement, the first token we see is a BREAK token
        TokenName::BREAK => {
            // Consume break token
            consume_token(current);
            current_token = &tokens[*current];

            // Break statement must be followed by a semicolon
            if current_token.name != TokenName::SEMICOLON {
                throw_error(&format!("Syntax Error on line {}: break statement must end with a semicolon",
                            current_token.line_num));
            }

            // Otherwise, consume semicolon token
            consume_token(current);

            return new_node("break", None, Some(tokens[*current - 2].line_num));
        }

        // If the statement is a return statement, the first token we see is a RETURN token
        TokenName::RETURN => {
            // Consume return token
            consume_token(current);
            current_token = &tokens[*current];

            if current_token.name == TokenName::SEMICOLON {
                // We have an empty return statement, consume semicolon token
                consume_token(current);
                current_token = &tokens[*current];

                return new_node("return", None, Some(current_token.line_num));

            } else {
                println!("\nNON-EMPTY RETURN!!!\n");
                let return_node = new_node("return", None, Some(current_token.line_num));

                return_node.borrow_mut().add_child(expression_(tokens, current));

                // Return statement must end with a semicolon
                current_token = &tokens[*current];
                if current_token.name != TokenName::SEMICOLON {
                    throw_error(&format!("Syntax Error on line {}: return statement must end with a semicolon",
                                current_token.line_num));
                }

                // Otherwise, consume semicolon token
                consume_token(current);

                return return_node;
            }
        }

        // If the statement is an if or if-else statement, the first token we see is an IF token
        TokenName::IF => {
            // Consume if token
            consume_token(current);
            
            // Parse if expression
            let if_expr_node = expression_(tokens, current);

            // Parse if body
            let statement_node = statement_(tokens, current);

            // Check if this is an if statement or an if-else statement
            current_token = &tokens[*current];
            if current_token.name != TokenName::ELSE {
                // If there is no else, create the if node
                let if_node = new_node("if", None, Some(current_token.line_num));

                // Add the expression and statement nodes
                if_node.borrow_mut().add_child(if_expr_node);
                if_node.borrow_mut().add_child(statement_node);

                // Return if node
                return if_node;
            } else {
                // If there is an else, create an if-else node and continue parsing
                let if_else_node = new_node("ifElse", None, Some(current_token.line_num));

                // Add the expression and statement nodes
                if_else_node.borrow_mut().add_child(if_expr_node);
                if_else_node.borrow_mut().add_child(statement_node);

                // Consume else token
                consume_token(current);

                // Add the else statement
                if_else_node.borrow_mut().add_child(statement_(tokens, current));

                // Return if-else node
                return if_else_node;
            }
        }

        // If the statement is a while loop, the first token we see is a WHILE token
        TokenName::WHILE => {
            // Consume while token
            consume_token(current);
            current_token = &tokens[*current];

            // Create while node
            let while_node = new_node("while", None, Some(current_token.line_num));

            // Add the expression node
            while_node.borrow_mut().add_child(expression_(tokens, current));

            // Add the body of the loop
            while_node.borrow_mut().add_child(statement_(tokens, current));

            return while_node;
        }

        // Otherwise, we have a syntax error
        _ => {
            throw_error(&format!("Syntax Error on line {}: not a valid statement",
                        current_token.line_num));
            
            // Return dummy node to avoid the compiler getting angry with me
            return new_node("statement", None, None);
        }
    }
}


// statementexpression     : assignment
//                         | functioninvocation
//                         ;
fn statementexpression_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    return new_node("voidStmt", None, Some(current_token.line_num));
}


// expression              : assignmentexpression
//                         ;
fn expression_(tokens: &Vec<Token>, current: &mut usize) -> Rc<RefCell<ASTNode>> {
    // Get current token
    let current_token = &tokens[*current];

    // HARD CODED
    consume_token(current);

    return new_node("expression of some kind", None, Some(current_token.line_num));
}


// -----------------------------------------------------------------
// MISC FUNCTIONS
// -----------------------------------------------------------------

fn consume_token(current: &mut usize) {
    *current += 1;
}
