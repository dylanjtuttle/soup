use crate::scanner::scanner_data::{Token, TokenType};
use crate::parser::parser_data::*;
use crate::parser::parser_driver::*;
use crate::throw_error;

// -----------------------------------------------------------------
// GRAMMAR NON-TERMINAL FUNCTIONS
// -----------------------------------------------------------------

// start		: {globaldeclarations}
// 			    ;
pub fn start_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Create the root program node for this code file
    let mut ast_root = ASTNode::new("program", None, None);

    if tokens[0].token_type != TokenType::EOF {
        // If this was an empty file, the first (and only) token would be EOF,
        // in which case we would just return the program node. However, since this file
        // is non-empty, we can parse through it and create our AST:

        ast_root.add_children(globaldeclarations_(tokens, current));
    }

    return ast_root;
}

// literal     : INTLIT
//             | STRLIT
//             | TRUE
//             | FALSE
//             ;
pub fn literal_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // Create AST leaf node for literal
    let mut literal_node = ASTNode::new("literal",
                                                      Some(current_token.lexeme.clone()),
                                                      Some(current_token.line_num));

    // Update the literal node type to correspond to the token we see
    match current_token.token_type {
        TokenType::INTLIT => {literal_node.node_type = String::from("number");}
        TokenType::STRLIT => {literal_node.node_type = String::from("string");}
        TokenType::TRUE => {literal_node.node_type = String::from("true");}
        TokenType::FALSE => {literal_node.node_type = String::from("false");}
        _ => {
            throw_error(&format!("Syntax Error on line {}: literal must be an integer, string, \"true\", or \"false\"",
                        tokens[*current + 1].line_num));
        }
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return the literal AST node
    return literal_node;
}


// type    	: BOOLEAN
// 	        | INT
// 	        ;
pub fn type_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // Create AST leaf node for type
    let mut type_node = ASTNode::new("type",
                                                   Some(current_token.lexeme.clone()),
                                                   Some(current_token.line_num));

    // Update the type node type to correspond to the token we see
    match current_token.token_type {
        TokenType::INT => {type_node.node_type = String::from("int");}
        TokenType::BOOL => {type_node.node_type = String::from("bool");}
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
pub fn globaldeclarations_(tokens: &Vec<Token>, current: &mut usize) -> Vec<ASTNode> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Initialize a vector to hold all of the global declaration nodes so we can return them
    let mut children_vec = Vec::new();

    // Loop until we reach the end of the file
    while current_token.token_type != TokenType::EOF {
        children_vec.push(globaldeclaration_(tokens, current));
        current_token = &tokens[*current];
    }

    return children_vec;
}


// globaldeclaration       : variabledeclaration
//                         | functiondeclaration
//                         | mainfunctiondeclaration
//                         ;
pub fn globaldeclaration_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // We have to find out what kind of global declaration this is, or throw an error if our token doesn't match
    if current_token.token_type == TokenType::FUNC {
        // We have a function declaration, so we just need to find out if it's a main function or just a regular one
        if tokens[*current + 1].token_type == TokenType::MAIN {
            // We have a main function
            return mainfunctiondeclaration_(tokens, current);
        } else if tokens[*current + 1].token_type == TokenType::ID {
            // We have a regular function
            return functiondeclaration_(tokens, current);
        } else {
            throw_error(&format!("Syntax Error on line {}: \"func\" keyword must be followed by \"main\" or identifier",
                        tokens[*current + 1].line_num));
        }

    } else if current_token.token_type == TokenType::INT || current_token.token_type == TokenType::BOOL {
        // We have a variable declaration
        let mut glob_var_decl = variabledeclaration_(tokens, current);

        // We have to rename the "varDecl" node "globVarDecl" to distinguish from a variable declaration inside a function
        glob_var_decl.node_type = String::from("globVarDecl");

        return glob_var_decl;

    } else {
        throw_error(&format!("Syntax Error on line {}: global declaration must take the form of a function or variable declaration",
                    tokens[*current + 1].line_num));
    }

    // Return a dummy node, this code is unreachable since throw_error() exits the program
    return ASTNode::new("globDecl", None, None);
}


// variabledeclaration     : type identifier SEMICOLON
//                         ;
pub fn variabledeclaration_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create variable declaration node
    let mut var_decl_node = ASTNode::new("varDecl",
                                                       None,
                                                       Some(current_token.line_num));
    
    // Add child for the variable type
    var_decl_node.add_child(type_(tokens, current));

    // Add child for the variable identifier
    var_decl_node.add_child(identifier_(tokens, current));

    // Check to see if current token is a semicolon
    current_token = &tokens[*current];
    if current_token.token_type != TokenType::SEMICOLON {
        // If the current token is not a semicolon, it could still be an assignment operator
        if current_token.token_type == TokenType::ASSIGN {
            // Consume the assignment token
            consume_token(current);
            // Parse an assignment expression on the other side
            var_decl_node.add_child(assignmentexpression_(tokens, current));
            // Check to see if current token is a semicolon
            current_token = &tokens[*current];
            if current_token.token_type != TokenType::SEMICOLON {
                throw_error(&format!("Syntax Error on line {}: Expected semicolon \";\"",
                                          &tokens[*current - 1].line_num));
            }

        } else {
            throw_error(&format!("Syntax Error on line {}: variable declaration must end with a semicolon \";\"",
                                      current_token.line_num));
        }
    }

    // Consume the semicolon token and move on to the next one
    consume_token(current);

    // If we made it to here, we must have successfully parsed the variable declaration,
    // so return the newly created node!
    return var_decl_node;
}


