use petgraph::algo::connected_components;
use petgraph::Graph;

fn is_fully_connected(graph: &Graph<(), (), petgraph::Undirected>) -> bool {
    connected_components(graph) == 1
}

fn main() {
    let mut graph = Graph::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    if is_fully_connected(&graph) {
        println!("The graph is fully connected.");
    } else {
        println!("The graph is not fully connected.");
    }
}
