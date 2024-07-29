use std::collections::HashMap;
use std::io;

use crate::graph::Graph;

pub fn program_loop() -> Graph {
    let mut number_of_vertices = String::new();
    println!("Tell me how many vertices your graph is made up of: \n ");
    io::stdin()
        .read_line(&mut number_of_vertices)
        .expect("Failed to read line");
    let number_of_vertices = number_of_vertices.trim();
    match number_of_vertices.parse::<i32>() {
        Ok(number) => builder_loop(number),
        Err(_) => {
            println!("\n I'm afraid that's not a number! Try again. \n ");
            program_loop()
        }
    }
}

fn builder_loop(no_of_vertices: i32) -> Graph {
    let mut graph_hash_map: HashMap<i32, String> = HashMap::new();
    for vertex in 0..no_of_vertices {
        let mut input = String::new();
        println!("\n List the neighbours of vertex {vertex}: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // check if string is numeric
        let mut input = input.trim().to_string();
        if input.parse::<i32>().is_ok() {
            input = vertex.to_string() + &input;
            graph_hash_map.insert(vertex, input.to_string());
        } else {
            println!("Invalid vertex set entered");
        }
    }

    Graph {
        vertices: (0..no_of_vertices).collect(),
        hash_map_rep: graph_hash_map,
    }
}