// identifier              : ID
//                         ;
pub fn identifier_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    if current_token.token_type != TokenType::ID {
        throw_error(&format!("Syntax Error on line {}: expected an identifier",
                    current_token.line_num));
    }

    // Consume this token and move on to the next one
    consume_token(current);

    // Return an identifier AST node corresponding to the ID token
    return ASTNode::new("id",
                    Some(current_token.lexeme.clone()),
                    Some(current_token.line_num));
}


// functiondeclaration     : functionheader block
//                         ;
pub fn functiondeclaration_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // Create function declaration node
    let mut new_node = ASTNode::new("funcDecl",
                                             None,
                                         Some(current_token.line_num));
    
    // Add child through function header
    new_node.add_children(functionheader_(tokens, current));

    // Add child for block
    new_node.add_child(block_(tokens, current));

    // Return function declaration node
    return new_node;
}


// functionheader          : FUNC functiondeclarator RETURNS [type | VOID]
//                         ;
pub fn functionheader_(tokens: &Vec<Token>, current: &mut usize) -> Vec<ASTNode> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create a vector to hold the AST nodes
    let mut node_vec = Vec::new();

    // A function header always starts with a "func" keyword, otherwise we have a syntax error
    if current_token.token_type != TokenType::FUNC {
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
    if current_token.token_type != TokenType::RETURNS {
        throw_error(&format!("Syntax Error on line {}: expected \"returns\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "returns" keyword, so we can consume it
    consume_token(current);

    // Create a node to hold the return value of the function
    let mut returns_node = ASTNode::new("returns", None, None);

    current_token = &tokens[*current];
    if current_token.token_type == TokenType::VOID {
        returns_node.add_child(ASTNode::new("void",
                                                     Some(String::from("void")),
                                                     Some(current_token.line_num)));
        
        // Consume void token
        consume_token(current);

    } else {
        // Otherwise we should see a type
        returns_node.add_child(type_(tokens, current));
    }

    // Add the return node to the list
    node_vec.push(returns_node);

    // Finally we can return our function header nodes
    return node_vec;
}


// functiondeclarator      : identifier OPENPAR {formalparameterlist} CLOSEPAR
//                         ;
pub fn functiondeclarator_(tokens: &Vec<Token>, current: &mut usize) -> Vec<ASTNode> {
    // Create a vector to hold the AST nodes
    let mut node_vec = Vec::new();

    // Add node for function token_type (identifier)
    node_vec.push(identifier_(tokens, current));

    // Next we should see an open parenthesis:
    let mut current_token = &tokens[*current];
    if current_token.token_type != TokenType::OPENPAR {
        throw_error(&format!("Syntax Error on line {}: function token_type must be followed by a parameter list enclosed in parentheses \"(\" \")\"",
                    current_token.line_num));
    }

    // Otherwise we found a "(", so we can consume it
    consume_token(current);

    // Now we can start parsing the parameter list
    let mut param_list = ASTNode::new("parameters", None, None);
    
    // Add one child for each parameter in the list
    param_list.add_children(formalparameterlist_(tokens, current));

    // Add param list to function declarator node
    node_vec.push(param_list);

    // Next we should see an close parenthesis:
    current_token = &tokens[*current];
    if current_token.token_type != TokenType::CLOSEPAR {
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
pub fn formalparameterlist_(tokens: &Vec<Token>, current: &mut usize) -> Vec<ASTNode> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create a vector to hold the AST nodes
    let mut param_list = Vec::new();

    if current_token.token_type == TokenType::CLOSEPAR {
        // If the current token is a close parenthesis, this function has no parameters and we can return an empty list
        return param_list;
    }

    // Otherwise, we have at least one parameter that we need to parse
    param_list.push(formalparameter_(tokens, current));

    // Loop through more parameters until we reach the close parenthesis
    current_token = &tokens[*current];

    while current_token.token_type != TokenType::CLOSEPAR {

        if current_token.token_type == TokenType::COMMA {
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
pub fn formalparameter_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    let mut param = ASTNode::new("parameter",
                                          None,
                                      Some(current_token.line_num));

    // Add child for parameter type
    param.add_child(type_(tokens, current));

    // Add child for parameter identifier
    param.add_child(identifier_(tokens, current));

    return param;
}


// mainfunctiondeclaration : FUNC mainfunctiondeclarator RETURNS VOID block
//                         ;
pub fn mainfunctiondeclaration_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create function declaration node
    let mut main_decl_node = ASTNode::new("mainFuncDecl",
                                             None,
                                         Some(current_token.line_num));

    // A function declaration always starts with a "func" keyword, otherwise we have a syntax error
    if current_token.token_type != TokenType::FUNC {
        throw_error(&format!("Syntax Error on line {}: main function declaration must always start with a \"func\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "func" keyword, so we can consume it
    consume_token(current);
    
    // Parse main function declarator
    main_decl_node.add_child(mainfunctiondeclarator_(tokens, current));

    // Add "parameters" node, even though it doesn't take any params, just so it can have the same format as a regular funcDecl
    main_decl_node.add_child(ASTNode::new("parameters", None, None));

    // Next we should see the "returns" keyword
    current_token = &tokens[*current];
    if current_token.token_type != TokenType::RETURNS {
        throw_error(&format!("Syntax Error on line {}: expected \"returns\" keyword",
                    current_token.line_num));
    }

    // Otherwise we found a "returns" keyword, so we can consume it
    consume_token(current);

    // Create a node to hold the return value of the function
    let mut returns_node = ASTNode::new("returns", None, None);

    current_token = &tokens[*current];
    if current_token.token_type == TokenType::VOID {
        returns_node.add_child(ASTNode::new("void",
                                                     Some(String::from("void")),
                                                     Some(current_token.line_num)));
        
        // Consume void token
        consume_token(current);

    } else {
        throw_error(&format!("Syntax Error on line {}: main function must return \"void\"",
                    current_token.line_num));
    }

    // Add returns node to main declaration node
    main_decl_node.add_child(returns_node);

    // Add child for block
    main_decl_node.add_child(block_(tokens, current));

    // Return function declaration node
    return main_decl_node;
}


// mainfunctiondeclarator  : MAIN OPENPAR CLOSEPAR
//                         ;
pub fn mainfunctiondeclarator_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    // Main function must be called "main"
    if current_token.token_type != TokenType::MAIN {
        throw_error(&format!("Syntax Error on line {}: main function must be called \"main\"",
                    current_token.line_num));
    }

    // Otherwise, we found a "main" keyword, so we can consume it
    consume_token(current);
    current_token = &tokens[*current];

    // "main" keyword must be followed by "()"
    if current_token.token_type != TokenType::OPENPAR || tokens[*current + 1].token_type != TokenType::CLOSEPAR {
        throw_error(&format!("Syntax Error on line {}: \"main\" keyword must be followed by \"()\"",
                    current_token.line_num));
    }

    // Otherwise, we found a pair of tokens "()", so we can consume them
    consume_token(current);
    consume_token(current);
    current_token = &tokens[*current];

    return ASTNode::new("id", Some(String::from("main")), Some(current_token.line_num));
}


// block                   : OPENBRACE {blockstatements} CLOSEBRACE
//                         ;
pub fn block_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    let mut block_node = ASTNode::new("block", None, Some(current_token.line_num));

    // A block should always start with an open brace
    if current_token.token_type != TokenType::OPENBRACE {
        throw_error(&format!("Syntax Error on line {}: expected an open brace \"{{\"",
                    current_token.line_num));
    }

    // Otherwise, we found an open brace token, so we can consume it
    consume_token(current);

    // Add block statements as children to our block node
    block_node.add_children(blockstatements_(tokens, current));

    // A block should always end with a close brace
    current_token = &tokens[*current];
    if current_token.token_type != TokenType::CLOSEBRACE {
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
pub fn blockstatements_(tokens: &Vec<Token>, current: &mut usize) -> Vec<ASTNode> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create vector to hold block statement nodes
    let mut statement_vec = Vec::new();

    // Blocks cannot be empty, so if the first token we see is a close brace, we have a syntax error:
    if current_token.token_type == TokenType::CLOSEBRACE {
        throw_error(&format!("Syntax Error on line {}: block cannot be empty",
                    current_token.line_num));
    }

    // Otherwise, we have a non-empty block, so we can loop until we find that close brace
    while current_token.token_type != TokenType::CLOSEBRACE {
        statement_vec.push(blockstatement_(tokens, current));
        current_token = &tokens[*current];
    }

    return statement_vec;
}


// blockstatement          : variabledeclaration
//                         | statement
//                         ;
pub fn blockstatement_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // A block statement can either be a variable declaration or a statement
    // If it is a variable declaration, the first token we will find is a type (int or bool)
    if current_token.token_type == TokenType::INT || current_token.token_type == TokenType::BOOL {
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
pub fn statement_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    match current_token.token_type {
        // If the statement is a block, the first token we see is an open brace
        TokenType::OPENBRACE => {return block_(tokens, current);}

        // If the statement is a void statement, the first token we see is a semicolon
        TokenType::SEMICOLON => {
            // Consume semicolon token
            consume_token(current);
            current_token = &tokens[*current];

            return ASTNode::new("voidStmt", None, Some(current_token.line_num));
        }

        // If the statement is a statement expression (which can be either an assignment or a function call),
        // the first token we see is an identifier
        TokenType::ID => {
            // Parse statement expression
            let stmt_expr = statementexpression_(tokens, current);

            // Statement expression must be followed by a semicolon
            current_token = &tokens[*current];
            if current_token.token_type != TokenType::SEMICOLON {
                throw_error(&format!("Syntax Error on line {}: expression must end with a semicolon",
                            tokens[*current - 1].line_num));
            }

            // Otherwise, consume semicolon token
            consume_token(current);

            return stmt_expr;
        }

        // If the statement is a break statement, the first token we see is a BREAK token
        TokenType::BREAK => {
            // Consume break token
            consume_token(current);
            current_token = &tokens[*current];

            // Break statement must be followed by a semicolon
            if current_token.token_type != TokenType::SEMICOLON {
                throw_error(&format!("Syntax Error on line {}: break statement must end with a semicolon",
                            current_token.line_num));
            }

            // Otherwise, consume semicolon token
            consume_token(current);

            return ASTNode::new("break", None, Some(tokens[*current - 2].line_num));
        }

        // If the statement is a return statement, the first token we see is a RETURN token
        TokenType::RETURN => {
            // Consume return token
            consume_token(current);
            current_token = &tokens[*current];

            if current_token.token_type == TokenType::SEMICOLON {
                // We have an empty return statement, consume semicolon token
                consume_token(current);
                current_token = &tokens[*current];

                return ASTNode::new("return", None, Some(current_token.line_num));

            } else {
                let mut return_node = ASTNode::new("return", None, Some(current_token.line_num));

                return_node.add_child(expression_(tokens, current));

                // Return statement must end with a semicolon
                current_token = &tokens[*current];
                if current_token.token_type != TokenType::SEMICOLON {
                    throw_error(&format!("Syntax Error on line {}: return statement must end with a semicolon",
                                current_token.line_num));
                }

                // Otherwise, consume semicolon token
                consume_token(current);

                return return_node;
            }
        }

        // If the statement is an if or if-else statement, the first token we see is an IF token
        TokenType::IF => {
            // Get line number of the IF token
            let if_line_num = current_token.line_num;

            // Consume if token
            consume_token(current);
            
            // Parse if expression
            let if_expr_node = expression_(tokens, current);

            // Parse if body
            let statement_node = statement_(tokens, current);

            // Check if this is an if statement or an if-else statement
            current_token = &tokens[*current];
            if current_token.token_type != TokenType::ELSE {
                // If there is no else, create the if node
                let mut if_node = ASTNode::new("if", None, Some(if_line_num));

                // Add the expression and statement nodes
                if_node.add_child(if_expr_node);
                if_node.add_child(statement_node);

                // Return if node
                return if_node;
            } else {
                // If there is an else, create an if-else node and continue parsing
                let mut if_else_node = ASTNode::new("ifElse", None, Some(if_line_num));

                // Add the expression and statement nodes
                if_else_node.add_child(if_expr_node);
                if_else_node.add_child(statement_node);

                // Consume else token
                consume_token(current);

                // Add the else statement
                if_else_node.add_child(statement_(tokens, current));

                // Return if-else node
                return if_else_node;
            }
        }

        // If the statement is a while loop, the first token we see is a WHILE token
        TokenType::WHILE => {
            // Consume while token
            consume_token(current);

            // Create while node
            let mut while_node = ASTNode::new("while", None, Some(current_token.line_num));

            // Add the expression node
            while_node.add_child(expression_(tokens, current));

            // Add the body of the loop
            while_node.add_child(statement_(tokens, current));

            return while_node;
        }

        // If the first token we see is MAIN, the user is probably trying to call the main function
        TokenType::MAIN => {
            throw_error(&format!("Line {}: main function cannot be invoked",
                        current_token.line_num));
            
            // Return dummy node to avoid the compiler getting angry with me
            return ASTNode::new("statement", None, None);
        }

        // Otherwise, we have a syntax error
        _ => {
            throw_error(&format!("Syntax Error on line {}: not a valid statement",
                        current_token.line_num));
            
            // Return dummy node to avoid the compiler getting angry with me
            return ASTNode::new("statement", None, None);
        }
    }
}


// statementexpression     : assignment
//                         | functioninvocation
//                         ;
pub fn statementexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get next token
    let token_2 = &tokens[*current + 1];

    // If we have a function invocation, the second token should be an open parenthesis
    if token_2.token_type == TokenType::OPENPAR {
        return functioninvocation_(tokens, current);
    } else {
        // Otherwise, we have an assignment
        return assignment_(tokens, current);
    }
}


// primary                 : literal
//                         | OPENPAR expression CLOSEPAR
//                         | functioninvocation
//                         ;
pub fn primary_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    if current_token.token_type == TokenType::OPENPAR {
        // Consume open parenthesis token
        consume_token(current);

        // Parse expression
        let expr_node = expression_(tokens, current);

        // Make sure the open parenthesis is matched by a close parenthesis
        current_token = &tokens[*current];
        if current_token.token_type != TokenType::CLOSEPAR {
            throw_error(&format!("Syntax Error on line {}: missing close parenthesis",
                        current_token.line_num));
        }

        // Otherwise, consume close parenthesis token
        consume_token(current);

        return expr_node;

    } else if tokens[*current + 1].token_type == TokenType::OPENPAR {
        // We have a function invocation
        return functioninvocation_(tokens, current);

    } else {
        // We have a literal
        return literal_(tokens, current);
    }
}


// argumentlist            : expression
//                         | argumentlist COMMA expression
//                         ;
pub fn argumentlist_(tokens: &Vec<Token>, current: &mut usize) -> Vec<ASTNode> {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create a vector to hold the AST nodes
    let mut arg_list = Vec::new();

    if current_token.token_type == TokenType::CLOSEPAR {
        // If the current token is a close parenthesis, this function call has no arguments and we can return an empty list
        return arg_list;
    }

    // Otherwise, we have at least one argument that we need to parse
    let mut arg = ASTNode::new("argument", None, None);
    arg.add_child(expression_(tokens, current));
    arg_list.push(arg);

    // Loop through more parameters until we reach the close parenthesis
    current_token = &tokens[*current];

    while current_token.token_type != TokenType::CLOSEPAR {

        if current_token.token_type == TokenType::COMMA {
            // Consume comma token and then parse the following parameter
            consume_token(current);
            let mut arg = ASTNode::new("argument", None, None);
            arg.add_child(expression_(tokens, current));
            arg_list.push(arg);

            // Update current token
            current_token = &tokens[*current];

        } else {
            throw_error(&format!("Syntax Error on line {}: function call argument list must be a comma separated list of expressions",
                        current_token.line_num));
        }
    }

    return arg_list;
}


// functioninvocation      : identifier OPENPAR argumentlist CLOSEPAR
//                         | identifier OPENPAR CLOSEPAR
//                         ;
pub fn functioninvocation_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let mut current_token = &tokens[*current];

    // Create function invocation node
    let mut func_inv_node = ASTNode::new("funcCall", None, Some(current_token.line_num));

    // Add function identifier as child
    func_inv_node.add_child(identifier_(tokens, current));

    // Next, we should see an open parenthesis
    current_token = &tokens[*current];
    if current_token.token_type != TokenType::OPENPAR {
        throw_error(&format!("Syntax Error on line {}: function call token_type must be followed by an open parenthesis",
                    current_token.line_num));
    }

    // Otherwise, consume the open parenthesis token
    consume_token(current);

    // Add argument list
    let mut arg_list = ASTNode::new("arguments", None, None);
    arg_list.add_children(argumentlist_(tokens, current));
    func_inv_node.add_child(arg_list);

    // Finally, we should see an close parenthesis
    current_token = &tokens[*current];
    if current_token.token_type != TokenType::CLOSEPAR {
        throw_error(&format!("Syntax Error on line {}: function call argument list must be followed by a close parenthesis",
                    current_token.line_num));
    }

    // Otherwise, consume the close parenthesis token
    consume_token(current);

    return func_inv_node;
}


// postfixexpression       : primary
//                         | identifier
//                         ;
pub fn postfixexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // A postfix expression can either be a primary or an identifier
    // A primary can be a literal (first token is INTLIT, STRLIT, TRUE, or FALSE),
    // an expression surrounded by parentheses (first token is OPENPAR),
    // or a function invocation (second token is OPENPAR)
    if current_token.token_type == TokenType::INTLIT ||
    current_token.token_type == TokenType::STRLIT ||
    current_token.token_type == TokenType::TRUE ||
    current_token.token_type == TokenType::FALSE ||
    current_token.token_type == TokenType::OPENPAR ||
    tokens[*current + 1].token_type == TokenType::OPENPAR {
        return primary_(tokens, current);
    } else {
        return identifier_(tokens, current);
    }
}


// unaryexpression         : MINUS unaryexpression
//                         | NOT unaryexpression
//                         | postfixexpression
//                         ;
pub fn unaryexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Get current token
    let current_token = &tokens[*current];

    // A unary expression can either start with a -, a !, or just be a postfix expression
    if current_token.token_type == TokenType::MINUS {
        // Consume minus token
        consume_token(current);

        // Create unary minus node
        let mut unary_minus_node = ASTNode::new("u-", None, Some(current_token.line_num));
        
        // Add RHS expression as child
        unary_minus_node.add_child(unaryexpression_(tokens, current));
        
        // Return node
        return unary_minus_node;

    } else if  current_token.token_type == TokenType::NOT {
        // Consume not token
        consume_token(current);

        // Create unary not node
        let mut unary_not_node = ASTNode::new("!", None, Some(current_token.line_num));
        
        // Add RHS expression as child
        unary_not_node.add_child(unaryexpression_(tokens, current));
        
        // Return node
        return unary_not_node;

    } else {
        return postfixexpression_(tokens, current);
    }
}


// multiplicativeexpression: unaryexpression multiplicativerhs
//                         ;
pub fn multiplicativeexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse expression on left hand side
    let lhs = unaryexpression_(tokens, current);

    // Parse expression on right hand side (starting with <, >, <=, >=, or is empty)
    let rhs = multiplicativerhs_(tokens, current);

    match rhs {
        // If there is no right hand side, we can simply return the left hand side
        None => {
            return lhs;
        }
        // If there is a right hand side, rhs is a "<" or ">" or "<=" or ">=" node and the lhs should be the first child of it
        Some(mut rhs) => {
            rhs.add_child_to_front(lhs);
            return rhs;
        }
    }
}


// multiplicativerhs		: MULT unaryexpression multiplicativerhs
//  						| DIV unaryexpression multiplicativerhs
//  						| MOD unaryexpression multiplicativerhs
//  						| /* nothing */
//   						;
pub fn multiplicativerhs_(tokens: &Vec<Token>, current: &mut usize) -> Option<ASTNode> {
    // Get current token
    let current_token = &tokens[*current];

    // Either we see an mult token, or we return nothing
    if current_token.token_type == TokenType::MULT ||
       current_token.token_type == TokenType::DIV ||
       current_token.token_type == TokenType::MOD {
        // Consume token
        consume_token(current);

        let mut mult_node;

        // Make correct kind of node
        if current_token.token_type == TokenType::MULT {
            mult_node = ASTNode::new("*", None, Some(current_token.line_num));
        } else if current_token.token_type == TokenType::DIV {
            mult_node = ASTNode::new("/", None, Some(current_token.line_num));
        } else {
            mult_node = ASTNode::new("%", None, Some(current_token.line_num));
        }

        // get right hand side of rel
        let rhs = unaryexpression_(tokens, current);

        // Get what might be another rel
        let possible_mult = multiplicativerhs_(tokens, current);

        match possible_mult {
            // If there is no other rel, we can simply return the rhs
            None => {
                mult_node.add_child(rhs);
                return Some(mult_node);
            }
            // If there is another mult, mult is a "*" or "/" or "%" node and the rhs should be the first child of it
            Some(mut mult) => {
                mult.add_child_to_front(rhs);
                mult_node.add_child(mult);
                return Some(mult_node);
            }
        }
    } else {
        return None;
    }
}


// additiveexpression      : multiplicativeexpression additiverhs
//                         ;
pub fn additiveexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse expression on left hand side
    let lhs = multiplicativeexpression_(tokens, current);

    // Parse expression on right hand side (starting with ==, !=, or is empty)
    let rhs = additiverhs_(tokens, current);

    match rhs {
        // If there is no right hand side, we can simply return the left hand side
        None => {
            return lhs;
        }
        // If there is a right hand side, rhs is a "+" or "-" node and the lhs should be the first child of it
        Some(mut rhs) => {
            rhs.add_child_to_front(lhs);
            return rhs;
        }
    }
}


