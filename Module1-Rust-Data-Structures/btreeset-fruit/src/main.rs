use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeSet, HashMap};
use std::env;

// Function to count occurrences of generated fruits
fn count_fruit_occurrences(fruits: &[String], amounts: &[usize]) -> HashMap<String, usize> {
    let mut rng = thread_rng();
    let mut fruit_counts: HashMap<String, usize> = HashMap::new();

    for amount in amounts {
        let mut shuffled_fruits = fruits.to_vec();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits.iter().take(*amount) {
            *fruit_counts.entry(fruit.clone()).or_insert(0) += 1;
        }
    }

    fruit_counts
}

fn main() {
    let fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
        "grape".to_string(),
        "honeydew".to_string(),
    ];
    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();
    let mut fruit_set = BTreeSet::new();

    // Shuffle and fill the set
    let mut shuffled_fruits = fruits.clone();
    shuffled_fruits.shuffle(&mut rng);
    for fruit in shuffled_fruits {
        fruit_set.insert(fruit);
        if fruit_set.len() >= amounts[amounts.len() - 1] {
            break;
        }
    }

    println!("Initial fruits: {:?}", fruit_set);

    // Remove fruit if provided as a command-line argument
    if let Some(fruit_to_remove) = env::args().nth(1) {
        let fruit_to_remove = fruit_to_remove.to_string(); // Convert &str to String
        if fruit_set.remove(&fruit_to_remove) {
            println!(
                "Removed '{}'. Updated fruits: {:?}",
                fruit_to_remove, fruit_set
            );
        } else {
            println!("'{}' not found in the set.", fruit_to_remove);
        }
    } else {
        println!("No fruit specified for removal.");
    }

    // Print the unique fruits in reverse order
    println!("Unique fruits in reverse order:");
    for fruit in fruit_set.iter().rev() {
        println!("{}", fruit);
    }

    // Count fruit occurrences
    let fruit_counts = count_fruit_occurrences(&fruits, &amounts);
    println!("\nFruit generation counts:");
    for (fruit, count) in &fruit_counts {
        println!("{}: {}", fruit, count);
    }
}
