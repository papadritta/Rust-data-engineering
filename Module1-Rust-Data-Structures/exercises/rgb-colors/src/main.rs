fn main() {
    // RGB color values stored in a vector
    let rgb_colors = vec![
        (255, 0, 0),   // Red
        (0, 255, 0),   // Green
        (0, 0, 255),   // Blue
        (255, 255, 0), // Yellow
    ];

    // Print out the RGB values
    println!("RGB Colors:");
    for (r, g, b) in &rgb_colors {
        println!("R: {}, G: {}, B: {}", r, g, b);
    }
}
