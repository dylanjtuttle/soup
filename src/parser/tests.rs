mod tests {
    use crate::parser::parser_data::ASTNode;
    use crate::parser::parser_grammar::*;
    use crate::scanner::scanner_data::{Token, TokenType};

    #[test]
    fn test_function_header() {
        // func test_func() returns void {;}
        let tokens = vec![
            Token {
                token_type: TokenType::FUNC,
                lexeme: String::from("func"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::ID,
                lexeme: String::from("test_func"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::OPENPAR,
                lexeme: String::from("("),
                line_num: 1,
            },
            Token {
                token_type: TokenType::CLOSEPAR,
                lexeme: String::from(")"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::RETURNS,
                lexeme: String::from("returns"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::VOID,
                lexeme: String::from("void"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::OPENBRACE,
                lexeme: String::from("{"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: String::from(";"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::CLOSEBRACE,
                lexeme: String::from("}"),
                line_num: 1,
            },
        ];

        let mut func = ASTNode::new("funcDecl", None, Some(1));
        let id = ASTNode::new("id", Some(String::from("test_func")), Some(1));
        let params = ASTNode::new("parameters", None, None);
        let mut returns = ASTNode::new("returns", None, None);
        let void = ASTNode::new("void", Some(String::from("void")), Some(1));
        let mut block = ASTNode::new("block", None, Some(1));
        let void_stmt = ASTNode::new("voidStmt", None, Some(1));

        returns.add_child(void);
        block.add_child(void_stmt);
        func.add_children(vec![id, params, returns, block]);

        assert_eq!(func, functiondeclaration_(&tokens, &mut 0));
    }

    #[test]
    fn test_binary_operator_precedence() {
        // 1 + 2 * 3;
        let tokens = vec![
            Token {
                token_type: TokenType::INTLIT,
                lexeme: String::from("1"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::PLUS,
                lexeme: String::from("+"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::INTLIT,
                lexeme: String::from("2"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::MULT,
                lexeme: String::from("*"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::INTLIT,
                lexeme: String::from("3"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: String::from(";"),
                line_num: 1,
            },
        ];

        let mut plus = ASTNode::new("+", None, Some(1));
        let mut times = ASTNode::new("*", None, Some(1));
        let one = ASTNode::new("number", Some(String::from("1")), Some(1));
        let two = ASTNode::new("number", Some(String::from("2")), Some(1));
        let three = ASTNode::new("number", Some(String::from("3")), Some(1));

        // * is evaluated first, and so is lower down on the tree
        times.add_child(two);
        times.add_child(three);

        // Next we evaluate +
        plus.add_child(one);
        plus.add_child(times);

        assert_eq!(plus, assignmentexpression_(&tokens, &mut 0));
    }

    #[test]
    fn test_assignmentexpression() {
        // x = 1;
        let mut tokens = vec![
            Token {
                token_type: TokenType::ID,
                lexeme: String::from("x"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::ASSIGN,
                lexeme: String::from("="),
                line_num: 1,
            },
            Token {
                token_type: TokenType::INTLIT,
                lexeme: String::from("1"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::SEMICOLON,
                lexeme: String::from(";"),
                line_num: 1,
            },
        ];

        let mut assign = ASTNode::new("=", None, Some(1));
        let id = ASTNode::new("id", Some(String::from("x")), Some(1));
        let intlit = ASTNode::new("number", Some(String::from("1")), Some(1));

        assign.add_child(id);
        assign.add_child(intlit);

        assert_eq!(assign, assignmentexpression_(&tokens, &mut 0));

        // x <= 1;
        tokens[1] = Token {
            token_type: TokenType::LEQ,
            lexeme: String::from("<="),
            line_num: 1,
        };
        assign.node_type = String::from("<=");
        assign.attr = None;

        assert_eq!(assign, assignmentexpression_(&tokens, &mut 0));
    }

    #[test]
    fn test_get_func_sig() {
        let mut root = ASTNode::new("funcDecl", None, None);
        let id = ASTNode::new("id", None, None);
        let params = ASTNode::new("parameters", None, None);
        let mut param = ASTNode::new("parameter", None, None);
        let mut int = ASTNode::new("int", None, None);
        int.type_sig = Some(String::from("int"));

        root.add_child(id);
        root.add_child(params);

        assert_eq!(String::from("f()"), root.get_func_sig());

        param.add_child(int);
        root.children[1].add_child(param);

        assert_eq!(String::from("f(int)"), root.get_func_sig());

        let mut param2 = ASTNode::new("parameter", None, None);
        let mut bool = ASTNode::new("bool", None, None);
        bool.type_sig = Some(String::from("bool"));

        param2.add_child(bool);
        root.children[1].add_child(param2);

        assert_eq!(String::from("f(int, bool)"), root.get_func_sig());
    }

    #[test]
    fn test_has_nonempty_return() {
        let mut root = ASTNode::new("funcDecl", None, None);
        let mut return_node = ASTNode::new("return", None, None);
        let return_val = ASTNode::new("id", None, None);
        return_node.add_child(return_val);
        root.add_child(return_node);

        assert!(root.has_nonempty_return());
        assert!(!ASTNode::new("funcDecl", None, None).has_nonempty_return());
    }

    #[test]
    fn test_literal() {
        let tokens = vec![
            Token {
                token_type: TokenType::INTLIT,
                lexeme: String::from("0"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::STRLIT,
                lexeme: String::from("hello world"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::TRUE,
                lexeme: String::from("true"),
                line_num: 1,
            },
            Token {
                token_type: TokenType::FALSE,
                lexeme: String::from("false"),
                line_num: 1,
            }
        ];

        let intlit_node = ASTNode::new(
            "number",
            Some(String::from("0")),
            Some(1),
        );

        let strlit_node = ASTNode::new(
            "string",
            Some(String::from("hello world")),
            Some(1),
        );

        let true_node = ASTNode::new(
            "true",
            Some(String::from("true")),
            Some(1),
        );

        let false_node = ASTNode::new(
            "false",
            Some(String::from("false")),
            Some(1),
        );

        assert_eq!(intlit_node, literal_(&tokens, &mut 0));
        assert_eq!(strlit_node, literal_(&tokens, &mut 1));
        assert_eq!(true_node, literal_(&tokens, &mut 2));
        assert_eq!(false_node, literal_(&tokens, &mut 3));
    }
}
