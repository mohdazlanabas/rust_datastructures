// STACK DATA STRUCTURE
// LIFO: Last In, First Out
// Like a stack of plates - you add and remove from the top
// Operations: push (add), pop (remove), peek (view top)
// Use cases: undo functionality, expression evaluation, backtracking

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

pub fn demonstrate() {
    println!("Stack: LIFO (Last In, First Out) structure\n");

    // Example: Process control system state tracking
    let mut system_states: Stack<String> = Stack::new();

    println!("System startup sequence (states pushed to stack):");
    
    let states = vec![
        "Idle",
        "Initializing sensors",
        "Calibrating instruments",
        "Pre-heating reactor",
        "Normal operation",
    ];

    for state in states {
        system_states.push(state.to_string());
        println!("  → Pushed: {}", state);
    }

    println!("\nCurrent state (peek): {:?}", system_states.peek());
    println!("Stack size: {}", system_states.size());

    // Emergency shutdown - undo states in reverse order
    println!("\nEmergency shutdown (pop states in reverse):");
    while let Some(state) = system_states.pop() {
        println!("  ← Popped: {}", state);
        if system_states.size() == 1 {
            break;
        }
    }

    println!("\nFinal state: {:?}", system_states.peek());
    println!("Is empty? {}", system_states.is_empty());

    // Another example: Undo functionality
    println!("\n--- Undo Operation Example ---");
    let mut actions: Stack<String> = Stack::new();
    
    actions.push("Set temperature to 75°C".to_string());
    actions.push("Increase pressure to 150 kPa".to_string());
    actions.push("Enable mixing".to_string());
    
    println!("Actions performed: {}", actions.size());
    
    println!("Undoing last action...");
    if let Some(action) = actions.pop() {
        println!("  Undone: {}", action);
    }
    
    println!("Remaining actions: {}", actions.size());
}
