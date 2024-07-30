# Given a graph find the minimal dominating vertex set.

## Table of Contents
- [Introduction](#introduction)
- [Demo](#demo)
- [Installation](#installation)
- [Usage](#usage)


## Introduction
Code to compute the minimal dominating vertex set for an undirected graph. Sometimes also called domination number, see: https://en.wikipedia.org/wiki/Dominating_set


## Demo
A demo showing how to use the cli tool to compute a minimal dominating vertex set for the graph drawn on the right hand side


https://github.com/user-attachments/assets/03760914-b1b6-4e4a-8bb3-bdff4193e281

## Installation
Follow official instructions to install the Rust toolchain:

https://www.rust-lang.org/tools/install

Once Rust is installed, do the following:

1.) Clone the repository

```bash
git clone https://github.com/IgorPietrzak/DominatingGraphs.git
```

2.) cd into the directory


```bash
cd DominatingGraphs
```


3.) Run the program

```bash
cargo run
```

## Usage


Use of the library shown in demo

```rust
extern crate dominating_graphs;
use dominating_graphs::graph_builder_cli;

fn main() {
    let graph = graph_builder_cli::program_loop();
    let min_vertex_set = graph.get_dominating_vertex_set().unwrap();
    println!("\n ------------------------------------------------------------------------------");
    println!("Your graph is: {:?}", graph);
    println!(" ------------------------------------------------------------------------------");
    println!("\n ------------------------------------------------------------------------------");
    println!("A minimal dominating vertex set is: {:?}", min_vertex_set);
    println!(" ------------------------------------------------------------------------------ \n");
}
```
