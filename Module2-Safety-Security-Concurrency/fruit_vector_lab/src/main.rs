use std::collections::HashMap;

fn main() {
    // Step 1: Create a Vector of fruits
    let mut fruit_salad = vec![
        "apple", "banana", "cherry", "mango", "apple", "grape", "banana",
    ];

    // Step 2: Add a new fruit using the `push` method
    fruit_salad.push("orange");
    println!("After adding orange: {:?}", fruit_salad);

    // Step 3: Remove the last fruit using the `pop` method
    if let Some(removed_fruit) = fruit_salad.pop() {
        println!("Removed fruit: {}", removed_fruit);
    }
    println!("After removing the last fruit: {:?}", fruit_salad);

    // Step 4: Iterate through the Vector and print each fruit
    println!("Final fruit salad:");
    for fruit in &fruit_salad {
        println!("{}", fruit);
    }

    // Step 5: Remove a specific fruit
    remove_fruit(&mut fruit_salad, "banana");
    println!("After removing banana: {:?}", fruit_salad);

    // Step 6: Sort the fruits alphabetically
    fruit_salad.sort();
    println!("Fruits sorted alphabetically: {:?}", fruit_salad);

    // Step 7: Count occurrences of each fruit
    let occurrences = count_fruit_occurrences(&fruit_salad);
    println!("Occurrences of each fruit:");
    for (fruit, count) in occurrences {
        println!("{}: {}", fruit, count);
    }
}

// Function to remove a specific fruit from the Vector
fn remove_fruit(fruits: &mut Vec<&str>, fruit_name: &str) {
    fruits.retain(|&fruit| fruit != fruit_name);
}

// Function to count the occurrences of each fruit in the Vector
fn count_fruit_occurrences<'a>(fruits: &'a Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut occurrences = HashMap::new();
    for &fruit in fruits {
        *occurrences.entry(fruit).or_insert(0) += 1;
    }
    occurrences
}
