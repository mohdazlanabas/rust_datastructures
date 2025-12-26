// Main application to demonstrate 8 core data structures
// Each data structure is implemented in its own module for clarity

mod array_demo;
mod linked_list_demo;
mod stack_demo;
mod queue_demo;
mod hash_table_demo;
mod tree_demo;
mod graph_demo;
mod heap_demo;

fn main() {
    println!("=================================================");
    println!("   RUST DATA STRUCTURES SHOWCASE");
    println!("   Demonstrating 8 Core Data Structures");
    println!("=================================================\n");

    // 1. Array Demo
    println!("\n{:=<50}", "");
    println!("1. ARRAY - Fixed/Dynamic Sequential Collection");
    println!("{:=<50}", "");
    array_demo::demonstrate();

    // 2. Linked List Demo
    println!("\n{:=<50}", "");
    println!("2. LINKED LIST - Dynamic Node-Based Structure");
    println!("{:=<50}", "");
    linked_list_demo::demonstrate();

    // 3. Stack Demo
    println!("\n{:=<50}", "");
    println!("3. STACK - LIFO (Last In, First Out)");
    println!("{:=<50}", "");
    stack_demo::demonstrate();

    // 4. Queue Demo
    println!("\n{:=<50}", "");
    println!("4. QUEUE - FIFO (First In, First Out)");
    println!("{:=<50}", "");
    queue_demo::demonstrate();

    // 5. Hash Table Demo
    println!("\n{:=<50}", "");
    println!("5. HASH TABLE - Key-Value Mapping");
    println!("{:=<50}", "");
    hash_table_demo::demonstrate();

    // 6. Tree Demo
    println!("\n{:=<50}", "");
    println!("6. TREE - Hierarchical Binary Search Tree");
    println!("{:=<50}", "");
    tree_demo::demonstrate();

    // 7. Graph Demo
    println!("\n{:=<50}", "");
    println!("8. GRAPH - Nodes and Edges Network");
    println!("{:=<50}", "");
    graph_demo::demonstrate();

    // 8. Heap Demo
    println!("\n{:=<50}", "");
    println!("8. HEAP - Priority Queue (Min/Max)");
    println!("{:=<50}", "");
    heap_demo::demonstrate();

    println!("\n{:=<50}", "");
    println!("All demonstrations completed!");
    println!("{:=<50}\n", "");
}
