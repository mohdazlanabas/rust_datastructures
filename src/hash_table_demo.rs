// HASH TABLE (HashMap) DATA STRUCTURE
// Key-Value pairs with O(1) average lookup time
// Uses hash function to compute index for storage
// Operations: insert, get, remove, contains_key
// Use cases: caching, indexing, counting, lookups

use std::collections::HashMap;

pub fn demonstrate() {
    println!("Hash Table: Fast key-value lookups with O(1) average time\n");

    // Example: Equipment inventory tracking
    let mut equipment_inventory: HashMap<String, i32> = HashMap::new();

    println!("Building equipment inventory:");
    equipment_inventory.insert("Pressure sensors".to_string(), 12);
    equipment_inventory.insert("Temperature probes".to_string(), 8);
    equipment_inventory.insert("Flow meters".to_string(), 5);
    equipment_inventory.insert("Safety valves".to_string(), 15);
    equipment_inventory.insert("pH sensors".to_string(), 6);

    for (item, count) in &equipment_inventory {
        println!("  {}: {} units", item, count);
    }

    // Lookup operations
    println!("\nLookup operations:");
    let item = "Pressure sensors";
    match equipment_inventory.get(item) {
        Some(count) => println!("  {} → {} units in stock", item, count),
        None => println!("  {} → Not found", item),
    }

    // Check if key exists
    let check_item = "Gas detectors";
    println!("\n  Contains '{}'? {}", 
             check_item, 
             equipment_inventory.contains_key(check_item));

    // Update value
    println!("\nUpdating inventory:");
    if let Some(count) = equipment_inventory.get_mut("Flow meters") {
        *count += 3; // Received 3 more units
        println!("  Flow meters updated: {} units", count);
    }

    // Insert or update
    equipment_inventory.entry("Gas detectors".to_string())
        .or_insert(4);
    println!("  Gas detectors added: {} units", 
             equipment_inventory.get("Gas detectors").unwrap());

    // Example: Sensor reading statistics
    println!("\n--- Sensor Reading Count ---");
    let readings = vec![
        "Temperature", "Pressure", "Temperature", "Flow", 
        "Pressure", "Temperature", "pH", "Flow", "Temperature"
    ];

    let mut reading_count: HashMap<&str, i32> = HashMap::new();
    
    for reading in readings {
        *reading_count.entry(reading).or_insert(0) += 1;
    }

    println!("Reading frequency:");
    for (sensor, count) in &reading_count {
        println!("  {}: {} readings", sensor, count);
    }

    // Remove an item
    println!("\nRemoving 'pH sensors' from inventory...");
    if let Some(count) = equipment_inventory.remove("pH sensors") {
        println!("  Removed: pH sensors ({} units)", count);
    }

    println!("\nFinal inventory size: {} items", equipment_inventory.len());
}
