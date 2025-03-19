use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::symbol_table::{Symbol, SymbolScope, SymbolTable};

#[test]
fn test_define() {
    let expected = HashMap::from([
        (
            "a".to_string(),
            Symbol::new("a", SymbolScope::GlobalScope, 0),
        ),
        (
            "b".to_string(),
            Symbol::new("b", SymbolScope::GlobalScope, 1),
        ),
        (
            "c".to_string(),
            Symbol::new("c", SymbolScope::LocalScope, 0),
        ),
        (
            "d".to_string(),
            Symbol::new("d", SymbolScope::LocalScope, 1),
        ),
        (
            "e".to_string(),
            Symbol::new("e", SymbolScope::LocalScope, 0),
        ),
        (
            "f".to_string(),
            Symbol::new("f", SymbolScope::LocalScope, 1),
        ),
    ]);

    let global = Rc::new(RefCell::new(SymbolTable::new()));

    assert_eq!(Some(&global.borrow_mut().define("a")), expected.get("a"));
    assert_eq!(Some(&global.borrow_mut().define("b")), expected.get("b"));

    let mut first_local = SymbolTable::new_with_enclosed(global.clone());

    assert_eq!(Some(&first_local.define("c")), expected.get("c"));
    assert_eq!(Some(&first_local.define("d")), expected.get("d"));

    let mut second_local = SymbolTable::new_with_enclosed(global.clone());

    assert_eq!(Some(&second_local.define("e")), expected.get("e"));
    assert_eq!(Some(&second_local.define("f")), expected.get("f"));

    // let mut symbols = global.store.into_values().collect::<Vec<Symbol>>();
    // symbols.sort_by(|a, b| a.index.cmp(&b.index));
    // let mut expected_symbols = expected.into_values().collect::<Vec<Symbol>>();
    // expected_symbols.sort_by(|a, b| a.index.cmp(&b.index));
    //
    // assert_eq!(symbols, expected_symbols)
}

#[test]
fn test_resolve_global() {
    let mut global = SymbolTable::new();

    global.define("a");
    global.define("b");

    let expected = [
        Symbol::new("a", SymbolScope::GlobalScope, 0),
        Symbol::new("b", SymbolScope::GlobalScope, 1),
    ];

    for expected_symbol in expected.iter() {
        let symbol = global.resolve(&expected_symbol.name);
        assert_eq!(symbol, Some(expected_symbol.clone()));
    }
}

#[test]
fn test_resolve_local() {
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let mut local = SymbolTable::new_with_enclosed(global.clone());
    local.define("c");
    local.define("d");
    let expected = [
        Symbol::new("a", SymbolScope::GlobalScope, 0),
        Symbol::new("b", SymbolScope::GlobalScope, 1),
        Symbol::new("c", SymbolScope::LocalScope, 0),
        Symbol::new("d", SymbolScope::LocalScope, 1),
    ];
    for expected_symbol in expected.iter() {
        let symbol = local.resolve(&expected_symbol.name);
        assert_eq!(symbol, Some(expected_symbol.clone()));
    }
}

#[test]
fn test_resolve_nested_local() {
    struct Test {
        table: SymbolTable,
        expected: Vec<Symbol>,
    }
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let first_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(global.clone())));
    first_local.borrow_mut().define("c");
    first_local.borrow_mut().define("d");
    let mut second_local = SymbolTable::new_with_enclosed(first_local.clone());
    second_local.define("e");
    second_local.define("f");
    let tests = [
        Test {
            table: first_local.borrow().clone(),
            expected: vec![
                Symbol::new("a", SymbolScope::GlobalScope, 0),
                Symbol::new("b", SymbolScope::GlobalScope, 1),
                Symbol::new("c", SymbolScope::LocalScope, 0),
                Symbol::new("d", SymbolScope::LocalScope, 1),
            ],
        },
        Test {
            table: second_local,
            expected: vec![
                Symbol::new("a", SymbolScope::GlobalScope, 0),
                Symbol::new("b", SymbolScope::GlobalScope, 1),
                Symbol::new("e", SymbolScope::LocalScope, 0),
                Symbol::new("f", SymbolScope::LocalScope, 1),
            ],
        },
    ];
    for Test {
        mut table,
        expected,
    } in tests
    {
        for expected_symbol in expected.iter() {
            let symbol = table.resolve(&expected_symbol.name);
            assert_eq!(symbol, Some(expected_symbol.clone()));
        }
    }
}

