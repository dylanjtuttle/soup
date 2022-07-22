mod tests {
    use crate::scanner::{scanner_utils::*, scanner_data::{Char, Token, TokenType}};

    #[test]
    fn test_get_separators() {
        let open_p = vec![Char{char_val: '(', line_num: 1}];
        let open_b = vec![Char{char_val: '{', line_num: 1}];
        let semi = vec![Char{char_val: ';', line_num: 1}];
        let comma = vec![Char{char_val: ',', line_num: 1}];
        
        assert_eq!(Some(Token {token_type: TokenType::OPENPAR, lexeme: String::from("("), line_num: 1}), get_separators(&open_p, &mut 0));
        assert_eq!(Some(Token {token_type: TokenType::OPENBRACE, lexeme: String::from("{"), line_num: 1}), get_separators(&open_b, &mut 0));
        assert_eq!(Some(Token {token_type: TokenType::SEMICOLON, lexeme: String::from(";"), line_num: 1}), get_separators(&semi, &mut 0));
        assert_eq!(Some(Token {token_type: TokenType::COMMA, lexeme: String::from(","), line_num: 1}), get_separators(&comma, &mut 0));
    }

    #[test]
    fn test_get_binary_ops() {
        let div = vec![Char{char_val: '/', line_num: 1},
                                  Char{char_val: ' ', line_num: 1}];
                        
        let div_eq = vec![Char{char_val: '/', line_num: 1},
                                     Char{char_val: '=', line_num: 1},
                                     Char{char_val: ' ', line_num: 1}];

        let comment = vec![Char{char_val: '/', line_num: 1},
                                      Char{char_val: '/', line_num: 1},
                                      Char{char_val: ' ', line_num: 1},
                                      Char{char_val: '\n', line_num: 1}];

        let expected_div = Token {token_type: TokenType::DIV,
                                         lexeme: String::from("/"),
                                         line_num: 1};

        let expected_diveq = Token {token_type: TokenType::DIVEQ,
                                           lexeme: String::from("/="),
                                           line_num: 1};
        
        assert_eq!(Some(expected_div), get_binary_ops(&div, &mut 0));
        assert_eq!(Some(expected_diveq), get_binary_ops(&div_eq, &mut 0));
        assert_eq!(None, get_binary_ops(&comment, &mut 0));
    }

    #[test]
    fn test_get_binary_op() {
        let div = vec![Char{char_val: '/', line_num: 1},
                                  Char{char_val: ' ', line_num: 1}];
                        
        let div_eq = vec![Char{char_val: '/', line_num: 1},
                                     Char{char_val: '=', line_num: 1},
                                     Char{char_val: ' ', line_num: 1}];

        let comment = vec![Char{char_val: '/', line_num: 1},
                                      Char{char_val: '/', line_num: 1},
                                      Char{char_val: ' ', line_num: 1},
                                      Char{char_val: '\n', line_num: 1}];

        let expected_div = Token {token_type: TokenType::DIV,
                                         lexeme: String::from("/"),
                                         line_num: 1};

        let expected_diveq = Token {token_type: TokenType::DIVEQ,
                                           lexeme: String::from("/="),
                                           line_num: 1};
        
        assert_eq!(Some(expected_div), get_binary_op(&div, &mut 0, TokenType::DIV, TokenType::DIVEQ, "/"));
        assert_eq!(Some(expected_diveq), get_binary_op(&div_eq, &mut 0, TokenType::DIV, TokenType::DIVEQ, "/"));
        assert_eq!(None, get_binary_op(&comment, &mut 0, TokenType::DIV, TokenType::DIVEQ, "/"));
    }

    #[test]
    fn test_get_and_or() {
        let and = vec![Char{char_val: '&', line_num: 1},
                                  Char{char_val: '&', line_num: 1}];

        let or = vec![Char{char_val: '|', line_num: 1},
                                 Char{char_val: '|', line_num: 1}];

        let expected_and = Token {token_type: TokenType::AND,
                                         lexeme: String::from("&&"),
                                         line_num: 1};

        let expected_or = Token {token_type: TokenType::OR,
                                        lexeme: String::from("||"),
                                        line_num: 1};

        assert_eq!(Some(expected_and), get_and_or(&and, &mut 0, TokenType::AND, "&&"));
        assert_eq!(Some(expected_or), get_and_or(&or, &mut 0, TokenType::OR, "||"));
    }

    #[test]
    fn test_get_reserved_words() {
        let reserved = vec![Char{char_val: 'i', line_num: 1},
                                       Char{char_val: 'n', line_num: 1},
                                       Char{char_val: 't', line_num: 1},
                                       Char{char_val: ' ', line_num: 1}];
                        
        let not_reserved = vec![Char{char_val: 'n', line_num: 1},
                                           Char{char_val: 'o', line_num: 1},
                                           Char{char_val: 't', line_num: 1}];

        let expected_token = Token {token_type: TokenType::INT,
                                           lexeme: String::from("int"),
                                           line_num: 1};
        
        let mut index = 0;
        assert_eq!(Some(expected_token), get_reserved_words(&reserved, &mut index));
        assert_eq!(None, get_reserved_words(&not_reserved, &mut index));
    }

    #[test]
    fn test_get_reserved_word() {
        let reserved = vec![Char{char_val: 'i', line_num: 1},
                                       Char{char_val: 'f', line_num: 1},
                                       Char{char_val: ' ', line_num: 1}];
                        
        let not_reserved = vec![Char{char_val: 'i', line_num: 1},
                                           Char{char_val: 'f', line_num: 1},
                                           Char{char_val: '_', line_num: 1}];

        let expected_token = Token {token_type: TokenType::IF,
                                           lexeme: String::from("if"),
                                           line_num: 1};
        
        let mut index = 0;
        assert_eq!(Some(expected_token), get_reserved_word(&reserved, &mut index, TokenType::IF, "if"));
        assert_eq!(None, get_reserved_word(&not_reserved, &mut index, TokenType::IF, "if"));
    }

    #[test]
    fn test_is_reserved() {
        let reserved = vec!['i', 'f', ' '];
        let not_reserved = vec!['i', 'f', '_'];

        assert!(is_reserved(reserved, "if"));
        assert!(!is_reserved(not_reserved, "if"));
    }
    
    #[test]
    fn test_has_enough_chars() {
        let maybe_reserved = vec![Char{char_val: 'i', line_num: 1},
                                             Char{char_val: 'f', line_num: 1},
                                             Char{char_val: ' ', line_num: 1}];
           
        assert!(has_enough_chars(&maybe_reserved, &mut 0, 2));
        assert!(!has_enough_chars(&maybe_reserved, &mut 0, 5));
    }

    #[test]
    fn test_get_identifier() {
        let identifier = vec![Char{char_val: 'I', line_num: 1},
                                         Char{char_val: 'd', line_num: 1},
                                         Char{char_val: '_', line_num: 1},
                                         Char{char_val: '1', line_num: 1},
                                         Char{char_val: ' ', line_num: 1}];

        let expected_token = Token {token_type: TokenType::ID,
                                           lexeme: String::from("Id_1"),
                                           line_num: 1};
 
        let mut index = 0;
        assert_eq!(expected_token, get_identifier(&identifier, &mut index));
        // Ensure we moved the index far enough (should now point at the final ' ' char)
        assert_eq!(4, index);
    }

    #[test]
    fn test_get_int_lits() {
        let int_lit = vec![Char{char_val: '0', line_num: 1},
                                      Char{char_val: '9', line_num: 1},
                                      Char{char_val: '2', line_num: 1},
                                      Char{char_val: '6', line_num: 1},
                                      Char{char_val: '8', line_num: 1},
                                      Char{char_val: ';', line_num: 1}];
        
        let expected_token = Token {token_type: TokenType::INTLIT,
                                           lexeme: String::from("09268"),
                                           line_num: 1};

        let mut index = 0;
        assert_eq!(expected_token, get_int_lits(&int_lit, &mut index));
        // Ensure we moved the index far enough (should now point at the final ' ' char)
        assert_eq!(5, index);
    }

    #[test]
    fn test_get_str_lits() {
        let str_lit = vec![Char{char_val: '"', line_num: 1},
                                      Char{char_val: 'H', line_num: 1},
                                      Char{char_val: 'e', line_num: 1},
                                      Char{char_val: 'l', line_num: 1},
                                      Char{char_val: 'l', line_num: 1},
                                      Char{char_val: 'o', line_num: 1},
                                      Char{char_val: '!', line_num: 1},
                                      Char{char_val: '\n', line_num: 1},
                                      Char{char_val: '"', line_num: 1},
                                      Char{char_val: ' ', line_num: 1}];

        let expected_token = Token {token_type: TokenType::STRLIT,
                                           lexeme: String::from("Hello!\n"),
                                           line_num: 1};

        let mut index = 0;
        assert_eq!(expected_token, get_str_lits(&str_lit, &mut index));
        // Ensure we moved the index far enough (should now point at the final ' ' char)
        assert_eq!(9, index);
    }

    #[test]
    fn test_is_id_char() {
        let test_chars = vec![('A', true), ('a', true), ('0', true), ('_', true),
                                                 (' ', false), ('\t', false), ('\r', false), ('\n', false),
                                                 ('(', false), ('{', false), (';', false), (',', false),
                                                 ('+', false), ('-', false), ('*', false), ('/', false), ('%', false),
                                                 ('=', false), ('!', false), ('&', false), ('|', false), ('<', false), ('>', false)];
        
        for char_pair in test_chars {
            assert_eq!(is_id_char(char_pair.0), char_pair.1);
        }
    }

    #[test]
    fn test_is_digit() {
        let test_chars = vec![('0', true), ('1', true), ('2', true), ('3', true), ('4', true),
                                                 ('5', true), ('6', true), ('7', true), ('8', true), ('9', true), 
                                                 ('A', false), ('a', false), ('_', false),
                                                 (' ', false), ('\t', false), ('\r', false), ('\n', false),
                                                 ('(', false), ('{', false), (';', false), (',', false),
                                                 ('+', false), ('-', false), ('*', false), ('/', false), ('%', false),
                                                 ('=', false), ('!', false), ('&', false), ('|', false), ('<', false), ('>', false)];
        
        for char_pair in test_chars {
            assert_eq!(is_digit(char_pair.0), char_pair.1);
        }
    }
}