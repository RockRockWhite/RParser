use crate::{Derivation, DerivationBuilder, SymbolRef};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
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
    pub const END_SYMBOL: &str = "__$__";
    const EPSILON_SYMBOL: &str = "__EPSILON__";
    const DUMMY_START_SYMBOL: &str = "__DUMMY_START__";

    pub fn new() -> GrammerBuilder {
        let mut res = GrammerBuilder {
            start: None,
            symbols: RefCell::new(HashMap::new()),
        };

        // add the end symbol
        res.get_symbol(Self::EPSILON_SYMBOL);
        res.get_symbol(Self::END_SYMBOL);
        res.get_symbol(Self::DUMMY_START_SYMBOL);
        res
    }

    pub fn get_symbol(&mut self, symbol_name: &str) -> SymbolRef {
        if !self.symbols.borrow().contains_key(symbol_name) {
            self.symbols
                .borrow_mut()
                .insert(symbol_name.to_string(), SymbolRef::build(symbol_name));
        }
        SymbolRef::clone(self.symbols.borrow().get(symbol_name).unwrap())
    }

    pub fn get_epsilon_symbol(&self) -> SymbolRef {
        self.symbols
            .borrow()
            .get(Self::EPSILON_SYMBOL)
            .unwrap()
            .clone()
    }

    pub fn get_dummy_start_symbol(&self) -> SymbolRef {
        self.symbols
            .borrow()
            .get(Self::DUMMY_START_SYMBOL)
            .unwrap()
            .clone()
    }

    pub fn get_end_symbol(&self) -> SymbolRef {
        self.symbols.borrow().get(Self::END_SYMBOL).unwrap().clone()
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

            let dummy = self.get_dummy_start_symbol();
            dummy.borrow_mut().derivations = Vec::new();
            dummy
                .borrow_mut()
                .add_derivation(DerivationBuilder::new().add_symbol(&start).build());

            // calculate the first and follow set
            self.calculate_first_and_follow();

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
        // calculate the follow set
        self.calculate_follow();
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

    /// Calculate the follow set for each symbol
    /// This function should be called when the grammer is built
    fn calculate_follow(&mut self) {
        // add the end symbol to the follow set of the start symbol
        let dummy = self.get_symbol(Self::DUMMY_START_SYMBOL);
        dummy
            .borrow_mut()
            .follow_set
            .insert(self.get_symbol(Self::END_SYMBOL));

        loop {
            let mut changed = false;

            // tarverse all the derivations
            self.symbols
                .borrow()
                .values()
                .into_iter()
                .filter(|symbol| !symbol.borrow().is_terminal())
                .for_each(|symbol| {
                    symbol.borrow().derivations.iter().for_each(|derication| {
                        // for each 2 symbols in the right hand side of the derivation
                        derication
                            .windows(2)
                            .map(|windows| {
                                (SymbolRef::clone(&windows[0]), SymbolRef::clone(&windows[1]))
                            })
                            .for_each(|(left, right)| {
                                let mut follow = left.borrow().follow_set.clone();
                                // add the first set of the right symbol
                                // to the follow set of the left symbol
                                follow.extend(right.borrow().first_set.clone());

                                // if the follow set is changed, set the changed flag to true
                                if follow != left.borrow().follow_set {
                                    changed = true;
                                    left.borrow_mut().follow_set = follow;
                                }
                            });

                        // for the last symbol in the right hand side of the derivation
                        // add the follow set of the left symbol
                        for right_each in derication.iter().rev() {
                            // add the left hand side symbol's follow set
                            let mut follow = right_each.borrow().follow_set.clone();

                            follow.extend(symbol.borrow().follow_set.clone());

                            // if the first set is changed, set the changed flag to true
                            if follow != right_each.borrow().follow_set {
                                changed = true;
                                right_each.borrow_mut().follow_set = follow;
                            }

                            // if the symbol's first set contains epsilon
                            // contine the loop
                            if !right_each
                                .borrow()
                                .first_set
                                .contains(&self.get_epsilon_symbol())
                            {
                                break;
                            }
                        }
                    });
                });

            if !changed {
                break;
            }
        }

        let epsilon = self.get_epsilon_symbol();
        // delete all the epsilon in the follow set
        self.symbols
            .borrow_mut()
            .values_mut()
            .into_iter()
            .for_each(|symbol| {
                let mut follow = symbol.borrow().follow_set.clone();
                follow.remove(&epsilon);
                symbol.borrow_mut().follow_set = follow;
            });
    }
}
