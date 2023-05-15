use std::{collections::HashMap, ops::Deref};

#[derive(Clone)]
pub struct Symbol {
    pub derivations: Vec<Derivation>,
}

impl Symbol {
    pub fn new() -> Symbol {
        Symbol {
            derivations: Vec::new(),
        }
    }

    pub fn add_derivation(&mut self, derivation: Derivation) {
        self.derivations.push(derivation);
    }
}

#[derive(Clone)]
pub struct Derivation(Vec<String>);
impl Deref for Derivation {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Derivation {
    pub fn from(derivation: Vec<String>) -> Derivation {
        Derivation(derivation)
    }
}

pub struct Grammer {
    pub symbols: HashMap<String, Symbol>,
}

impl Grammer {
    pub fn get_dummy_start_name(&self) -> &str {
        &DUMMY_START_SYMBOL
    }

    pub fn get_real_start_name(&self) -> &str {
        let dummy_start = self.get_symbol(&self.get_dummy_start_name()).unwrap();
        &dummy_start.derivations[0][0]
    }

    pub fn get_symbol(&self, symbol_name: &str) -> Option<&Symbol> {
        self.symbols.get(symbol_name)
    }
}

pub struct GrammerBuilder {
    pub start: Option<String>,
    pub symbols: HashMap<String, Symbol>,
}

pub const DUMMY_START_SYMBOL: &str = "__START__";
impl GrammerBuilder {
    pub fn new() -> GrammerBuilder {
        GrammerBuilder {
            start: None,
            symbols: HashMap::new(),
        }
    }

    pub fn add_derivation(
        &mut self,
        symbol_name: &str,
        derivation: Derivation,
    ) -> &mut GrammerBuilder {
        // if the symbol doesn't exist, add it
        if let Some(symbol) = self.symbols.get_mut(symbol_name) {
            symbol.add_derivation(derivation);
        } else {
            let mut symbol = Symbol::new();
            symbol.add_derivation(derivation);
            self.symbols.insert(symbol_name.to_string(), symbol);
        }
        self
    }

    pub fn set_start(&mut self, symbol_name: &str) -> &mut GrammerBuilder {
        // if the symbol doesn't exist, add it
        if !self.symbols.contains_key(symbol_name) {
            self.symbols.insert(symbol_name.to_string(), Symbol::new());
        }
        self.start = Some(symbol_name.to_string());
        self
    }

    pub fn build(&mut self) -> Grammer {
        if let Some(start) = self.start.take() {
            // add a dummy start symbol
            let mut dummy = Symbol::new();
            dummy.add_derivation(Derivation::from(vec![start.clone()]));
            self.symbols.insert(DUMMY_START_SYMBOL.into(), dummy);

            // add the dericated symbols which are not in the grammer yet
            let mut new_symbols = Vec::new();
            self.symbols.iter().for_each(|(_, symbol)| {
                symbol.derivations.iter().for_each(|derivation| {
                    derivation.iter().for_each(|symbol_name| {
                        if !self.symbols.contains_key(symbol_name) {
                            new_symbols.push(symbol_name.clone());
                        }
                    });
                });
            });
            new_symbols.iter().for_each(|symbol_name| {
                self.symbols.insert(symbol_name.clone(), Symbol::new());
            });

            Grammer {
                symbols: self.symbols.clone(),
            }
        } else {
            panic!("No start symbol set");
        }
    }
}
