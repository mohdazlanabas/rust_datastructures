PROJECT STRUCTURE
=================

data_structures_showcase/
│
├── Cargo.toml                  # Project configuration
├── README.md                   # Full documentation
├── QUICKSTART.md              # Quick start guide
│
└── src/                       # Source code directory
    │
    ├── main.rs                # Main orchestrator (pulls all 8 modules)
    │
    ├── array_demo.rs          # 1. ARRAY - Vec<T>, [T; N]
    ├── linked_list_demo.rs    # 2. LINKED LIST - LinkedList<T>
    ├── stack_demo.rs          # 3. STACK - LIFO structure
    ├── queue_demo.rs          # 4. QUEUE - FIFO structure (VecDeque)
    ├── hash_table_demo.rs     # 5. HASH TABLE - HashMap<K,V>
    ├── tree_demo.rs           # 6. TREE - Binary Search Tree
    ├── graph_demo.rs          # 7. GRAPH - Adjacency list
    └── heap_demo.rs           # 8. HEAP - BinaryHeap (priority queue)


HOW main.rs IMPORTS THE 8 MODULES
==================================

In main.rs (line 4-11):

mod array_demo;           // Declares array_demo module → looks for src/array_demo.rs
mod linked_list_demo;     // Declares linked_list_demo module → looks for src/linked_list_demo.rs
mod stack_demo;           // Declares stack_demo module → looks for src/stack_demo.rs
mod queue_demo;           // Declares queue_demo module → looks for src/queue_demo.rs
mod hash_table_demo;      // Declares hash_table_demo module → looks for src/hash_table_demo.rs
mod tree_demo;            // Declares tree_demo module → looks for src/tree_demo.rs
mod graph_demo;           // Declares graph_demo module → looks for src/graph_demo.rs
mod heap_demo;            // Declares heap_demo module → looks for src/heap_demo.rs


Then main() calls each module's demonstrate() function:

fn main() {
    array_demo::demonstrate();         // Calls demonstrate() from array_demo.rs
    linked_list_demo::demonstrate();   // Calls demonstrate() from linked_list_demo.rs
    stack_demo::demonstrate();         // Calls demonstrate() from stack_demo.rs
    queue_demo::demonstrate();         // Calls demonstrate() from queue_demo.rs
    hash_table_demo::demonstrate();    // Calls demonstrate() from hash_table_demo.rs
    tree_demo::demonstrate();          // Calls demonstrate() from tree_demo.rs
    graph_demo::demonstrate();         // Calls demonstrate() from graph_demo.rs
    heap_demo::demonstrate();          // Calls demonstrate() from heap_demo.rs
}


FILE SIZES
==========
array_demo.rs        : 1.6 KB
linked_list_demo.rs  : 2.0 KB
stack_demo.rs        : 2.4 KB
queue_demo.rs        : 2.1 KB
hash_table_demo.rs   : 2.7 KB
tree_demo.rs         : 3.6 KB
graph_demo.rs        : 4.1 KB
heap_demo.rs         : 3.5 KB
main.rs              : 2.0 KB


TOTAL: 9 separate .rs files
- 1 main orchestrator (main.rs)
- 8 data structure modules (one per data structure)
