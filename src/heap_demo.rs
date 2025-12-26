// HEAP DATA STRUCTURE (Priority Queue)
// Binary tree where parent is always greater (max-heap) or smaller (min-heap) than children
// Efficient for finding min/max element: O(1)
// Insert and remove: O(log n)
// Use cases: priority queues, scheduling, finding k largest/smallest elements

use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn demonstrate() {
    println!("Heap: Priority Queue for efficient min/max operations\n");

    // Max-Heap example: Priority task queue (higher number = higher priority)
    println!("Max-Heap: Priority Task Queue");
    let mut task_queue: BinaryHeap<Task> = BinaryHeap::new();

    println!("Adding tasks with priorities:");
    
    let tasks = vec![
        Task { priority: 3, description: "Regular maintenance".to_string() },
        Task { priority: 9, description: "Emergency shutdown".to_string() },
        Task { priority: 5, description: "Safety inspection".to_string() },
        Task { priority: 7, description: "Critical alarm response".to_string() },
        Task { priority: 2, description: "Update logs".to_string() },
        Task { priority: 8, description: "Equipment malfunction".to_string() },
    ];

    for task in tasks {
        println!("  Added: [P:{}] {}", task.priority, task.description);
        task_queue.push(task);
    }

    println!("\nProcessing tasks by priority (highest first):");
    while let Some(task) = task_queue.pop() {
        println!("  → [Priority {}] {}", task.priority, task.description);
    }

    // Min-Heap example: Temperature monitoring (lowest temperature first)
    println!("\n--- Min-Heap: Temperature Alerts (Lowest First) ---");
    
    // BinaryHeap is max-heap by default, use Reverse for min-heap
    let mut temp_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let temperatures = vec![28, 15, 32, 18, 25, 12, 30];
    
    println!("Recording temperatures (°C):");
    for &temp in &temperatures {
        temp_heap.push(Reverse(temp));
        println!("  Recorded: {}°C", temp);
    }

    println!("\nTemperature alerts (coldest first):");
    while let Some(Reverse(temp)) = temp_heap.pop() {
        println!("  → {}°C", temp);
        if temp > 20 {
            println!("    (Above threshold, stopping alerts)");
            break;
        }
    }

    // Example: Top K largest values
    println!("\n--- Finding Top 3 Pressure Readings ---");
    let pressures = vec![101.3, 102.5, 100.8, 103.2, 99.5, 102.1, 101.7];
    
    let mut pressure_heap: BinaryHeap<i32> = BinaryHeap::new();
    
    println!("Pressure readings (kPa):");
    for &pressure in &pressures {
        let pressure_int = (pressure * 10.0) as i32;
        pressure_heap.push(pressure_int);
        print!("  {:.1}", pressure);
    }
    println!();

    println!("\nTop 3 highest pressures:");
    for i in 1..=3 {
        if let Some(pressure) = pressure_heap.pop() {
            println!("  {}. {:.1} kPa", i, pressure as f64 / 10.0);
        }
    }

    // Peek at top without removing
    println!("\nRemaining pressures: {}", pressure_heap.len());
    if let Some(&next) = pressure_heap.peek() {
        println!("Next highest: {:.1} kPa", next as f64 / 10.0);
    }
}
