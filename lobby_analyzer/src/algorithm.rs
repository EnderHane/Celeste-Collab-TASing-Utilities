use std::collections::{BTreeMap, BinaryHeap};

pub type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;


pub fn add_edge(graph: &mut Graph<i32, i32>, v1: i32, v2: i32, e: i32) {
    graph.entry(v1).or_insert(BTreeMap::new()).entry(v2).and_modify(|p| {*p = e}).or_insert(e);
}


pub fn travel(graph: &Graph<i32, i32>, source: i32, destination: i32, ranking_count: usize)
    -> (i32 , usize, BinaryHeap<(i32, Vec<i32>)>) {
    if !graph.contains_key(&source) {
        panic!("City is not on the map!");
    }
    let mut path: Vec<i32> = Vec::new();
    let mut rank: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();
    let mut distance: i32 = 0;
    let mut counter: i32 = 0;
    search(graph, source, destination, &mut path, &mut distance, &mut counter, &mut rank, ranking_count);
    return (counter, ranking_count, rank);
}


fn search(
    graph: &Graph<i32, i32>, 
    vertex: i32, 
    destination: i32, 
    path: &mut Vec<i32>,
    distance: &mut i32,
    counter: &mut i32,
    rank: &mut BinaryHeap<(i32, Vec<i32>)>,
    ranking_count: usize
    ) {
    if let Some(&(worst, _)) = rank.peek() {
        if *distance >= worst {
            return;
        }
    }
    if let Some(adj) = graph.get(&vertex) {
        path.push(vertex);
        if path.len() == graph.len() - 1 && adj.contains_key(&destination) {
            path.push(destination);
            *distance += adj[&destination];
            rank.push((*distance, path.to_vec()));
            if rank.len() > ranking_count {
                rank.pop();
            }
            *counter += 1;
            *distance -= adj[&destination];
            path.pop();
            return;
        } else if path.len() < graph.len() - 1 {
            for (&v, &w) in adj {
                if !path.contains(&v) && v != destination {
                    *distance += w;
                    search(graph, v, destination, path, distance, counter, rank, ranking_count);
                    *distance -= w;
                }
            } 
        }
        path.pop();
    }
}