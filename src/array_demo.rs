// ARRAY DATA STRUCTURE
// Arrays store elements in contiguous memory locations
// - Fixed-size arrays: [T; N] - size known at compile time
// - Dynamic arrays: Vec<T> - size can grow/shrink at runtime

pub fn demonstrate() {
    println!("Arrays provide O(1) access by index\n");

    // Fixed-size array
    println!("Fixed-size array:");
    let temperatures: [f64; 5] = [23.5, 25.0, 24.8, 26.2, 25.5];
    println!("  Temperatures (°C): {:?}", temperatures);
    println!("  First reading: {}", temperatures[0]);
    println!("  Last reading: {}", temperatures[temperatures.len() - 1]);
    
    // Calculate average
    let sum: f64 = temperatures.iter().sum();
    let avg = sum / temperatures.len() as f64;
    println!("  Average: {:.2}°C", avg);

    // Dynamic array (Vector)
    println!("\nDynamic array (Vec):");
    let mut pressures: Vec<f64> = Vec::new();
    
    // Simulating sensor readings
    pressures.push(101.3); // kPa
    pressures.push(102.1);
    pressures.push(100.8);
    pressures.push(101.5);
    
    println!("  Pressure readings (kPa): {:?}", pressures);
    println!("  Count: {}", pressures.len());
    println!("  Capacity: {}", pressures.capacity());
    
    // Remove last element
    if let Some(last) = pressures.pop() {
        println!("  Removed last reading: {}", last);
    }
    println!("  After removal: {:?}", pressures);
    
    // Iterate and process
    println!("\n  Processing all readings:");
    for (index, &pressure) in pressures.iter().enumerate() {
        println!("    Reading {}: {:.2} kPa", index + 1, pressure);
    }
}
