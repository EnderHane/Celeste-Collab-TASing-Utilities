use serde_derive::Deserialize;

use crate::algorithm::{Graph, add_edge};

#[derive(Debug, Deserialize)]
pub struct LobbyData {
    pub lobby_path: String,
    pub source: String,
    pub destination: String,
    pub adj_table: Graph<String, String>
}

pub fn parse_graph(graph_in_str: &Graph<String, String>) -> Graph<i32, i32> {
    let mut graph: Graph<i32, i32> = Graph::new();
    for (v_s, adj_s) in graph_in_str {
        let v: i32 = v_s.parse().unwrap();
        for (v2_s, w_s) in adj_s {
            let v2: i32 = v2_s.parse().unwrap();
            let w: i32 = w_s.parse().unwrap();
            add_edge(&mut graph, v, v2, w);
        }
    }
    return graph;
}
