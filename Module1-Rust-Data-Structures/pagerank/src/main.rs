use std::io;
use textwrap::fill;

struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        let n = graph.len();
        let mut ranks = vec![1.0 / (n as f64); n];

        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; n];

            for (node, edges) in graph.iter().enumerate() {
                let contribution = ranks[node] / (edges.len() as f64);
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            ranks = new_ranks;
        }

        ranks
    }

    // Normalize the PageRank values so they sum to 1
    fn normalize(&self, ranks: &mut Vec<f64>) {
        let sum: f64 = ranks.iter().sum();
        for rank in ranks.iter_mut() {
            *rank /= sum;
        }
    }
}

// Function to adapt PageRank for social networks
fn create_social_network() -> (Vec<Vec<usize>>, Vec<String>) {
    let graph = vec![
        vec![1, 2], // Alice links to Bob, Charlie
        vec![2],    // Bob links to Charlie
        vec![0],    // Charlie links to Alice
        vec![0, 1], // Dave links to Alice, Bob
    ];
    let names = vec!["Alice", "Bob", "Charlie", "Dave"];
    (graph, names.iter().map(|&s| s.to_string()).collect())
}

// Function to adapt PageRank for citation networks
fn create_citation_network() -> (Vec<Vec<usize>>, Vec<String>) {
    let graph = vec![
        vec![1, 2], // Paper A cites Paper B, Paper C
        vec![2],    // Paper B cites Paper C
        vec![],     // Paper C cites no one
    ];
    let names = vec!["Paper A", "Paper B", "Paper C"];
    (graph, names.iter().map(|&s| s.to_string()).collect())
}

fn main() {
    let mut graph = vec![];
    let mut names = vec![];

    let mut input = String::new();
    println!("Enter the number of nodes:");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for i in 0..n {
        println!("Enter the name for node {}:", i);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        names.push(input.trim().to_string());

        println!(
            "Enter the indices of nodes linked from {} (comma-separated):",
            input.trim()
        );
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edges: Vec<usize> = input
            .trim()
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .collect();
        graph.push(edges);
    }

    let pagerank = PageRank::new(0.85, 100);
    let mut ranks = pagerank.rank(&graph);

    // Normalize the PageRank values
    pagerank.normalize(&mut ranks);

    println!("\nPageRank Results:");
    for (i, rank) in ranks.iter().enumerate() {
        println!("The normalized PageRank of {} is {}", names[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";
    println!("{}", fill(explanation, 78));

    // Demonstrate PageRank with a social network
    let (social_graph, social_names) = create_social_network();
    let mut social_ranks = pagerank.rank(&social_graph);
    pagerank.normalize(&mut social_ranks);
    println!("\nPageRank Results for Social Network:");
    for (i, rank) in social_ranks.iter().enumerate() {
        println!("The normalized PageRank of {} is {}", social_names[i], rank);
    }

    // Demonstrate PageRank with a citation network
    let (citation_graph, citation_names) = create_citation_network();
    let mut citation_ranks = pagerank.rank(&citation_graph);
    pagerank.normalize(&mut citation_ranks);
    println!("\nPageRank Results for Citation Network:");
    for (i, rank) in citation_ranks.iter().enumerate() {
        println!(
            "The normalized PageRank of {} is {}",
            citation_names[i], rank
        );
    }
}
