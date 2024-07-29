mod filter;
mod graph;
mod graph_builder_cli;

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
