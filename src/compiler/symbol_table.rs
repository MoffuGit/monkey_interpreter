use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    GlobalScope,
}

impl Display for SymbolScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolScope::GlobalScope => write!(f, "GLOBAL"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: i64,
}

impl Symbol {
    pub fn new(name: impl Into<String>, scope: SymbolScope, index: i64) -> Self {
        Symbol {
            name: name.into(),
            scope,
            index,
        }
    }
}

pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    num_definitions: i64,
}

impl SymbolTable {
    pub fn new() -> Self {
        let store = HashMap::new();
        SymbolTable {
            store,
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: impl Into<String>) -> Symbol {
        let symbol = Symbol::new(name, SymbolScope::GlobalScope, self.num_definitions);
        self.store.insert(symbol.name.clone(), symbol.clone());
        self.num_definitions += 1;
        symbol
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        self.store.get(name).cloned()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
