use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    GlobalScope,
    LocalScope,
    BuiltinScope,
    FreeScope,
    FunctionScope,
}

impl Display for SymbolScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolScope::GlobalScope => write!(f, "GLOBAL"),
            SymbolScope::LocalScope => write!(f, "LOCAL"),
            SymbolScope::BuiltinScope => write!(f, "BUILTIN"),
            SymbolScope::FreeScope => write!(f, "FREE"),
            SymbolScope::FunctionScope => write!(f, "FUNCTION"),
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
    pub free_symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let store = HashMap::new();
        SymbolTable {
            outer: None,
            store,
            num_definitions: 0,
            free_symbols: vec![],
        }
    }

    pub fn new_with_enclosed(outer: Rc<RefCell<SymbolTable>>) -> Self {
        let store = HashMap::new();
        SymbolTable {
            outer: Some(outer),
            store,
            num_definitions: 0,
            free_symbols: vec![],
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

    pub fn define_builtin(&mut self, index: usize, name: String) -> Symbol {
        let symbol = Symbol {
            name,
            index,
            scope: SymbolScope::BuiltinScope,
        };
        self.store.insert(symbol.name.clone(), symbol.clone());
        symbol
    }

    pub fn define_free(&mut self, original: Symbol) -> Symbol {
        self.free_symbols.push(original.clone());

        let symbol = Symbol::new(
            &original.name,
            SymbolScope::FreeScope,
            self.free_symbols.len() - 1,
        );
        self.store.insert(original.name, symbol.clone());
        symbol
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        self.store
            .get(name)
            .cloned()
            .or(self.outer.clone().and_then(|store| {
                store.borrow_mut().resolve(name).map(|symbol| {
                    if symbol.scope == SymbolScope::GlobalScope
                        || symbol.scope == SymbolScope::BuiltinScope
                        || symbol.scope == SymbolScope::FunctionScope
                    {
                        return symbol;
                    }

                    self.define_free(symbol)
                })
            }))
    }

    pub fn define_function(&mut self, name: &str) -> Option<Symbol> {
        let symbol = Symbol::new(name, SymbolScope::FunctionScope, 0);
        self.store.insert(name.into(), symbol.clone());
        Some(symbol)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
