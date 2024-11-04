use std::collections::HashMap;
use std::io;

fn logic(words: Vec<String>) -> Vec<(String, u32)> {
    let mut frequencies = HashMap::new();

    // Count the frequency of each word
    for word in words {
        let frequency = frequencies.entry(word).or_insert(0);
        *frequency += 1;
    }

    // Collect the results into a vector
    let mut result: Vec<(String, u32)> = frequencies.into_iter().collect();

    // Sort the results by frequency in descending order
    result.sort_by(|a, b| b.1.cmp(&a.1));

    result
}

fn main() {
    // Prompt the user for input
    println!("Enter a sentence:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input into a vector of words
    let words: Vec<String> = input
        .split_whitespace() // Split by spaces
        .map(|s| s.to_string()) // Convert each &str to String
        .collect();

    // Calculate the frequency of each word and sort by frequency
    let result = logic(words);

    // Print the results in a human-readable format
    println!("The frequency of each word in the sentence is (sorted by frequency):");
    for (word, frequency) in result {
        println!("Word '{}' appears {} time(s)", word, frequency);
    }
}
