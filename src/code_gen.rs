use crate::{config::Rule, ActionTable, GrammerBuilder};

pub fn gen_code(
    declarations: &str,
    variables: &str,
    action_table: &ActionTable,
    rules: &Vec<Rule>,
) -> String {
    let action_table_json = format!("r#\"{}\"#", serde_json::to_string(action_table).unwrap());
    // 生成handler_funcs_str
    let mut handler_funcs_str = String::new();
    rules.iter().for_each(|rule| {
        let left_str = format!(r#""{}".into()"#, rule.left);
        let right_str = rule
            .right
            .iter()
            .filter(|symbol| symbol != &&GrammerBuilder::EPSILON_SYMBOL.to_string())
            .map(|symbol| format!(r#""{}".into()"#, symbol))
            .collect::<Vec<_>>()
            .join(", ");

        handler_funcs_str.push_str(&format!(
            r#"handlers.insert(
            ReduceDerivation::build({}, vec![{}]),
            Box::new({}),
        );"#,
            left_str, right_str, rule.handler
        ));
    });

    format!(
        r#"
use serde::{{Deserialize, Serialize}};
use std::{{collections::HashMap, error::Error, hash::Hash}};

// declarations
// ======================
{declarations}
// ======================

/// Token
/// must implement Clone and Token trait
/// Token trait is used to convert a token to a tree node
pub trait Token: Clone {{
    fn to_tree_node(&self) -> ParsingTreeNode;
}}

pub struct ParsingTreeNode {{
    pub symbol_type: String,
    pub data: String,
    pub children: Vec<ParsingTreeNode>,
}}

impl ParsingTreeNode {{
    pub fn build(symbol_type: String, data: String, children: Vec<ParsingTreeNode>) -> Self {{
        ParsingTreeNode {{
            symbol_type,
            data,
            children,
        }}
    }}
}}

/// NodePair
/// a pair of a node and a state.
/// (TreeNode, state)
pub struct NodePair(ParsingTreeNode, usize);
impl NodePair {{
    pub fn new(node: ParsingTreeNode, state: usize) -> Self {{
        NodePair(node, state)
    }}
}}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReduceDerivation {{
    pub left: String,
    pub right: Vec<String>,
}}

impl PartialEq for ReduceDerivation {{
    fn eq(&self, other: &Self) -> bool {{
        self.left == other.left && self.right == other.right
    }}
}}

impl Eq for ReduceDerivation {{}}

impl Hash for ReduceDerivation {{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {{
        self.left.hash(state);
        self.right.hash(state);
    }}
}}

impl ReduceDerivation {{
    pub fn build(left: String, right: Vec<String>) -> Self {{
        ReduceDerivation {{ left, right }}
    }}
}}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {{
    Shift(usize),
    Reduce(ReduceDerivation),
    Accept,
    Error,
}}

#[derive(Serialize, Deserialize)]
pub struct State {{
    pub actions: HashMap<String, Action>,
}}

impl State {{
    pub fn new() -> Self {{
        State {{
            actions: HashMap::new(),
        }}
    }}
}}

#[derive(Serialize, Deserialize, Default)]
pub struct ActionTable {{
    pub states: Vec<State>,
}}

impl ActionTable {{
    pub fn get_action(&self, state: usize, symbol: &str) -> Option<&Action> {{
        self.states[state].actions.get(symbol)
    }}
}}

#[derive(Default)]
pub struct RParser {{
    action_table: ActionTable,
    handlers: HashMap<ReduceDerivation, Box<dyn Fn(Vec<String>) -> String>>,
    // variables
    // ======================
    {variables}
    // ======================
}}

impl RParser {{
    pub const END_SYMBOL: &'static str = "__$__";
    pub const EPSILON_SYMBOL: &'static str = "__EPSILON__";
    pub const DUMMY_START_SYMBOL: &'static str = "__DUMMY_START__";

    pub fn new() -> Self {{
        // action table generated by rparser
        // ======================
        let action_table: ActionTable =  serde_json::from_str({action_table_json}).unwrap();
        // ======================

        let mut handlers: HashMap<ReduceDerivation, Box<dyn Fn(Vec<String>) -> String>> =
            HashMap::new();

        // handlers generated by rparser
        // ======================
        {handler_funcs_str}
        // ======================

        handlers.insert(
            ReduceDerivation::build(Self::DUMMY_START_SYMBOL.into(), vec!["S".into()]),
            Box::new(|vals| vals[0].clone()),
        );

        let mut res: RParser = RParser::default();
        res.action_table = action_table;
        res.handlers = handlers;
        res
    }}

    // do shift-reduce parsing
    pub fn parse<T>(&self, tokens: Vec<T>) -> Result<ParsingTreeNode, Box<dyn Error>>
    where
        T: Token,
    {{
        let mut shift_index = 0;
        let mut stack: Vec<NodePair> = Vec::new();

        stack.push(NodePair::new(
            ParsingTreeNode::build(Self::DUMMY_START_SYMBOL.into(), String::new(), Vec::new()),
            0,
        ));

        loop {{
            let token_node = &tokens[shift_index].to_tree_node();

            let action = self
                .action_table
                .get_action(stack.last().unwrap().1, &token_node.symbol_type);

            match action {{
                Some(Action::Shift(next_state)) => {{
                    // shift
                    stack.push(NodePair::new(
                        tokens[shift_index].to_tree_node(),
                        *next_state,
                    ));
                    shift_index += 1;
                }}
                Some(Action::Reduce(derivation)) => {{
                    // pop right hand
                    let mut children: Vec<ParsingTreeNode> = Vec::new();
                    let mut datas = Vec::new();
                    for _ in 0..derivation.right.len() {{
                        if let Some(top) = stack.pop() {{
                            datas.push(top.0.data.clone());
                            children.push(top.0);
                        }} else {{
                            Err("parsing error: stack is empty.")?;
                        }}
                    }}

                    let children: Vec<_> = children.into_iter().rev().collect();
                    let datas: Vec<_> = datas.into_iter().rev().collect();
                    let handler = self.handlers.get(&derivation).unwrap();

                    // if the left hand side is dummy start symbol
                    // do nothing
                    if derivation.left == Self::DUMMY_START_SYMBOL {{
                        stack.push(NodePair(
                            ParsingTreeNode::build(
                                derivation.left.to_string(),
                                handler(datas),
                                children,
                            ),
                            0,
                        ));
                        continue;
                    }}

                    // goto[top_state(stack), X]
                    if let Action::Shift(next_state) = self
                        .action_table
                        .get_action(stack.last().unwrap().1, &derivation.left)
                        .unwrap()
                    {{
                        stack.push(NodePair(
                            ParsingTreeNode::build(
                                derivation.left.to_string(),
                                handler(datas),
                                children,
                            ),
                            *next_state,
                        ));
                    }} else {{
                        Err("parsing error: invalid Shift action.")?;
                    }}
                }}
                Some(Action::Accept) => {{
                    let res = stack.pop().unwrap().0;
                    return Ok(res);
                }}
                _ => {{
                    Err("parsing error: unknown.")?;
                }}
            }}
        }}
    }}
}}
    "#
    )
    .to_string()
}
