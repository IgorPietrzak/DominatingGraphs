use std::collections::HashMap;

use filter::filter;
mod filter;
mod stringify;

fn main() {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    graph.insert(0, vec![0, 1]);
    graph.insert(1, vec![0, 1]);
    graph.insert(2, vec![3, 2]);
    graph.insert(3, vec![3, 2]);

    let stringified_graph = stringify::get_string_reps(&mut graph);
    let filtered_graph = filter(stringified_graph);
    println!("{:?}", filtered_graph);
}
