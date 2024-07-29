use crate::filter::filter;
use crate::stringify::get_string_reps;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Graph {
    pub vertices: Vec<i32>,
    pub hash_map_rep: HashMap<i32, String>,
}

impl Graph {
    pub fn new(vector_hashmap_graph: HashMap<i32, Vec<i32>>) -> Graph {
        let hash_map_rep = get_string_reps(vector_hashmap_graph);
        let vertices: Vec<i32> = hash_map_rep.keys().cloned().collect();
        Graph {
            vertices,
            hash_map_rep,
        }
    }
    pub fn get_dominating_vertex_set(&self) -> Option<Vec<i32>> {
        let filtered_graph = filter(self.hash_map_rep.clone());
        let filtered_vertices: Vec<i32> = filtered_graph.keys().copied().collect();
        for k in 1..filtered_vertices.len() {
            let combos_of_length_k = generate_combos(&filtered_vertices, &k);
            for combo in combos_of_length_k {
                let mut current_union_nhood: String = String::new();
                for vertex in &combo {
                    current_union_nhood =
                        current_union_nhood + &filtered_graph.get(&vertex).unwrap();
                    if is_spanning(&current_union_nhood, &filtered_graph) {
                        return Some(combo);
                    }
                }
            }
        }
        return None;
    }
}

// Helper functions:

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
