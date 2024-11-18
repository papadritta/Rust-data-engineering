use std::collections::HashMap;
use std::fs;

fn clean_word(word: &str) -> String {
    word.to_lowercase()
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_string() // Convert the cleaned word back to a String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("text.txt")?;
    let mut word_count = HashMap::new();

    for word in file_content.split_whitespace() {
        let cleaned_word = clean_word(word);
        if !cleaned_word.is_empty() {
            *word_count.entry(cleaned_word).or_insert(0) += 1;
        }
    }

    println!("Word frequencies:");
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

    Ok(())
}
