use rand::seq::SliceRandom; // Corrected import for SliceRandom trait
use rand::thread_rng;
use std::collections::LinkedList;
use std::io;

fn main() {
    let mut fruit: LinkedList<String> = LinkedList::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Convert the LinkedList to a Vec for shuffling
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<_> = fruit.iter().cloned().collect(); // Use `iter().cloned()` to avoid consuming `fruit`
    fruit_vec.shuffle(&mut rng);

    // Convert it back to LinkedList
    fruit = fruit_vec.into_iter().collect();

    // Add predefined fruits to both ends after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    // Print the fruit salad after shuffling and adding predefined fruits
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Allow the user to insert fruits at any position in the LinkedList
    println!("You can add fruits at any position in the queue. Type the position followed by the fruit name (e.g., '2 Mango'), or type 'done' to stop:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "done" {
            break;
        }

        // Parse the position and fruit from the user input
        let mut parts = input.splitn(2, ' ');
        if let (Some(pos_str), Some(fruit_name)) = (parts.next(), parts.next()) {
            if let Ok(pos) = pos_str.parse::<usize>() {
                let mut temp_vec: Vec<_> = fruit.iter().cloned().collect(); // Convert LinkedList to Vec
                if pos <= temp_vec.len() {
                    temp_vec.insert(pos, fruit_name.to_string());
                    fruit = temp_vec.into_iter().collect(); // Convert Vec back to LinkedList

                    // Print the updated fruit salad
                    println!("Updated Fruit Salad:");
                    for (i, item) in fruit.iter().enumerate() {
                        if i != fruit.len() - 1 {
                            print!("{}, ", item);
                        } else {
                            println!("{}", item);
                        }
                    }
                } else {
                    println!(
                        "Invalid position. Please enter a position between 0 and {}.",
                        temp_vec.len()
                    );
                }
            } else {
                println!("Invalid input. Please use the format '<position> <fruit>' or type 'done' to finish.");
            }
        } else {
            println!("Invalid input. Please use the format '<position> <fruit>' or type 'done' to finish.");
        }
    }

    // Use the `choose` method to select a random fruit from the salad
    let fruit_slice: Vec<&String> = fruit.iter().collect(); // Convert LinkedList to a slice for `choose`
    if let Some(random_fruit) = fruit_slice.choose(&mut rng) {
        println!("Randomly selected fruit from the salad: {}", random_fruit);
    } else {
        println!("The fruit salad is empty!");
    }

    // Allow the user to remove fruits from any position in the LinkedList
    println!("You can remove a fruit at any position in the list. Type the position to remove (e.g., '2'), or type 'done' to stop:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "done" {
            break;
        }

        if let Ok(pos) = input.parse::<usize>() {
            let mut temp_vec: Vec<_> = fruit.iter().cloned().collect(); // Convert LinkedList to Vec
            if pos < temp_vec.len() {
                let removed_fruit = temp_vec.remove(pos); // Remove fruit at the specified position
                println!("Removed fruit: {}", removed_fruit);
                fruit = temp_vec.into_iter().collect(); // Convert Vec back to LinkedList

                // Print the updated fruit salad
                println!("Updated Fruit Salad:");
                for (i, item) in fruit.iter().enumerate() {
                    if i != fruit.len() - 1 {
                        print!("{}, ", item);
                    } else {
                        println!("{}", item);
                    }
                }
            } else {
                println!(
                    "Invalid position. Please enter a position between 0 and {}.",
                    temp_vec.len() - 1
                );
            }
        } else {
            println!("Invalid input. Please enter a valid number or type 'done' to finish.");
        }
    }
}
