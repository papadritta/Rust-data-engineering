use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    id: usize,
    tx: mpsc::Sender<Request>,
}

struct Request {
    philosopher_id: usize,
    response_tx: mpsc::Sender<bool>,
}

impl Philosopher {
    fn new(name: &str, id: usize, tx: mpsc::Sender<Request>) -> Self {
        Philosopher {
            name: name.to_string(),
            id,
            tx,
        }
    }

    fn dine(&self) {
        loop {
            let (response_tx, response_rx) = mpsc::channel();
            self.tx
                .send(Request {
                    philosopher_id: self.id,
                    response_tx,
                })
                .unwrap();

            if response_rx.recv().unwrap() {
                println!("{} picked up forks.", self.name);
                println!("{} is eating.", self.name);
                thread::sleep(Duration::from_secs(1));
                println!("{} put down forks.", self.name);
            }

            thread::sleep(Duration::from_millis(500)); // Wait before requesting again
        }
    }
}

fn controller(num_philosophers: usize, rx: mpsc::Receiver<Request>) {
    let forks = Arc::new(Mutex::new(vec![true; num_philosophers])); // Forks availability

    while let Ok(request) = rx.recv() {
        let forks_clone = Arc::clone(&forks); // Clone `Arc` to move into the thread
        let mut available_forks = forks.lock().unwrap();

        let left = request.philosopher_id;
        let right = (request.philosopher_id + 1) % num_philosophers;

        if available_forks[left] && available_forks[right] {
            available_forks[left] = false;
            available_forks[right] = false;

            request.response_tx.send(true).unwrap(); // Grant access
        } else {
            request.response_tx.send(false).unwrap(); // Deny access
        }

        // After philosopher has eaten, release forks
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1)); // Simulate eating time
            let mut available_forks = forks_clone.lock().unwrap();
            available_forks[left] = true;
            available_forks[right] = true;
        });
    }
}

fn main() {
    const NUM_PHILOSOPHERS: usize = 4;

    let (tx, rx) = mpsc::channel();
    let controller_handle = thread::spawn(move || controller(NUM_PHILOSOPHERS, rx));

    let philosophers: Vec<_> = (0..NUM_PHILOSOPHERS)
        .map(|id| {
            let name = format!("Philosopher {}", id + 1);
            Philosopher::new(&name, id, tx.clone())
        })
        .collect();

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|philosopher| thread::spawn(move || philosopher.dine()))
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    controller_handle.join().unwrap();
}