// additiverhs				: PLUS multiplicativeexpression additiverhs
// 						    | MINUS multiplicativeexpression additiverhs
// 						    | /* nothing */
// 						    ;
pub fn additiverhs_(tokens: &Vec<Token>, current: &mut usize) -> Option<ASTNode> {
    // Get current token
    let current_token = &tokens[*current];

    // Either we see an PLUS or MINUS token, or we return nothing
    if current_token.token_type == TokenType::PLUS || current_token.token_type == TokenType::MINUS {
        // Consume token
        consume_token(current);

        let mut add_node;

        // Make correct kind of node
        if current_token.token_type == TokenType::PLUS {
            add_node = ASTNode::new("+", None, Some(current_token.line_num));
        } else {
            add_node = ASTNode::new("-", None, Some(current_token.line_num));
        }

        // get right hand side of add
        let rhs = multiplicativeexpression_(tokens, current);

        // Get what might be another add
        let possible_eq = additiverhs_(tokens, current);

        match possible_eq {
            // If there is no other add, we can simply return the rhs
            None => {
                add_node.add_child(rhs);
                return Some(add_node);
            }
            // If there is another add, add is a "+" or "-" node and the rhs should be the first child of it
            Some(mut add) => {
                add.add_child_to_front(rhs);
                add_node.add_child(add);
                return Some(add_node);
            }
        }
    } else {
        return None;
    }
}


