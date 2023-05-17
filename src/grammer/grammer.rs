use crate::{Derivation, DerivationBuilder, SymbolRef};
use std::{cell::RefCell, collections::HashMap};

pub struct Grammer {
    pub symbols: HashMap<String, SymbolRef>,
    pub start: SymbolRef,
}

pub struct GrammerBuilder {
    pub start: Option<SymbolRef>,
    pub symbols: RefCell<HashMap<String, SymbolRef>>,
}

impl GrammerBuilder {
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
    }

    // /// Calculate the first set for each symbol
    // /// This function should be called when the grammer is built
    // fn calculate_first(&mut self) {
    //     // for all the terminal symbols, the first set is itself
    //     self.symbols
    //         .iter_mut()
    //         .filter(|(_, symbol)| symbol.is_terminal())
    //         .for_each(|(symbol_name, symbol_data)| {
    //             symbol_data.first_set.insert(symbol_name.clone());
    //         });

    //     // tarverse all the derivations
    //     self.symbols
    //         .iter_mut()
    //         .filter(|(_, symbol)| !symbol.is_terminal())
    //         .for_each(|(symbol_name, symbol_data)| {
    //             symbol_data.derivations.iter().for_each(|derication| {
    //                 // for the first symbol in the right hand side of the derivation
    //                 // if it is a terminal symbol, add it to the first set
    //                 derication.iter().for_each(|right_hand_side_each|{
    //                     if self.symbols[right_hand_side_each].is_terminal() {
    //                         symbol_data.first_set.insert(right_hand_side_each.clone());
    //                     }

    //                 });

    //             })
    //         });
    // }
}
