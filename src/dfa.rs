use crate::{grammer::Symbol, Derivation, DerivationBuilder, Grammer, GrammerBuilder, SymbolRef};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct Item {
    pub symbol: SymbolRef,
    pub derivation: Derivation,
    pub dot: usize,
}

impl Item {
    pub fn next_symbol(&self) -> Option<SymbolRef> {
        if self.dot < self.derivation.len() {
            Some(SymbolRef::clone(&self.derivation[self.dot]))
        } else {
            None
        }
    }

    pub fn is_reducible(&self) -> bool {
        self.dot == self.derivation.len()
    }
}
pub struct StateVertex {
    pub items: HashSet<Item>,
    pub neighbors: HashMap<SymbolRef, DfaVertexRef>,
}

pub struct DfaVertexRef(Rc<RefCell<StateVertex>>);

impl DfaVertexRef {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(StateVertex {
            items: HashSet::new(),
            neighbors: HashMap::new(),
        })))
    }
}

impl Deref for DfaVertexRef {
    type Target = Rc<RefCell<StateVertex>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for DfaVertexRef {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().items == other.borrow().items
    }
}

impl Clone for DfaVertexRef {
    fn clone(&self) -> Self {
        DfaVertexRef(Rc::clone(&self.0))
    }
}

pub struct Dfa {
    pub start: DfaVertexRef,
    pub vertices: Vec<DfaVertexRef>,
}

impl Dfa {
    pub fn from(grammer: &Grammer) -> Self {
        // start state items contains the dummy derivation
        // S' -> .S
        let start_item = Item {
            symbol: SymbolRef::clone(&grammer.start),
            derivation: grammer.start.borrow().derivations[0].clone(),
            dot: 0,
        };

        let start = DfaVertexRef::new();
        start.borrow_mut().items = Self::epsilon_closure(&start_item, grammer, &mut Vec::new());

        let mut visited: Vec<DfaVertexRef> = Vec::new();
        Self::taverse_build(DfaVertexRef::clone(&start), &mut visited, grammer);

        Dfa {
            start,
            vertices: visited,
        }
    }

    /// traverse the dfa and build the dfa
    pub fn taverse_build(start: DfaVertexRef, visited: &mut Vec<DfaVertexRef>, grammer: &Grammer) {
        // if the start state has been visited, return
        if visited.contains(&start) {
            return;
        }
        // mark the start state as visited
        visited.push(DfaVertexRef::clone(&start));

        let mut neighbors: HashMap<SymbolRef, DfaVertexRef> = HashMap::new();

        // get all the next symbols of the start state
        start.borrow().items.iter().for_each(|item| {
            if let Some(next_symbol) = item.next_symbol() {
                // if the state in this cond not exists
                // add it to the visited
                if !neighbors.contains_key(&next_symbol) {
                    let next_state = DfaVertexRef::new();
                    neighbors.insert(
                        SymbolRef::clone(&next_symbol),
                        DfaVertexRef::clone(&next_state),
                    );
                }
                let next_state = neighbors.get(&next_symbol).unwrap();

                // add new item to the next state
                let next_item = Item {
                    symbol: SymbolRef::clone(&item.symbol),
                    derivation: item.derivation.clone(),
                    dot: item.dot + 1,
                };
                // get the epsilon closure of the next state
                next_state.borrow_mut().items.extend(Self::epsilon_closure(
                    &next_item,
                    grammer,
                    &mut Vec::new(),
                ));
            }
        });

        // if some state in the neighbors is in the visited
        // use visited state instead of the new state
        neighbors.values_mut().for_each(|next_state| {
            if let Some(visited_state) = visited.iter().find(|state| **state == *next_state) {
                *next_state = DfaVertexRef::clone(visited_state);
            }
        });

        // set the start state's neighbors
        start.borrow_mut().neighbors = neighbors;

        // traverse the next states
        start.borrow().neighbors.values().for_each(|next_state| {
            Self::taverse_build(DfaVertexRef::clone(next_state), visited, grammer);
        });
    }

    pub fn epsilon_closure(
        item: &Item,
        grammer: &Grammer,
        visited: &mut Vec<Item>,
    ) -> HashSet<Item> {
        let mut res = HashSet::new();

        // if the item has been visited, return
        if visited.contains(&item) {
            return res;
        }
        visited.push(item.clone());

        // if the right hand side is epsilon
        // set the item as A -> .
        let epsilon_symbol = SymbolRef::build(GrammerBuilder::EPSILON_SYMBOL.into());
        let item = if item.derivation[0] == epsilon_symbol {
            Item {
                symbol: SymbolRef::clone(&item.symbol),
                derivation: DerivationBuilder::new().build(),
                dot: 0,
            }
        } else {
            item.clone()
        };
        res.insert(item.clone());

        // for the item behind the dot
        // add it's epsilon closure
        if !item.is_reducible() {
            let next_symbol = item.next_symbol().unwrap();

            next_symbol
                .borrow()
                .derivations
                .iter()
                .for_each(|derivation| {
                    let curr_item = Item {
                        symbol: SymbolRef::clone(&next_symbol),
                        derivation: derivation.clone(),
                        dot: 0,
                    };

                    res.extend(Self::epsilon_closure(&curr_item, grammer, visited));
                });
        }

        res
    }
}
