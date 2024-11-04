use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct LanguageData {
    creation_year: i32,
    num_users: i32,
    growth_rate: f64, // Growth rate as a percentage (e.g., 1.5 for 1.5%)
}

fn init_languages() -> HashMap<String, LanguageData> {
    let mut languages = HashMap::new();
    languages.insert(
        "JavaScript".to_string(),
        LanguageData {
            creation_year: 1995,
            num_users: 1000,
            growth_rate: 2.5,
        },
    );
    languages.insert(
        "HTML/CSS".to_string(),
        LanguageData {
            creation_year: 1990,
            num_users: 1200,
            growth_rate: 1.8,
        },
    );
    languages.insert(
        "Python".to_string(),
        LanguageData {
            creation_year: 1991,
            num_users: 1500,
            growth_rate: 3.0,
        },
    );
    languages.insert(
        "SQL".to_string(),
        LanguageData {
            creation_year: 1974,
            num_users: 800,
            growth_rate: 1.2,
        },
    );
    languages.insert(
        "TypeScript".to_string(),
        LanguageData {
            creation_year: 2012,
            num_users: 600,
            growth_rate: 4.5,
        },
    );
    languages.insert(
        "Bash/Shell".to_string(),
        LanguageData {
            creation_year: 1989,
            num_users: 500,
            growth_rate: 1.1,
        },
    );
    languages.insert(
        "Java".to_string(),
        LanguageData {
            creation_year: 1995,
            num_users: 1100,
            growth_rate: 2.0,
        },
    );
    languages.insert(
        "C#".to_string(),
        LanguageData {
            creation_year: 2000,
            num_users: 900,
            growth_rate: 1.9,
        },
    );
    languages.insert(
        "C++".to_string(),
        LanguageData {
            creation_year: 1985,
            num_users: 700,
            growth_rate: 1.4,
        },
    );
    languages.insert(
        "C".to_string(),
        LanguageData {
            creation_year: 1972,
            num_users: 650,
            growth_rate: 1.0,
        },
    );
    languages.insert(
        "PHP".to_string(),
        LanguageData {
            creation_year: 1995,
            num_users: 750,
            growth_rate: 1.5,
        },
    );
    languages.insert(
        "PowerShell".to_string(),
        LanguageData {
            creation_year: 2006,
            num_users: 400,
            growth_rate: 2.3,
        },
    );
    languages.insert(
        "Go".to_string(),
        LanguageData {
            creation_year: 2007,
            num_users: 550,
            growth_rate: 3.8,
        },
    );
    languages.insert(
        "Rust".to_string(),
        LanguageData {
            creation_year: 2010,
            num_users: 500,
            growth_rate: 4.0,
        },
    );

    languages
}

fn calculate_weights(languages: &mut HashMap<String, LanguageData>) -> HashMap<String, i32> {
    // Calculate active years
    for data in languages.values_mut() {
        data.creation_year = 2024 - data.creation_year;
    }

    // Find min and max for normalization
    let min_year = languages
        .values()
        .map(|data| data.creation_year)
        .min()
        .unwrap_or(0);
    let max_year = languages
        .values()
        .map(|data| data.creation_year)
        .max()
        .unwrap_or(0);
    let min_users = languages
        .values()
        .map(|data| data.num_users)
        .min()
        .unwrap_or(1);
    let max_users = languages
        .values()
        .map(|data| data.num_users)
        .max()
        .unwrap_or(1);
    let min_growth = languages
        .values()
        .map(|data| data.growth_rate)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);
    let max_growth = languages
        .values()
        .map(|data| data.growth_rate)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);

    let mut weights = HashMap::new();

    for (language, data) in languages.iter() {
        // Normalize active years, number of users, and growth rate
        let normalized_year = (data.creation_year - min_year) as f64 / (max_year - min_year) as f64;
        let normalized_users = (data.num_users - min_users) as f64 / (max_users - min_users) as f64;
        let normalized_growth = (data.growth_rate - min_growth) / (max_growth - min_growth);

        // Combine the factors to calculate the weight (adjust weights as needed)
        let combined_weight =
            (0.5 * normalized_year + 0.3 * normalized_users + 0.2 * normalized_growth) * 99.0 + 1.0;
        let weight = combined_weight as i32;

        weights.insert(language.clone(), weight);
    }

    weights
}

fn main() {
    let mut languages = init_languages();

    // Prompt the user to add new languages
    println!("Would you like to add new languages? Enter 'yes' to add or 'no' to skip:");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read input");
    response = response.trim().to_lowercase();

    while response == "yes" {
        println!("Enter the name of the language:");
        let mut language = String::new();
        io::stdin()
            .read_line(&mut language)
            .expect("Failed to read input");
        let language = language.trim().to_string();

        println!("Enter the year the language was created:");
        let mut year_input = String::new();
        io::stdin()
            .read_line(&mut year_input)
            .expect("Failed to read input");

        println!("Enter the number of users (e.g., 1000):");
        let mut users_input = String::new();
        io::stdin()
            .read_line(&mut users_input)
            .expect("Failed to read input");

        println!("Enter the growth rate as a percentage (e.g., 3.5 for 3.5%):");
        let mut growth_input = String::new();
        io::stdin()
            .read_line(&mut growth_input)
            .expect("Failed to read input");

        // Parse the inputs and add the language to the HashMap if valid
        if let (Ok(year), Ok(num_users), Ok(growth_rate)) = (
            year_input.trim().parse::<i32>(),
            users_input.trim().parse::<i32>(),
            growth_input.trim().parse::<f64>(),
        ) {
            languages.insert(
                language.clone(),
                LanguageData {
                    creation_year: year,
                    num_users,
                    growth_rate,
                },
            );
            println!(
                "Added {} (created in {}, {} users, {:.1}% growth) to the list.",
                language, year, num_users, growth_rate
            );
        } else {
            println!("Invalid input entered. Please enter valid numbers.");
        }

        // Ask if the user wants to add more languages
        println!(
            "Would you like to add another language? Enter 'yes' to continue or 'no' to stop:"
        );
        response.clear();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read input");
        response = response.trim().to_lowercase();
    }

    // Calculate the weights and print the results sorted by weight
    let weights = calculate_weights(&mut languages);
    let mut weights_vec: Vec<(String, i32)> = weights.into_iter().collect();
    weights_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by weight in descending order

    println!("Language weighing from 1-100 by combined factors (sorted by weight):");
    for (language, weight) in weights_vec {
        println!("{}: {}", language, weight);
    }
}
