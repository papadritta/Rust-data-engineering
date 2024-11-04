use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    // Simulate people entering the line
    queue.push_back("Alice");
    queue.push_back("Bob");
    queue.push_back("Charlie");

    // Print the queue
    println!("Initial Queue: {:?}", queue);

    // Simulate serving a person in the line
    if let Some(served) = queue.pop_front() {
        println!("Serving: {}", served);
    }

    // Print the queue after serving
    println!("Queue after serving: {:?}", queue);
}
