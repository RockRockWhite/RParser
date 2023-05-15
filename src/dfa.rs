use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

use crate::{grammer, Grammer};

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct Item {
    pub symbol_name: String,
    pub production_index: usize,
    pub dot: usize,
}

impl Item {
    pub fn next_symbol(&self, grammer: &Grammer) -> Option<String> {
        let symbol = grammer.get_symbol(&self.symbol_name).unwrap();
        let derivation = &symbol.derivations[self.production_index];
        if self.dot < derivation.len() {
            Some(derivation[self.dot].clone())
        } else {
            None
        }
    }
}
pub struct StateVertex {
    pub items: HashSet<Item>,
    pub neighbors: HashMap<String, DfaVertexRef>,
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

impl PartialEq for DfaVertexRef {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().items == other.borrow().items
    }
}

impl Deref for DfaVertexRef {
    type Target = Rc<RefCell<StateVertex>>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
            symbol_name: grammer.get_dummy_start_name().into(),
            production_index: 0,
            dot: 0,
        };

        let start = DfaVertexRef::new();
        start.borrow_mut().items = Self::epsilon_closure(&start_item, grammer);

        let mut visited = Vec::new();
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

        let mut neighbors: HashMap<String, DfaVertexRef> = HashMap::new();

        // get all the next symbols of the start state
        start.borrow().items.iter().for_each(|item| {
            if let Some(next_symbol) = item.next_symbol(&grammer) {
                // if the state in this cond not exists
                // add it to the visited
                if !neighbors.contains_key(&next_symbol) {
                    let next_state = DfaVertexRef::new();
                    neighbors.insert(next_symbol.clone(), DfaVertexRef::clone(&next_state));
                }

                let next_state = neighbors.get(&next_symbol).unwrap();

                // get the next state
                let next_item = Item {
                    symbol_name: item.symbol_name.clone(),
                    production_index: item.production_index,
                    dot: item.dot + 1,
                };
                // get the epsilon closure of the next state
                next_state
                    .borrow_mut()
                    .items
                    .extend(Self::epsilon_closure(&next_item, grammer));
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

    pub fn epsilon_closure(item: &Item, grammer: &Grammer) -> HashSet<Item> {
        let mut res = HashSet::new();

        // 等于 item 能推导的每个元素的闭包集合+item本身
        // 1. item 本身
        res.insert(item.clone());
        // 2. item 能推导的每个元素的闭包集合
        // 即 item 的推倒式 dot 后面的元素
        if let Some(next) = item.next_symbol(grammer) {
            // 添加所有能next能够推导的item
            let next_symbol = grammer.get_symbol(&next).unwrap();
            for index in 0..next_symbol.derivations.len() {
                let curr_item = Item {
                    symbol_name: next.clone(),
                    production_index: index,
                    dot: 0,
                };

                res.extend(Self::epsilon_closure(&curr_item, grammer));
            }
        };
        res
    }
}