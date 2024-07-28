use filter::filter;
use itertools::Combinations;
use itertools::Itertools;
use std::collections::HashMap;
use std::slice::Iter;
mod filter;
mod stringify;

fn main() {
    // Create a graph here

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    graph.insert(0, vec![0, 1, 4]);
    graph.insert(1, vec![0, 1, 2]);
    graph.insert(2, vec![2, 1, 3]);
    graph.insert(3, vec![3, 2, 4]);
    graph.insert(4, vec![0, 4, 3]);
    graph.insert(5, vec![5, 4, 3]);
    graph.insert(6, vec![6, 5, 7]);
    graph.insert(7, vec![7, 6]);

    // get string representation of neighbourhoods, see stringify.rs

    let stringified_graph = stringify::get_string_reps(&mut graph);

    // filter out redundant vertices, see filter.rs

    let filtered_graph = filter(stringified_graph);
    let out = get_spanning_graph(&vec![0, 1, 2, 3, 4, 5, 6, 7], &filtered_graph);
    println!("{:?}", out);
}

fn get_spanning_graph(nums: &Vec<i32>, stringified_graph: &HashMap<i32, String>) -> Vec<i32> {
    for k in 1..nums.len() {
        let combos_of_length_k = generate_combos(nums, &k);
        for combo in combos_of_length_k {
            // combo of keys
            let mut current_union_nhood: String = String::new();
            for vertex in &combo {
                current_union_nhood =
                    current_union_nhood + &stringified_graph.get(&vertex).unwrap();
                if is_spanning(&current_union_nhood, stringified_graph) {
                    // return nhood and current combo
                    return combo;
                }
            }
        }
    }
    return vec![0, 0, 0, 0, 0, 0, 0];
}

fn generate_combos(nums: &Vec<i32>, k: &usize) -> Vec<Vec<i32>> {
    nums.iter()
        .combinations(*k)
        .map(|combo| combo.into_iter().copied().collect())
        .collect()
}

fn is_spanning(vertex_set: &String, stringified_graph: &HashMap<i32, String>) -> bool {
    for key in stringified_graph.keys() {
        if !&vertex_set.contains(&key.to_string()) {
            return false;
        }
    }

    true
}
