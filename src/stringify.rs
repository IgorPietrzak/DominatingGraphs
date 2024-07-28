use std::collections::HashMap;

pub fn get_string_reps(graph: &mut HashMap<i32, Vec<i32>>) -> HashMap<i32, String> {
    let mut string_graph: HashMap<i32, String> = HashMap::new();
    for (key, value) in graph.iter() {
        string_graph.insert(key.clone(), get_string_rep(&mut value.clone()));
    }
    string_graph
}

pub fn get_string_rep(neighbours: &mut Vec<i32>) -> String {
    let mut str_rep = String::new();
    neighbours.sort();
    for num in neighbours {
        str_rep.push_str(&num.to_string())
    }

    str_rep
}
