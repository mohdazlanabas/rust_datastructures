// QUEUE DATA STRUCTURE
// FIFO: First In, First Out
// Like a queue at a bank - first person to arrive is first to be served
// Operations: enqueue (add to back), dequeue (remove from front)
// Use cases: job scheduling, message queues, breadth-first search

use std::collections::VecDeque;

pub fn demonstrate() {
    println!("Queue: FIFO (First In, First Out) structure\n");

    // Using VecDeque which is Rust's double-ended queue
    let mut job_queue: VecDeque<String> = VecDeque::new();

    println!("Job scheduling system - adding jobs to queue:");
    
    // Enqueue jobs
    let jobs = vec![
        "Analyze water quality",
        "Generate daily report",
        "Backup system data",
        "Run diagnostics",
        "Update firmware",
    ];

    for job in jobs {
        job_queue.push_back(job.to_string());
        println!("  Enqueued: {}", job);
    }

    println!("\nQueue size: {}", job_queue.len());
    println!("Front of queue: {:?}", job_queue.front());
    println!("Back of queue: {:?}", job_queue.back());

    // Process jobs (dequeue)
    println!("\nProcessing jobs in order:");
    let mut processed = 0;
    while let Some(job) = job_queue.pop_front() {
        processed += 1;
        println!("  [Job {}] Processing: {}", processed, job);
        
        // Stop after processing a few for demo
        if processed >= 3 {
            println!("  ... pausing processing");
            break;
        }
    }

    println!("\nJobs remaining in queue: {}", job_queue.len());
    for (i, job) in job_queue.iter().enumerate() {
        println!("  Position {}: {}", i + 1, job);
    }

    // Demonstrate priority addition
    println!("\nAdding high-priority job to front:");
    job_queue.push_front("CRITICAL: Emergency shutdown sequence".to_string());
    
    println!("Updated queue:");
    for (i, job) in job_queue.iter().enumerate() {
        println!("  Position {}: {}", i + 1, job);
    }

    // Clear demonstration
    println!("\nQueue length: {}", job_queue.len());
    println!("Is empty? {}", job_queue.is_empty());
}
