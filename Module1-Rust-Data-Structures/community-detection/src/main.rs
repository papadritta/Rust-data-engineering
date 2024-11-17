use community_detection::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() {
    // Create a new directed Graph
    let mut graph = DiGraph::<&str, &str>::new();

    // Create a HashMap to store node indices by user name
    let mut nodes = HashMap::new();

    // Populate the graph with edges from the dataset
    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        // Add nodes and edges to the graph
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        graph.add_edge(user_node, mention_node, "retweets");
    }

    // Detect strongly connected components using Kosaraju's algorithm
    let scc = kosaraju_scc(&graph);

    println!("Detected communities:");
    for (i, component) in scc.iter().enumerate() {
        println!("Community {} ({} nodes):", i + 1, component.len());
        let usernames: Vec<&str> = component
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!("{:?}", usernames);
    }

    // Find and print the largest community
    let largest_community = scc.iter().max_by_key(|component| component.len()).unwrap();
    println!(
        "\nThe largest community has {} nodes:",
        largest_community.len()
    );
    let usernames: Vec<&str> = largest_community
        .iter()
        .map(|&node_index| graph[node_index])
        .collect();
    println!("{:?}", usernames);
}
