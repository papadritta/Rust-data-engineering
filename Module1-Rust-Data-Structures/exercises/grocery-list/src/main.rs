fn main() {
    // Grocery list stored in a vector
    let grocery_list = vec!["Milk", "Eggs", "Bread", "Butter", "Cheese"];

    // Print out each item
    println!("Grocery List:");
    for item in &grocery_list {
        println!("- {}", item);
    }
}
