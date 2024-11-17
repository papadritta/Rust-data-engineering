use rasciigraph::{plot, Config};

fn main() {
    let cities = vec![
        "Lisbon",
        "Madrid",
        "Paris",
        "Berlin",
        "Copenhagen",
        "Stockholm",
        "Moscow",
    ];

    let distances_set1 = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];
    let distances_set2 = vec![0.0, 450.0, 900.0, 1800.0, 2600.0, 3200.0, 4500.0];

    println!("Route 1: {}", cities.join(" > "));
    println!(
        "{}",
        plot(
            distances_set1.clone(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Route 1 - Travelled distances (km)".to_string())
        )
    );

    println!("\nRoute 2: {}", cities.join(" > "));
    println!(
        "{}",
        plot(
            distances_set2.clone(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Route 2 - Travelled distances (km)".to_string())
        )
    );
}
