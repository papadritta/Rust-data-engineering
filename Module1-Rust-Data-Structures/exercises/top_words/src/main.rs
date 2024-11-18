use std::collections::LinkedList;

fn main() {
    let mut word_list: LinkedList<(&str, usize)> = LinkedList::new();

    // Example data
    word_list.push_back(("hello", 5));
    word_list.push_back(("world", 3));
    word_list.push_back(("rust", 8));

    // Convert LinkedList to Vec for sorting
    let mut word_vec: Vec<_> = word_list.into_iter().collect();

    // Sort the Vec
    word_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Convert Vec back to LinkedList (if needed)
    let sorted_list: LinkedList<_> = word_vec.into_iter().collect();

    // Print sorted list
    for (word, count) in &sorted_list {
        println!("{}: {}", word, count);
    }
}
