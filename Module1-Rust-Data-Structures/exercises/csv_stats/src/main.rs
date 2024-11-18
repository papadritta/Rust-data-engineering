use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut values = Vec::new();

    for record in csv_reader.records() {
        let record = record?;
        let value: f64 = record[0].parse()?; // Assume data is in the first column
        values.push(value);
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len() as f64;

    println!("Min: {}", min);
    println!("Max: {}", max);
    println!("Mean: {}", mean);

    Ok(())
}
