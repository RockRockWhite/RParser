use crate::{Derivation, DerivationBuilder, SymbolRef};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Grammer {
    pub symbols: HashMap<String, SymbolRef>,
    pub start: SymbolRef,
}

pub struct GrammerBuilder {
    pub start: Option<SymbolRef>,
    pub symbols: RefCell<HashMap<String, SymbolRef>>,
}

impl GrammerBuilder {
    pub const EPSILON_SYMBOL: &str = "__EPSILON__";
    const DUMMY_START_SYMBOL: &str = "__DUMMY_START__";

    pub fn new() -> GrammerBuilder {
        GrammerBuilder {
            start: None,
            symbols: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_symbol(&mut self, symbol_name: &str) -> SymbolRef {
        if symbol_name == Self::DUMMY_START_SYMBOL {
            panic!("Symbol {} is reserved", Self::DUMMY_START_SYMBOL);
        }

        if !self.symbols.borrow().contains_key(symbol_name) {
            self.symbols
                .borrow_mut()
                .insert(symbol_name.to_string(), SymbolRef::build(symbol_name));
        }
        SymbolRef::clone(self.symbols.borrow().get(symbol_name).unwrap())
    }

    pub fn set_start(&mut self, symbol: &SymbolRef) -> &mut GrammerBuilder {
        // check if the symbol exists
        if !self.symbols.borrow().contains_key(&symbol.borrow().name) {
            panic!("Symbol {} doesn't exist", symbol.borrow().name);
        }

        self.start = Some(SymbolRef::clone(symbol));
        self
    }

    pub fn add_derivation(
        &mut self,
        symbol: &SymbolRef,
        derivation: Derivation,
    ) -> &mut GrammerBuilder {
        // check if the symbol exists
        if !self.symbols.borrow().contains_key(&symbol.borrow().name) {
            panic!("Symbol {} doesn't exist", &symbol.borrow().name);
        }

        symbol.borrow_mut().add_derivation(derivation);
        self
    }

    pub fn build(&mut self) -> Grammer {
        if let Some(start) = self.start.take() {
            // add a dummy start symbol
            // dummy -> start
            self.symbols.borrow_mut().insert(
                Self::DUMMY_START_SYMBOL.into(),
                SymbolRef::build(Self::DUMMY_START_SYMBOL),
            );
            let dummy =
                SymbolRef::clone(self.symbols.borrow().get(Self::DUMMY_START_SYMBOL).unwrap());
            dummy.borrow_mut().derivations = Vec::new();
            dummy
                .borrow_mut()
                .add_derivation(DerivationBuilder::new().add_symbol(&start).build());

            // calculate the first and follow set
            self.calculate_first_and_follow();

            for each in self.symbols.borrow().values() {
                // 输出每个符号的first集合
                println!(
                    "{}: {:?}",
                    each.borrow().name,
                    each.borrow()
                        .first_set
                        .iter()
                        .map(|x| x.borrow().name.clone())
                        .collect::<Vec<_>>()
                );
            }

            Grammer {
                symbols: self.symbols.borrow().clone(),
                start: dummy,
            }
        } else {
            panic!("No start symbol set");
        }
    }

    /// Calculate the first and follow set for each symbol
    /// This function should be called when the grammer is built
    fn calculate_first_and_follow(&mut self) {
        // calculate the first set
        self.calculate_first();
    }

    /// Calculate the first set for each symbol
    /// This function should be called when the grammer is built
    fn calculate_first(&mut self) {
        // for all the terminal symbols, the first set is itself
        self.symbols
            .borrow_mut()
            .values()
            .into_iter()
            .filter(|symbol| symbol.borrow().is_terminal())
            .for_each(|symbol| {
                let mut first: HashSet<SymbolRef> = HashSet::new();
                first.insert(SymbolRef::clone(symbol));
                symbol.borrow_mut().first_set = first;
            });

        loop {
            let mut changed = false;

            // tarverse all the derivations
            self.symbols
                .borrow_mut()
                .values()
                .into_iter()
                .filter(|symbol| !symbol.borrow().is_terminal())
                .for_each(|symbol| {
                    let mut first: HashSet<SymbolRef> = symbol.borrow_mut().first_set.clone();

                    symbol
                        .borrow_mut()
                        .derivations
                        .iter()
                        .for_each(|derication| {
                            // for the first symbol in the right hand side of the derivation
                            for right_each in derication.iter() {
                                // if it is a terminal symbol, add it to the first set, then break
                                if right_each.borrow().is_terminal() {
                                    first.insert(SymbolRef::clone(right_each));
                                    break;
                                }

                                // if the right_each is a non-terminal symbol
                                // add the first set of it to the first set
                                first.extend(right_each.borrow().first_set.clone());

                                // if the right_each's first set contains epsilon, continue
                                if !first.contains(&SymbolRef::build(Self::EPSILON_SYMBOL)) {
                                    break;
                                }
                            }
                        });

                    // if the first set is changed, set the changed flag to true
                    if first != symbol.borrow().first_set {
                        changed = true;
                        symbol.borrow_mut().first_set = first;
                    }
                });

            if !changed {
                break;
            }
        }
    }
}