#[test]
fn test_define_resolve_builtins() {
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    let first_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(global.clone())));
    let second_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(
        first_local.clone(),
    )));

    let expected = [
        Symbol::new("a", SymbolScope::BuiltinScope, 0),
        Symbol::new("b", SymbolScope::BuiltinScope, 1),
        Symbol::new("e", SymbolScope::BuiltinScope, 2),
        Symbol::new("f", SymbolScope::BuiltinScope, 3),
    ];

    for (idx, expect) in expected.iter().enumerate() {
        global.borrow_mut().define_builtin(idx, expect.name.clone());
    }

    for table in [global, first_local, second_local] {
        for symbol in expected.iter() {
            assert_eq!(
                table.borrow_mut().resolve(&symbol.name),
                Some(symbol.clone())
            );
        }
    }
}

#[test]
fn test_resolve_free() {
    struct Test {
        table: Rc<RefCell<SymbolTable>>,
        expected_symbols: Vec<Symbol>,
        expected_free: Vec<Symbol>,
    }
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    global.borrow_mut().define("a");
    global.borrow_mut().define("b");

    let first_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(global.clone())));
    first_local.borrow_mut().define("c");
    first_local.borrow_mut().define("d");

    let second_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(
        first_local.clone(),
    )));
    second_local.borrow_mut().define("e");
    second_local.borrow_mut().define("f");
    let tests = vec![
        Test {
            table: first_local,
            expected_symbols: vec![
                Symbol::new("a", SymbolScope::GlobalScope, 0),
                Symbol::new("b", SymbolScope::GlobalScope, 1),
                Symbol::new("c", SymbolScope::LocalScope, 0),
                Symbol::new("d", SymbolScope::LocalScope, 1),
            ],
            expected_free: vec![],
        },
        Test {
            table: second_local,
            expected_symbols: vec![
                Symbol::new("a", SymbolScope::GlobalScope, 0),
                Symbol::new("b", SymbolScope::GlobalScope, 1),
                Symbol::new("c", SymbolScope::FreeScope, 0),
                Symbol::new("d", SymbolScope::FreeScope, 1),
                Symbol::new("e", SymbolScope::LocalScope, 0),
                Symbol::new("f", SymbolScope::LocalScope, 1),
            ],
            expected_free: vec![
                Symbol::new("c", SymbolScope::LocalScope, 0),
                Symbol::new("d", SymbolScope::LocalScope, 1),
            ],
        },
    ];

    for test in tests {
        for expected in test.expected_symbols {
            assert_eq!(
                Some(expected.clone()),
                test.table.borrow_mut().resolve(&expected.name)
            )
        }

        assert_eq!(test.expected_free, test.table.borrow_mut().free_symbols);
    }
}

#[test]
fn test_resolve_unresolvable_free() {
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    global.borrow_mut().define("a");

    let first_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(global.clone())));
    first_local.borrow_mut().define("c");

    let second_local = Rc::new(RefCell::new(SymbolTable::new_with_enclosed(
        first_local.clone(),
    )));
    second_local.borrow_mut().define("e");
    second_local.borrow_mut().define("f");
    let expected = vec![
        Symbol::new("a", SymbolScope::GlobalScope, 0),
        Symbol::new("c", SymbolScope::FreeScope, 0),
        Symbol::new("e", SymbolScope::LocalScope, 0),
        Symbol::new("f", SymbolScope::LocalScope, 1),
    ];

    for symbol in expected {
        assert_eq!(
            Some(symbol.clone()),
            second_local.borrow_mut().resolve(&symbol.name)
        );
    }
    let expected_unresolvable = vec!["b", "d"];
    for name in expected_unresolvable {
        assert!(second_local.borrow_mut().resolve(name).is_none());
    }
}

#[test]
fn test_define_and_resolve_function_name() {
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    global.borrow_mut().define_function("a");

    let expected = Symbol::new("a", SymbolScope::FunctionScope, 0);

    assert_eq!(global.borrow_mut().resolve(&expected.name), Some(expected));
}

#[test]
fn test_shadow_function_name() {
    let global = Rc::new(RefCell::new(SymbolTable::new()));
    global.borrow_mut().define_function("a");
    global.borrow_mut().define("a");
    let expected = Symbol::new("a", SymbolScope::GlobalScope, 0);
    assert_eq!(global.borrow_mut().resolve(&expected.name), Some(expected));
}
