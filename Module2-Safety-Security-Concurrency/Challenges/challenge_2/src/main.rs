use reqwest;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://example.com",
        "https://example.org",
        "https://example.net",
    ];

    let semaphore = Arc::new(Semaphore::new(2)); // Limit to 2 concurrent requests
    let handles: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let semaphore = Arc::clone(&semaphore);
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                println!("Fetching: {}", url);
                let _response = reqwest::get(url).await.unwrap();
                sleep(Duration::from_secs(1)).await; // Politeness delay
                println!("Done: {}", url);
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }
}
