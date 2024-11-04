use std::collections::HashMap;

fn main() {
    let mut phone_book = HashMap::new();

    // Add entries to the phone book
    phone_book.insert("Alice", "555-1234");
    phone_book.insert("Bob", "555-5678");
    phone_book.insert("Charlie", "555-8765");

    // Print the phone book
    println!("Phone Book:");
    for (name, number) in &phone_book {
        println!("{}: {}", name, number);
    }

    // Lookup a phone number
    let search_name = "Alice";
    if let Some(&number) = phone_book.get(search_name) {
        println!("{}'s phone number is {}", search_name, number);
    } else {
        println!("{} is not in the phone book.", search_name);
    }
}
