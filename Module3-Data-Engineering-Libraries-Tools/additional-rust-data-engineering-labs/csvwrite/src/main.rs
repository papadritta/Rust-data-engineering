use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a list of fruits and their prices
    let fruits = [
        ("Apple", 1.25),
        ("Banana", 0.75),
        ("Orange", 1.00),
        ("Mango", 2.50),
        ("Pineapple", 3.00),
    ];

    // Open the CSV file in write mode
    let mut wtr = Writer::from_path("output.csv")?;

    // Write the header row
    wtr.write_record(["Fruit", "Price"])?;

    // Write each fruit and its price to the CSV file
    for (fruit, price) in fruits {
        wtr.write_record([fruit, &price.to_string()])?; // Convert price to string
    }

    wtr.flush()?; // Ensure data is written

    Ok(())
}
