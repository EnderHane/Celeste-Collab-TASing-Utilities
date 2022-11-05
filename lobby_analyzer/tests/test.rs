use lobby_analyzer::algorithm::{Graph, add_edge};

#[test]
fn test_graph() {
    let mut graph = Graph::new();
    add_edge(&mut graph, 0, 1, 2);
    add_edge(&mut graph, 1, 2, 4);
    add_edge(&mut graph, 1, 0, 3);
    println!("{:#?}", &graph);
}

#[test]
fn test_graph_update() {
    let mut graph = Graph::new();
    add_edge(&mut graph, 0, 1, 2);
    add_edge(&mut graph, 1, 2, 4);
    add_edge(&mut graph, 1, 0, 3);
    add_edge(&mut graph, 1, 0, 8);
    println!("{:#?}", &graph);
}

#[test]
fn test_graph_seq() {
    let mut graph = Graph::new();
    add_edge(&mut graph, 4, 1, 2);
    add_edge(&mut graph, 3, 2, 4);
    add_edge(&mut graph, 2, 0, 3);
    add_edge(&mut graph, 1, 0, 8);
    for (k, v) in &graph {
        println!("{}: ", k);
        for (k1, v1) in v {
            println!("{}: {}", k1, v1);
        }
    }
}
