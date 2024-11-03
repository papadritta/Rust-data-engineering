use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Create a fruit salad with a specified number of fruits or custom fruits"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,

    // Adjusted for `clap` to parse multiple values correctly
    #[clap(short, long, value_parser, num_args = 1..)]
    custom_fruits: Option<Vec<String>>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Check if custom fruits were provided, otherwise pass an empty vector
    let available_fruits = match &opts.custom_fruits {
        Some(fruits) => fruits.clone(),
        None => vec![
            "Arbutus".to_string(),
            "Loquat".to_string(),
            "Strawberry Tree Berry".to_string(),
            "Pomegranate".to_string(),
            "Fig".to_string(),
            "Cherry".to_string(),
            "Orange".to_string(),
            "Pear".to_string(),
            "Peach".to_string(),
            "Apple".to_string(),
        ],
    };

    // Validate input: Check if the requested number of fruits exceeds the available number
    if num_fruits > available_fruits.len() {
        eprintln!(
            "Error: Requested {} fruits, but only {} are available.",
            num_fruits,
            available_fruits.len()
        );
        std::process::exit(1); // Exit the program with an error code
    }

    // Create the fruit salad
    let mut fruit_salad = create_fruit_salad(num_fruits, available_fruits);

    // Print the fruit salad in human-readable format with a count of fruits used
    println!(
        "Created Fruit Salad with {} fruits: {:?}",
        num_fruits, fruit_salad
    );

    // Sort the fruits in alphabetical order and print them
    fruit_salad.sort();
    println!("Fruit Salad in alphabetical order: {:?}", fruit_salad);
}
