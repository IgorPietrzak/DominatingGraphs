use std::collections::{HashMap, HashSet};

fn is_subset(vec_one: &str, vec_two: &str) -> bool {
    let set_two: HashSet<_> = vec_two.chars().collect();

    vec_one.chars().all(|c| set_two.contains(&c))
}
pub fn filter(mut graph: HashMap<i32, String>) -> HashMap<i32, String> {
    let mut keys_to_remove = Vec::new();
    for (&key1, value1) in &graph {
        if keys_to_remove.len() == graph.keys().len() - 1 {
            break;
        }
        for (&key2, value2) in &graph {
            if key1 != key2 && !keys_to_remove.contains(&key1) && !keys_to_remove.contains(&key2) {
                if is_subset(value1, value2) {
                    keys_to_remove.push(key1);
                    break;
                }
            }
        }
    }
    for key in keys_to_remove {
        graph.remove(&key);
    }
    graph
}