// relationalexpression    : additiveexpression relationalrhs
//                         ;
pub fn relationalexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse expression on left hand side
    let lhs = additiveexpression_(tokens, current);

    // Parse expression on right hand side (starting with <, >, <=, >=, or is empty)
    let rhs = relationalrhs_(tokens, current);

    match rhs {
        // If there is no right hand side, we can simply return the left hand side
        None => {
            return lhs;
        }
        // If there is a right hand side, rhs is a "<" or ">" or "<=" or ">=" node and the lhs should be the first child of it
        Some(mut rhs) => {
            rhs.add_child_to_front(lhs);
            return rhs;
        }
    }
}


// relationalrhs			: LT additiveexpression relationalrhs
// 						    | GT additiveexpression relationalrhs
// 						    | LEQ additiveexpression relationalrhs
// 						    | GEQ additiveexpression relationalrhs
// 						    | /* nothing */
// 						    ;
pub fn relationalrhs_(tokens: &Vec<Token>, current: &mut usize) -> Option<ASTNode> {
    // Get current token
    let current_token = &tokens[*current];

    // Either we see an relational token, or we return nothing
    if current_token.token_type == TokenType::LT ||
       current_token.token_type == TokenType::GT ||
       current_token.token_type == TokenType::LEQ ||
       current_token.token_type == TokenType::GEQ {
        // Consume token
        consume_token(current);

        let mut rel_node;

        // Make correct kind of node
        if current_token.token_type == TokenType::LT {
            rel_node = ASTNode::new("<", None, Some(current_token.line_num));
        } else if current_token.token_type == TokenType::GT {
            rel_node = ASTNode::new(">", None, Some(current_token.line_num));
        } else if current_token.token_type == TokenType::LEQ {
            rel_node = ASTNode::new("<=", None, Some(current_token.line_num));
        } else {
            rel_node = ASTNode::new(">=", None, Some(current_token.line_num));
        }

        // get right hand side of rel
        let rhs = additiveexpression_(tokens, current);

        // Get what might be another rel
        let possible_rel = relationalrhs_(tokens, current);

        match possible_rel {
            // If there is no other rel, we can simply return the rhs
            None => {
                rel_node.add_child(rhs);
                return Some(rel_node);
            }
            // If there is another rel, rel is a "<" or ">" or "<=" or ">=" node and the rhs should be the first child of it
            Some(mut rel) => {
                rel.add_child_to_front(rhs);
                rel_node.add_child(rel);
                return Some(rel_node);
            }
        }
    } else {
        return None;
    }
}


