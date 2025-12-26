// LINKED LIST DATA STRUCTURE
// Each node contains data and a pointer to the next node
// - Efficient insertion/deletion at ends
// - Sequential access (no random access)
// - Uses more memory than arrays (due to pointers)

use std::collections::LinkedList;

pub fn demonstrate() {
    println!("Linked List: Dynamic collection with efficient insertion/removal\n");

    // Create a linked list for maintenance tasks
    let mut tasks: LinkedList<String> = LinkedList::new();

    // Add tasks to the back
    println!("Adding maintenance tasks:");
    tasks.push_back("Inspect boiler pressure".to_string());
    tasks.push_back("Check digester temperature".to_string());
    tasks.push_back("Test safety valves".to_string());
    tasks.push_back("Calibrate sensors".to_string());
    
    println!("  Tasks queue: {} items", tasks.len());
    for (i, task) in tasks.iter().enumerate() {
        println!("    {}. {}", i + 1, task);
    }

    // Add urgent task at front
    println!("\nUrgent task added to front:");
    tasks.push_front("URGENT: Check gas leak alarm".to_string());
    
    for (i, task) in tasks.iter().enumerate() {
        println!("    {}. {}", i + 1, task);
    }

    // Process tasks from front
    println!("\nProcessing tasks:");
    while let Some(task) = tasks.pop_front() {
        println!("  ✓ Completed: {}", task);
        if tasks.len() <= 2 {
            break; // Stop after processing a few
        }
    }

    println!("\nRemaining tasks: {}", tasks.len());
    for (i, task) in tasks.iter().enumerate() {
        println!("    {}. {}", i + 1, task);
    }

    // Demonstrate append
    let mut additional_tasks = LinkedList::new();
    additional_tasks.push_back("Update logbook".to_string());
    additional_tasks.push_back("Report to supervisor".to_string());
    
    println!("\nAppending additional tasks...");
    tasks.append(&mut additional_tasks);
    println!("  Total tasks now: {}", tasks.len());
}
