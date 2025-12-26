// TREE DATA STRUCTURE (Binary Search Tree)
// Hierarchical structure with nodes
// Each node has at most 2 children (left and right)
// BST property: left < parent < right
// Operations: insert, search, traverse
// Use cases: sorted data, hierarchical data, databases

use std::cmp::Ordering;

#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                match &mut self.left {
                    Some(left_node) => left_node.insert(value),
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                }
            }
            Ordering::Greater => {
                match &mut self.right {
                    Some(right_node) => right_node.insert(value),
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                }
            }
            Ordering::Equal => {} // Value already exists
        }
    }

    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                match &self.left {
                    Some(left_node) => left_node.search(value),
                    None => false,
                }
            }
            Ordering::Greater => {
                match &self.right {
                    Some(right_node) => right_node.search(value),
                    None => false,
                }
            }
        }
    }

    fn in_order_traversal<'a>(&'a self, result: &mut Vec<&'a T>) {
        if let Some(left) = &self.left {
            left.in_order_traversal(result);
        }
        result.push(&self.value);
        if let Some(right) = &self.right {
            right.in_order_traversal(result);
        }
    }
}

pub fn demonstrate() {
    println!("Binary Search Tree: Hierarchical sorted structure\n");

    // Create a BST for equipment IDs
    println!("Building BST with equipment IDs:");
    let mut tree = TreeNode::new(50);
    
    let equipment_ids = vec![30, 70, 20, 40, 60, 80, 10, 25, 35, 65];
    
    for &id in &equipment_ids {
        tree.insert(id);
        println!("  Inserted: Equipment-{:03}", id);
    }

    // Search operations
    println!("\nSearch operations:");
    let search_ids = vec![25, 55, 80, 15];
    for &id in &search_ids {
        let found = tree.search(&id);
        println!("  Equipment-{:03}: {}", id, if found { "Found ✓" } else { "Not found ✗" });
    }

    // In-order traversal (gives sorted output)
    println!("\nIn-order traversal (sorted):");
    let mut sorted = Vec::new();
    tree.in_order_traversal(&mut sorted);
    print!("  ");
    for id in sorted {
        print!("Equipment-{:03} → ", id);
    }
    println!("End\n");

    // Example with sensor readings (in tenths of degrees)
    println!("--- Sensor Reading BST (temps in 0.1°C) ---");
    let mut temp_tree = TreeNode::new(255);

    let temps = vec![232, 281, 225, 268, 240, 295];
    println!("Recording sensor readings:");
    for &temp in &temps {
        temp_tree.insert(temp);
        println!("  Logged: {:.1}°C", temp as f32 / 10.0);
    }

    println!("\nSensor readings in sorted order:");
    let mut sorted_temps = Vec::new();
    temp_tree.in_order_traversal(&mut sorted_temps);
    for temp in sorted_temps {
        println!("  {:.1}°C", *temp as f32 / 10.0);
    }
}