// equalityexpression      : relationalexpression equalityrhs
//                         ;
pub fn equalityexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse expression on left hand side
    let lhs = relationalexpression_(tokens, current);

    // Parse expression on right hand side (starting with ==, !=, or is empty)
    let rhs = equalityrhs_(tokens, current);

    match rhs {
        // If there is no right hand side, we can simply return the left hand side
        None => {
            return lhs;
        }
        // If there is a right hand side, rhs is a "==" or "!=" node and the lhs should be the first child of it
        Some(mut rhs) => {
            rhs.add_child_to_front(lhs);
            return rhs;
        }
    }
}


// equalityrhs				: EQ relationalexpression equalityrhs
// 						    | NEQ relationalexpression equalityrhs
// 						    | /* nothing */
// 						    ;
pub fn equalityrhs_(tokens: &Vec<Token>, current: &mut usize) -> Option<ASTNode> {
    // Get current token
    let current_token = &tokens[*current];

    // Either we see an EQ or NEQ token, or we return nothing
    if current_token.token_type == TokenType::EQ || current_token.token_type == TokenType::NEQ {
        // Consume token
        consume_token(current);

        let mut eq_node;

        // Make correct kind of node
        if current_token.token_type == TokenType::EQ {
            eq_node = ASTNode::new("==", None, Some(current_token.line_num));
        } else {
            eq_node = ASTNode::new("!=", None, Some(current_token.line_num));
        }

        // get right hand side of eq
        let rhs = relationalexpression_(tokens, current);

        // Get what might be another eq
        let possible_eq = equalityrhs_(tokens, current);

        match possible_eq {
            // If there is no other eq, we can simply return the rhs
            None => {
                eq_node.add_child(rhs);
                return Some(eq_node);
            }
            // If there is another eq, eq is a "==" or "!=" node and the rhs should be the first child of it
            Some(mut eq) => {
                eq.add_child_to_front(rhs);
                eq_node.add_child(eq);
                return Some(eq_node);
            }
        }
    } else {
        return None;
    }
}


