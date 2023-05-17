use crate::SymbolRef;
use std::{hash::Hash, ops::Deref};

#[derive(Clone)]
pub struct Derivation(Vec<SymbolRef>);
impl Deref for Derivation {
    type Target = Vec<SymbolRef>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Derivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Derivation {}

impl Hash for Derivation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub struct DerivationBuilder {
    derivation: Vec<SymbolRef>,
}

impl DerivationBuilder {
    pub fn new() -> DerivationBuilder {
        DerivationBuilder {
            derivation: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: &SymbolRef) -> &mut DerivationBuilder {
        self.derivation.push(SymbolRef::clone(symbol));
        self
    }

    pub fn build(&self) -> Derivation {
        Derivation(self.derivation.clone())
    }
}
