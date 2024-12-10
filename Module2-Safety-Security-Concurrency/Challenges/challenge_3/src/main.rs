use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

fn traverse_directory(path: &Path, tx: mpsc::Sender<u64>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let tx_clone = tx.clone();
                thread::spawn(move || traverse_directory(&path, tx_clone));
            } else if let Ok(metadata) = entry.metadata() {
                tx.send(metadata.len()).unwrap();
            }
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let path = Path::new(".");

    thread::spawn(move || traverse_directory(&path, tx));

    let total_size: u64 = rx.iter().sum();
    println!("Total size: {} bytes", total_size);
}