// conditionalandexpression: equalityexpression conditionalandrhs
//                         ;
pub fn conditionalandexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse expression on left hand side
    let lhs = equalityexpression_(tokens, current);

    // Parse expression on right hand side (either starting with AND, or is empty)
    let rhs = conditionalandrhs_(tokens, current);

    match rhs {
        // If there is no right hand side, we can simply return the left hand side
        None => {
            return lhs;
        }
        // If there is a right hand side, rhs is a "&&" node and the lhs should be the first child of it
        Some(mut rhs) => {
            rhs.add_child_to_front(lhs);
            return rhs;
        }
    }
}


// conditionalandrhs		: {AND equalityexpression conditionalandrhs}
// 						    ;
pub fn conditionalandrhs_(tokens: &Vec<Token>, current: &mut usize) -> Option<ASTNode> {
    // Get current token
    let current_token = &tokens[*current];

    // Either we see an AND token, or we return nothing
    if current_token.token_type == TokenType::AND {
        // Consume AND token
        consume_token(current);

        // Create an and node
        let mut and_node = ASTNode::new("&&", None, Some(current_token.line_num));

        // get right hand side of AND
        let rhs = equalityexpression_(tokens, current);

        // Get what might be another AND
        let possible_and = conditionalandrhs_(tokens, current);

        match possible_and {
            // If there is no other and, we can simply return the rhs
            None => {
                and_node.add_child(rhs);
                return Some(and_node);
            }
            // If there is another or, or is a "||" node and the rhs should be the first child of it
            Some(mut and) => {
                and.add_child_to_front(rhs);
                and_node.add_child(and);
                return Some(and_node);
            }
        }
    } else {
        return None;
    }
}


