use crate::{Dfa, DfaVertexRef};

/// Converts an DFA to a mermaid graph
/// https://mermaid-js.github.io/mermaid/#/graph?id=graph
/// ```mermaid
/// graph LR
/// A((A))
/// B((B))
/// A--a-->B
/// ```
/// **节点id仅用于标注，不保证与lookup table的id完全一样**
pub fn parse_dfa(dfa: &Dfa) -> String {
    // 遍历图
    let mut visited = Vec::new();
    let edge = tarverse_dfa_vertex(DfaVertexRef::clone(&dfa.start), &mut visited);

    // 添加节点
    let mut vertex = String::new();
    visited.iter().enumerate().for_each(|(id, each)| {
        let mut items = String::new();
        each.borrow().items.iter().for_each(|item| {
            let mut derivation_format = item
                .derivation
                .iter()
                .enumerate()
                .map(|(index, symbol)| {
                    if index == item.dot {
                        format!("{}{}", "•", symbol.borrow().name.clone())
                    } else {
                        symbol.borrow().name.clone()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");

            // add the dot in the end
            if item.dot == item.derivation.len() {
                derivation_format.push_str("•");
            }
            let item_format =
                &format!("{} -> {}\\n", &item.symbol.borrow().name, derivation_format);
            items.push_str(&item_format);
        });

        vertex.push_str(&format!("{}[\"{}\"]\n", id, items));
    });

    format!("graph LR\n{}\n{}", vertex, edge)
}

/// tarverse_dfa_vertex
/// 遍历dfa节点，返回其边
/// return edges
fn tarverse_dfa_vertex(vertex: DfaVertexRef, visited: &mut Vec<DfaVertexRef>) -> String {
    let mut edges = String::new();

    // 如果已经访问过，直接返回
    if visited.contains(&vertex) {
        return edges;
    }

    // 标记为已访问, 将vec的下标作为id
    let id = visited.len();
    visited.push(DfaVertexRef::clone(&vertex));

    let find_vertex_id = |vertex: DfaVertexRef, visited: &Vec<DfaVertexRef>| -> usize {
        visited
            .iter()
            .enumerate()
            .find(|(_, each)| (**each) == vertex)
            .unwrap()
            .0
    };

    // 遍历节点

    vertex.borrow().neighbors.iter().for_each(|(cond, vertex)| {
        let neighbor_edges = tarverse_dfa_vertex(DfaVertexRef::clone(&vertex), visited);
        // 将结果添加到边中
        edges.push_str(&neighbor_edges);

        let neighbor_id = find_vertex_id(DfaVertexRef::clone(&vertex), visited);

        // 添加当前节点到该节点的边
        edges.push_str(&format!("{}--\"{}\"-->{}\n", id, cond, neighbor_id));
    });

    edges
}
