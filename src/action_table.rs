use serde::{Deserialize, Serialize};

use crate::{Dfa, GrammerBuilder};
use std::collections::HashMap;

type Symbol = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReduceDerivation {
    pub left: Symbol,
    pub right: Vec<Symbol>,
}

impl ReduceDerivation {
    pub fn build(left: Symbol, right: Vec<Symbol>) -> Self {
        ReduceDerivation { left, right }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Shift(usize),
    Reduce(ReduceDerivation),
    Accept,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub actions: HashMap<Symbol, Action>,
}

impl State {
    pub fn new() -> Self {
        State {
            actions: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ActionTable {
    pub states: Vec<State>,
}

impl ActionTable {
    pub fn from_dfa(dfa: &Dfa) -> Self {
        let mut res = ActionTable { states: Vec::new() };

        for each_vertex in dfa.vertices.iter() {
            let mut curr_state = State::new();

            // Shift Actions
            each_vertex
                .borrow()
                .neighbors
                .iter()
                .for_each(|(cond, next_vertex)| {
                    //  找到在vertices中的index
                    let next_index = dfa
                        .vertices
                        .iter()
                        .position(|x| *x == *next_vertex)
                        .unwrap();

                    // let index = dfa.vertices.index(|&x| *x == *next_vertex).unwrap().0;
                    curr_state
                        .actions
                        .insert(cond.borrow().name.clone(), Action::Shift(next_index));
                });

            // Reduce Actions
            each_vertex.borrow().items.iter().for_each(|item| {
                if item.is_reducible() {
                    let derivation = ReduceDerivation::build(
                        item.symbol.borrow().name.clone(),
                        item.derivation
                            .iter()
                            .map(|x| x.borrow().name.clone())
                            .collect(),
                    );

                    // 对所有的follow集合中的符号，都添加reduce动作
                    item.symbol.borrow().follow_set.iter().for_each(|symbol| {
                        curr_state.actions.insert(
                            symbol.borrow().name.clone(),
                            Action::Reduce(derivation.clone()),
                        );
                    });
                }
            });

            // add accept action
            if each_vertex == &dfa.start {
                curr_state
                    .actions
                    .insert(Symbol::from(GrammerBuilder::END_SYMBOL), Action::Accept);
            }

            res.states.push(curr_state);
        }

        res
    }
}

impl ToString for ActionTable {
    fn to_string(&self) -> String {
        let mut res = String::new();

        for (index, state) in self.states.iter().enumerate() {
            res.push_str(&format!("State {}:\n", index));

            for (symbol, action) in state.actions.iter() {
                res.push_str(&format!("{}: {:?}\n", symbol, action));
            }

            res.push_str("===================");
            res.push('\n');
        }

        res
    }
}