// conditionalorexpression : conditionalandexpression conditionalorrhs
//                         ;
pub fn conditionalorexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse expression on left hand side
    let lhs = conditionalandexpression_(tokens, current);

    // Parse expression on right hand side (either starting with OR, or is empty)
    let rhs = conditionalorrhs_(tokens, current);

    match rhs {
        // If there is no right hand side, we can simply return the left hand side
        None => {
            return lhs;
        }
        // If there is a right hand side, rhs is a "||" node and the lhs should be the first child of it
        Some(mut rhs) => {
            rhs.add_child_to_front(lhs);
            return rhs;
        }
    }
}


// conditionalorrhs		: {OR conditionalandexpression conditionalorrhs}
//                      ;
pub fn conditionalorrhs_(tokens: &Vec<Token>, current: &mut usize) -> Option<ASTNode> {
    // Get current token
    let current_token = &tokens[*current];

    // Either we see an OR token, or we return nothing
    if current_token.token_type == TokenType::OR {
        // Consume OR token
        consume_token(current);

        // Create an or node
        let mut or_node = ASTNode::new("||", None, Some(current_token.line_num));

        // get right hand side of OR
        let rhs = conditionalandexpression_(tokens, current);

        // Get what might be another OR
        let possible_or = conditionalorrhs_(tokens, current);

        match possible_or {
            // If there is no other or, we can simply return the rhs
            None => {
                or_node.add_child(rhs);
                return Some(or_node);
            }
            // If there is another or, or is a "||" node and the rhs should be the first child of it
            Some(mut or) => {
                or.add_child_to_front(rhs);
                or_node.add_child(or);
                return Some(or_node);
            }
        }
    } else {
        return None;
    }
}


// assignmentexpression    : conditionalorexpression
//                         | assignment
//                         ;
pub fn assignmentexpression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // The second token of an expression is =, +=, -=, etc...
    let token_2 = &tokens[*current + 1];

    if token_2.token_type == TokenType::ASSIGN ||
       token_2.token_type == TokenType::PLUSEQ ||
       token_2.token_type == TokenType::MINUSEQ ||
       token_2.token_type == TokenType::MULTEQ ||
       token_2.token_type == TokenType::DIVEQ ||
       token_2.token_type == TokenType::MODEQ {
        // We have an assignment
        return assignment_(tokens, current);

    } else {
        // Otherwise, we have to continue parsing the expression
        return conditionalorexpression_(tokens, current);
    }
}


