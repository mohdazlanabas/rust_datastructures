# Rust Data Structures Showcase

A comprehensive demonstration of 8 fundamental data structures implemented in Rust. Each data structure is implemented in its own module with practical examples showing real-world usage patterns.

## Features

- Clean, modular architecture with separate modules for each data structure
- Practical demonstrations with real-world use cases
- Well-commented code for learning purposes
- Uses Rust's standard library collections where appropriate

## Data Structures Covered

1. **Array** - Fixed and dynamic sequential collections (`Vec<T>`, `[T; N]`)
2. **Linked List** - Dynamic node-based structure (`LinkedList<T>`)
3. **Stack** - LIFO (Last In, First Out) structure
4. **Queue** - FIFO (First In, First Out) structure (`VecDeque`)
5. **Hash Table** - Key-value mapping (`HashMap<K,V>`)
6. **Tree** - Hierarchical binary search tree
7. **Graph** - Node and edge network with adjacency list
8. **Heap** - Priority queue (min/max heap using `BinaryHeap`)

## Requirements

- Rust 1.56 or later (2021 edition)
- Cargo (included with Rust)

## Building and Running

Clone the repository and run:

```bash
cargo build
cargo run
```

The program will demonstrate all 8 data structures in sequence, showing their core operations and typical use cases.

## App Structure & Architecture

### Module Organization

The project follows a modular architecture where each data structure is isolated in its own module:

```
rust_datastructures/
├── Cargo.toml
├── README.md
├── STRUCTURE.md
└── src/
    ├── main.rs              # Main orchestrator (2.0 KB)
    ├── array_demo.rs        # Array demonstrations (1.6 KB)
    ├── linked_list_demo.rs  # Linked list demonstrations (2.0 KB)
    ├── stack_demo.rs        # Stack demonstrations (2.4 KB)
    ├── queue_demo.rs        # Queue demonstrations (2.1 KB)
    ├── hash_table_demo.rs   # Hash table demonstrations (2.7 KB)
    ├── tree_demo.rs         # Binary search tree demonstrations (3.6 KB)
    ├── graph_demo.rs        # Graph demonstrations (4.1 KB)
    └── heap_demo.rs         # Heap demonstrations (3.5 KB)
```

### How It Works

**Module Declaration (main.rs)**

The main orchestrator declares all 8 modules:

```rust
mod array_demo;
mod linked_list_demo;
mod stack_demo;
mod queue_demo;
mod hash_table_demo;
mod tree_demo;
mod graph_demo;
mod heap_demo;
```

**Execution Flow**

Each module exposes a public `demonstrate()` function that is called sequentially by `main()`:

```rust
fn main() {
    array_demo::demonstrate();
    linked_list_demo::demonstrate();
    stack_demo::demonstrate();
    queue_demo::demonstrate();
    hash_table_demo::demonstrate();
    tree_demo::demonstrate();
    graph_demo::demonstrate();
    heap_demo::demonstrate();
}
```

**Module Pattern**

Each data structure module follows a consistent pattern:

- Implements or uses the data structure
- Provides a public `demonstrate()` function
- Shows practical examples with console output
- Demonstrates core operations (insert, delete, search, etc.)
- Includes real-world use cases

### Design Principles

1. **Separation of Concerns** - Each data structure is completely isolated in its own module
2. **Single Responsibility** - Each module focuses on demonstrating one data structure
3. **Consistency** - All modules follow the same demonstrate() pattern
4. **Clarity** - Code is written for educational purposes with clear examples
5. **Modularity** - Easy to add new data structures or modify existing ones

## Learning Path

Each module follows a consistent pattern:
- Introduction to the data structure
- Core operations (insert, delete, search, etc.)
- Real-world examples
- Time complexity considerations

Start by running the program to see all demonstrations, then explore individual modules in `src/` to understand the implementations.

## License

This project is open source and available for educational purposes.
