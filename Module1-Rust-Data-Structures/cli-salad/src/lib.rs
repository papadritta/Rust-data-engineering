use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize, custom_fruits: Vec<String>) -> Vec<String> {
    // Default list of fruits if custom_fruits is empty
    let fruits = if custom_fruits.is_empty() {
        vec![
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
        ]
    } else {
        custom_fruits
    };

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}