// assignment              : identifier ASSIGN assignmentexpression
// 						   : identifier PLUSEQ INTLIT
// 						   : identifier MINUSEQ INTLIT
// 						   : identifier MULTEQ INTLIT
// 						   : identifier DIVEQ INTLIT
// 						   : identifier MODEQ INTLIT
// 						   : identifier POWEREQ INTLIT
//                         ;
pub fn assignment_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    // Parse identifier on LHS of assignment
    let id_node = identifier_(tokens, current);

    // The token of the assignment, for example, =, +=, -=, etc...
    let assign_token = &tokens[*current];

    match assign_token.token_type {
        TokenType::ASSIGN => {
            // Create assignment node and attach the LHS id node
            let mut assign_node = ASTNode::new("=", None, Some(assign_token.line_num));
            assign_node.add_child(id_node);

            // Consume assignment token
            consume_token(current);

            // Attach the RHS node
            assign_node.add_child(assignmentexpression_(tokens, current));

            // Return the assignment node
            return assign_node;
        }

        TokenType::PLUSEQ => {
            // Create plus-equal node and attach the LHS id node
            let mut assign_node = ASTNode::new("+=", None, Some(assign_token.line_num));
            assign_node.add_child(id_node);

            // Consume plus-equal token
            consume_token(current);
            let current_token = &tokens[*current];

            // Plus-equal must be followed by an integer literal
            if current_token.token_type != TokenType::INTLIT {
                throw_error(&format!("Syntax Error on line {}: += statement must be followed by an integer literal",
                            current_token.line_num));
            }

            // Otherwise, now that we know this token is an integer literal, we can call literal_() and attach the node
            assign_node.add_child(literal_(tokens, current));

            // Return the plus-equal node
            return assign_node;
        }

        TokenType::MINUSEQ => {
            // Create minus-equal node and attach the LHS id node
            let mut assign_node = ASTNode::new("-=", None, Some(assign_token.line_num));
            assign_node.add_child(id_node);

            // Consume minus-equal token
            consume_token(current);
            let current_token = &tokens[*current];

            // Minus-equal must be followed by an integer literal
            if current_token.token_type != TokenType::INTLIT {
                throw_error(&format!("Syntax Error on line {}: -= statement must be followed by an integer literal",
                            current_token.line_num));
            }

            // Otherwise, now that we know this token is an integer literal, we can call literal_() and attach the node
            assign_node.add_child(literal_(tokens, current));

            // Return the minus-equal node
            return assign_node;
        }

        TokenType::MULTEQ => {
            // Create multiply-equal node and attach the LHS id node
            let mut assign_node = ASTNode::new("*=", None, Some(assign_token.line_num));
            assign_node.add_child(id_node);

            // Consume multiply-equal token
            consume_token(current);
            let current_token = &tokens[*current];

            // Multiply-equal must be followed by an integer literal
            if current_token.token_type != TokenType::INTLIT {
                throw_error(&format!("Syntax Error on line {}: *= statement must be followed by an integer literal",
                            current_token.line_num));
            }

            // Otherwise, now that we know this token is an integer literal, we can call literal_() and attach the node
            assign_node.add_child(literal_(tokens, current));

            // Return the multiply-equal node
            return assign_node;
        }

        TokenType::DIVEQ => {
            // Create divide-equal node and attach the LHS id node
            let mut assign_node = ASTNode::new("/=", None, Some(assign_token.line_num));
            assign_node.add_child(id_node);

            // Consume divide-equal token
            consume_token(current);
            let current_token = &tokens[*current];

            // Divide-equal must be followed by an integer literal
            if current_token.token_type != TokenType::INTLIT {
                throw_error(&format!("Syntax Error on line {}: /= statement must be followed by an integer literal",
                            current_token.line_num));
            }

            // Otherwise, now that we know this token is an integer literal, we can call literal_() and attach the node
            assign_node.add_child(literal_(tokens, current));

            // Return the divide-equal node
            return assign_node;
        }

        TokenType::MODEQ => {
            // Create modulus-equal node and attach the LHS id node
            let mut assign_node = ASTNode::new("%=", None, Some(assign_token.line_num));
            assign_node.add_child(id_node);

            // Consume modulus-equal token
            consume_token(current);
            let current_token = &tokens[*current];

            // Modulus-equal must be followed by an integer literal
            if current_token.token_type != TokenType::INTLIT {
                throw_error(&format!("Syntax Error on line {}: %= statement must be followed by an integer literal",
                            current_token.line_num));
            }

            // Otherwise, now that we know this token is an integer literal, we can call literal_() and attach the node
            assign_node.add_child(literal_(tokens, current));

            // Return the Modulus-equal node
            return assign_node;
        }

        _ => {
            throw_error(&format!("Syntax Error on line {}: Invalid assignment statement, must be one of =, +=, -=, *=, /=, %=, or ^=",
                        assign_token.line_num));
            
            return ASTNode::new("assignment", None, None);
        }
    }
}


// expression              : assignmentexpression
//                         ;
pub fn expression_(tokens: &Vec<Token>, current: &mut usize) -> ASTNode {
    return assignmentexpression_(tokens, current);
}