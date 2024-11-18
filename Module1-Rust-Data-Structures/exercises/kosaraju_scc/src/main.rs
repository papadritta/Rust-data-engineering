use petgraph::algo::kosaraju_scc;
use petgraph::graph::DiGraph;

fn main() {
    let mut graph = DiGraph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());
    graph.add_edge(c, d, ());

    let scc = kosaraju_scc(&graph);
    println!("Strongly connected components: {:?}", scc);
}
