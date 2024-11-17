use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::{BinaryHeap, HashMap};
use std::env;

#[derive(Eq, PartialEq, Clone)] // Added Clone here
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(a), Fruit::Other(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Function to generate fruit salad and count occurrences
fn generate_fruit_salad_and_count() -> (BinaryHeap<Fruit>, HashMap<String, usize>) {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();
    let mut fruit_counts = HashMap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        *fruit_counts.entry(fruit.to_string()).or_insert(0) += 1;
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    (fruit_salad, fruit_counts)
}

fn main() {
    // Generate the fruit salad and count occurrences
    let (mut fruit_salad, fruit_counts) = generate_fruit_salad_and_count();

    println!("Original Fruit Salad:");
    for fruit in fruit_salad.clone().into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }

    // Display fruit counts
    println!("\nFruit Counts:");
    for (fruit, count) in &fruit_counts {
        println!("{}: {}", fruit, count);
    }

    // Command-line argument to remove a fruit
    if let Some(fruit_to_remove) = env::args().nth(1) {
        fruit_salad = fruit_salad
            .into_sorted_vec()
            .into_iter()
            .filter(|fruit| match fruit {
                Fruit::Fig => fruit_to_remove.to_lowercase() != "fig",
                Fruit::Other(name) => name.to_lowercase() != fruit_to_remove.to_lowercase(),
            })
            .collect::<BinaryHeap<Fruit>>();

        println!(
            "\nUpdated Fruit Salad After Removing '{}':",
            fruit_to_remove
        );
        for fruit in fruit_salad.clone().into_sorted_vec() {
            match fruit {
                Fruit::Fig => println!("Fig"),
                Fruit::Other(fruit_name) => println!("{}", fruit_name),
            }
        }
    }

    // Print unique fruits in reverse order
    println!("\nUnique Fruits in Reverse Order:");
    for fruit in fruit_salad.clone().into_sorted_vec().iter().rev() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(name) => println!("{}", name),
        }
    }
}
