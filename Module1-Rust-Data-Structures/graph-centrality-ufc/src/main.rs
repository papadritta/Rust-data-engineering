use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Add an edge between two nodes
fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

// Function to calculate and print betweenness centrality
fn calculate_betweenness_centrality(
    graph: &UnGraph<&Fighter, f32>,
    fighter_nodes: &[NodeIndex],
) -> HashMap<NodeIndex, f32> {
    let mut centrality = HashMap::new();

    // Initialize centrality map
    for &node in fighter_nodes {
        centrality.insert(node, 0.0);
    }

    // Calculate betweenness centrality by iterating over each node as a source
    for i in 0..fighter_nodes.len() {
        for j in i + 1..fighter_nodes.len() {
            let start = fighter_nodes[i];
            let end = fighter_nodes[j];

            // Find shortest path from start to end
            let paths = dijkstra(graph, start, Some(end), |_| 1.0);

            if let Some(&shortest_distance) = paths.get(&end) {
                // Count paths that pass through each node on the way from `start` to `end`
                for (&node, &distance) in &paths {
                    if node != start && node != end && distance < shortest_distance {
                        *centrality.get_mut(&node).unwrap() += 1.0;
                    }
                }
            }
        }
    }

    centrality
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    // Initialize fighters
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    // Create nodes for each fighter and add them to the graph
    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    // Adding edges (fights)
    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    // Calculate and display closeness and betweenness centrality
    println!("Centralities:");
    let betweenness = calculate_betweenness_centrality(&graph, &fighter_nodes);

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        let betweenness_value = betweenness[&node];

        println!("Centralities for {}:", name);
        println!("  Closeness centrality: {:.2}", closeness);
        println!("  Betweenness centrality: {:.2}", betweenness_value);

        // Explanation based on closeness
        match name.as_str() {
            "Conor McGregor" => println!(
                "  {} has the lowest centrality because he has fought with all other fighters in the network. A lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "  {} has a moderate centrality, implying they had fewer fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "  {} has the highest centrality as they have fought with the least number of fighters.",
                name
            ),
            _ => {}
        }
        println!("-----------------");
    }
}
