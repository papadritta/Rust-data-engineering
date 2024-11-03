use rand::seq::SliceRandom; // Corrected import for SliceRandom trait
use rand::thread_rng;
use std::collections::VecDeque;
use std::io;

fn main() {
    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Shuffle the fruit after converting to a Vec
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert it back to VecDeque
    fruit = fruit_vec.into_iter().collect();

    // Add predefined fruits to both ends after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    // Print the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Allow the user to add fruits to either end of the queue
    println!("You can add fruits to the front or back of the queue. Type 'front' or 'back' followed by the fruit name, or type 'done' to stop:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "done" {
            break;
        }

        if input.starts_with("front ") {
            let fruit_to_add = input[6..].trim().to_string();
            fruit.push_front(fruit_to_add);
        } else if input.starts_with("back ") {
            let fruit_to_add = input[5..].trim().to_string();
            fruit.push_back(fruit_to_add);
        } else {
            println!("Invalid input. Please use 'front <fruit>' or 'back <fruit>' or type 'done' to finish.");
        }
    }

    // Print the updated fruit salad
    println!("Updated Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // Use the `choose` method to select a random fruit from the salad
    let fruit_slice: Vec<&String> = fruit.iter().collect(); // Convert VecDeque to a slice
    if let Some(random_fruit) = fruit_slice.choose(&mut rng) {
        println!("Randomly selected fruit from the salad: {}", random_fruit);
    } else {
        println!("The fruit salad is empty!");
    }

    // Allow the user to remove fruits from either end of the queue
    println!("You can remove a fruit from the front or back of the queue. Type 'remove front' or 'remove back', or type 'done' to stop:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "done" {
            break;
        }

        if input == "remove front" {
            if let Some(removed_fruit) = fruit.pop_front() {
                println!("Removed fruit from the front: {}", removed_fruit);
            } else {
                println!("The queue is empty; nothing to remove from the front.");
            }
        } else if input == "remove back" {
            if let Some(removed_fruit) = fruit.pop_back() {
                println!("Removed fruit from the back: {}", removed_fruit);
            } else {
                println!("The queue is empty; nothing to remove from the back.");
            }
        } else {
            println!("Invalid input. Please use 'remove front', 'remove back', or type 'done' to finish.");
        }

        // Display the current state of the queue
        println!("Current Fruit Salad:");
        for (i, item) in fruit.iter().enumerate() {
            if i != fruit.len() - 1 {
                print!("{}, ", item);
            } else {
                println!("{}", item);
            }
        }
    }
}
