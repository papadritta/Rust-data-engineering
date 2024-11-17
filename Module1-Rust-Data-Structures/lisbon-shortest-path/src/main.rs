use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    // Original landmarks
    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    // New landmarks
    let rossio_square = graph.add_node("Rossio Square");
    let santa_justa_lift = graph.add_node("Santa Justa Lift");
    let castle_of_sao_jorge = graph.add_node("Castle of São Jorge");
    let calouste_gulbenkian_museum = graph.add_node("Calouste Gulbenkian Museum");

    // Connections (distances in km)
    graph.extend_with_edges([
        // Original connections
        (belem_tower, monastery, 1),
        (belem_tower, lx_factory, 3),
        (belem_tower, commerce_square, 7),
        (monastery, lx_factory, 3),
        (monastery, commerce_square, 6),
        (lx_factory, commerce_square, 5),
        (commerce_square, lisbon_cathedral, 1),
        // New connections
        (commerce_square, rossio_square, 2),
        (rossio_square, santa_justa_lift, 1),
        (santa_justa_lift, castle_of_sao_jorge, 2),
        (lisbon_cathedral, castle_of_sao_jorge, 3),
        (commerce_square, calouste_gulbenkian_museum, 8),
        (rossio_square, calouste_gulbenkian_museum, 5),
    ]);

    let landmarks = vec![
        ("Belem Tower", belem_tower),
        ("Jerónimos Monastery", monastery),
        ("LX Factory", lx_factory),
        ("Commerce Square", commerce_square),
        ("Lisbon Cathedral", lisbon_cathedral),
        ("Rossio Square", rossio_square),
        ("Santa Justa Lift", santa_justa_lift),
        ("Castle of São Jorge", castle_of_sao_jorge),
        ("Calouste Gulbenkian Museum", calouste_gulbenkian_museum),
    ];
    let name_to_node: HashMap<&str, NodeIndex> = landmarks.iter().cloned().collect();

    println!(
        "Available landmarks: {:?}",
        landmarks.iter().map(|(name, _)| *name).collect::<Vec<_>>()
    );

    // Separate input variables for start and end landmarks
    let mut start_input = String::new();
    println!("Enter the starting landmark:");
    io::stdin().read_line(&mut start_input).unwrap();
    let start_name = start_input.trim();

    let mut end_input = String::new();
    println!("Enter the destination landmark:");
    io::stdin().read_line(&mut end_input).unwrap();
    let end_name = end_input.trim();

    if let (Some(&start), Some(&end)) = (name_to_node.get(start_name), name_to_node.get(end_name)) {
        let node_map = dijkstra(&graph, start, Some(end), |e| *e.weight());

        if let Some(distance) = node_map.get(&end) {
            println!(
                "The shortest distance from {} to {} is {} km",
                start_name, end_name, distance
            );
        } else {
            println!("No route found from {} to {}", start_name, end_name);
        }
    } else {
        println!("Invalid landmarks entered.");
    }
}
