use std::collections::HashMap;

pub fn get_string_reps(graph: HashMap<i32, Vec<i32>>) -> HashMap<i32, String> {
    let mut string_graph: HashMap<i32, String> = HashMap::new();
    for (key, value) in graph.iter() {
        string_graph.insert(key.clone(), get_string_rep(value.clone()));
    }
    string_graph
}

fn get_string_rep(neighbours: Vec<i32>) -> String {
    let mut str_rep = String::new();
    for num in neighbours {
        str_rep.push_str(&num.to_string())
    }

    str_rep
}
