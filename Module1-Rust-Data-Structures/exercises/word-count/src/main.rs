use std::collections::HashMap;
use std::fs;

fn main() {
    // Read the contents of the file
    let contents = fs::read_to_string("text.txt").expect("Failed to read file");

    // Create a HashMap to store the word counts
    let mut word_count = HashMap::new();

    // Split the contents into words and count them
    for word in contents.split_whitespace() {
        let word = word.to_lowercase();
        *word_count.entry(word).or_insert(0) += 1;
    }

    // Print the word counts
    println!("Word Counts:");
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}
