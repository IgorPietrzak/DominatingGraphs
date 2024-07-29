use graph::Graph;
use std::collections::HashMap;

mod filter;
mod graph;
mod graph_builder_cli;
mod stringify;

fn main() {
    let mut graph_hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
    graph_hash_map.insert(0, vec![0, 1, 4]);
    graph_hash_map.insert(1, vec![0, 1, 2]);
    graph_hash_map.insert(2, vec![2, 1, 3]);
    graph_hash_map.insert(3, vec![3, 2, 4]);
    graph_hash_map.insert(4, vec![0, 4, 3]);
    graph_hash_map.insert(5, vec![5, 4, 3]);
    graph_hash_map.insert(6, vec![6, 5, 7]);
    graph_hash_map.insert(7, vec![7, 6, 8]);
    graph_hash_map.insert(8, vec![6, 8]);

    // build a graph given user inputs
    graph_builder_cli::program_loop();
}
