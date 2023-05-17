use crate::Derivation;
use std::{cell::RefCell, collections::HashSet, hash::Hash, hash::Hasher, ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub derivations: Vec<Derivation>,
    pub first_set: HashSet<SymbolRef>,
    pub follow_set: HashSet<SymbolRef>,
}

impl Symbol {
    pub fn build(name: &str) -> Self {
        Symbol {
            name: name.to_string(),
            derivations: Vec::new(),
            first_set: HashSet::new(),
            follow_set: HashSet::new(),
        }
    }

    pub fn add_derivation(&mut self, derivation: Derivation) -> &mut Self {
        self.derivations.push(derivation);
        self
    }

    pub fn is_terminal(&self) -> bool {
        self.derivations.len() != 0
    }
}

pub struct SymbolRef(Rc<RefCell<Symbol>>);

impl SymbolRef {
    pub fn build(name: &str) -> Self {
        SymbolRef(Rc::new(RefCell::new(Symbol::build(name))))
    }
}

impl Deref for SymbolRef {
    type Target = Rc<RefCell<Symbol>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for SymbolRef {
    fn clone(&self) -> Self {
        SymbolRef(Rc::clone(&self.0))
    }
}

impl PartialEq for SymbolRef {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().name == other.borrow().name
    }
}

impl Eq for SymbolRef {}

impl Hash for SymbolRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow().name.hash(state);
    }
}
