use std::collections::HashMap;

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
    ]);

    let mut global = SymbolTable::new();

    assert_eq!(Some(&global.define("a")), expected.get("a"));
    assert_eq!(Some(&global.define("b")), expected.get("b"));

    let mut symbols = global.store.into_values().collect::<Vec<Symbol>>();
    symbols.sort_by(|a, b| a.index.cmp(&b.index));
    let mut expected_symbols = expected.into_values().collect::<Vec<Symbol>>();
    expected_symbols.sort_by(|a, b| a.index.cmp(&b.index));

    assert_eq!(symbols, expected_symbols)
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
