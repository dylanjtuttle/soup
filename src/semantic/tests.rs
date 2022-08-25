mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::semantic::semantic_data::{ScopeStack, Symbol};


    #[test]
    fn test_is_in_scope_true() {
        let mut scope_stack = ScopeStack::new();
        scope_stack.open_scope();

        scope_stack.insert_symbol(
            String::from("symbol1"),
            Rc::new(RefCell::new(Symbol::new(
                String::from("symbol1"),
                String::from("type"),
                String::from("returns"),
            ))),
        );

        assert!(scope_stack.is_in_scope("symbol1"));
    }

    #[test]
    fn test_is_in_scope_empty_scope() {
        let mut scope_stack = ScopeStack::new();
        scope_stack.open_scope();

        assert!(!scope_stack.is_in_scope("symbol1"));
    }

    #[test]
    fn test_is_in_scope_no_scope() {
        let mut scope_stack = ScopeStack::new();

        assert!(!scope_stack.is_in_scope("symbol1"));
    }

    #[test]
    fn test_find_symbol() {
        let mut scope_stack = ScopeStack::new();
        scope_stack.open_scope();

        let test_symbol = Rc::new(RefCell::new(Symbol::new(
            String::from("symbol1"),
            String::from("type"),
            String::from("returns"),
        )));

        scope_stack.insert_symbol(String::from("symbol1"), test_symbol.clone());

        assert_eq!(Some(test_symbol), scope_stack.find_symbol("symbol1"));
    }

    #[test]
    fn test_find_symbol_first() {
        // Adding two symbols with the same name to different scopes
        // find_symbol() should return the one in the topmost scope

        // Open one scope, add a symbol
        let mut scope_stack = ScopeStack::new();
        scope_stack.open_scope();

        let symbol1 = Rc::new(RefCell::new(Symbol::new(
            String::from("symbol"),
            String::from(""),
            String::from(""),
        )));

        scope_stack.insert_symbol(String::from("symbol"), symbol1);


        // Open another scope, add a symbol with the same name
        scope_stack.open_scope();

        let symbol2 = Rc::new(RefCell::new(Symbol::new(
            String::from("symbol"),
            String::from("type_sig"),
            String::from("returns"),
        )));

        scope_stack.insert_symbol(String::from("symbol"), symbol2.clone());

        assert_eq!(Some(symbol2), scope_stack.find_symbol("symbol"));
    }

    #[test]
    fn test_find_symbol_empty_scope() {
        let mut scope_stack = ScopeStack::new();
        scope_stack.open_scope();

        assert_eq!(None, scope_stack.find_symbol("symbol1"));
    }

    #[test]
    fn test_find_symbol_no_scope() {
        let scope_stack = ScopeStack::new();

        assert_eq!(None, scope_stack.find_symbol("symbol1"));
    }
}