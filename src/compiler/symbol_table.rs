use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    GlobalScope,
    LocalScope,
}

impl Display for SymbolScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolScope::GlobalScope => write!(f, "GLOBAL"),
            SymbolScope::LocalScope => write!(f, "LOCAL"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

impl Symbol {
    pub fn new(name: impl Into<String>, scope: SymbolScope, index: usize) -> Self {
        Symbol {
            name: name.into(),
            scope,
            index,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<SymbolTable>>>,
    pub store: HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let store = HashMap::new();
        SymbolTable {
            outer: None,
            store,
            num_definitions: 0,
        }
    }

    pub fn new_with_enclosed(outer: Rc<RefCell<SymbolTable>>) -> Self {
        let store = HashMap::new();
        SymbolTable {
            outer: Some(outer),
            store,
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: impl Into<String>) -> Symbol {
        let scope = if self.outer.is_some() {
            SymbolScope::LocalScope
        } else {
            SymbolScope::GlobalScope
        };
        let symbol = Symbol::new(name, scope, self.num_definitions);
        self.store.insert(symbol.name.clone(), symbol.clone());
        self.num_definitions += 1;
        symbol
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        self.store.get(name).cloned().or(self
            .outer
            .clone()
            .and_then(|store| store.borrow_mut().resolve(name)))
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
