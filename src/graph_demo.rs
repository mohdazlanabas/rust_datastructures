// GRAPH DATA STRUCTURE
// Collection of nodes (vertices) connected by edges
// Can be directed or undirected, weighted or unweighted
// Represented using adjacency list or adjacency matrix
// Use cases: networks, social connections, routing, dependencies

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    // Adjacency list representation: node -> list of connected nodes
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: String) {
        self.adjacency_list.entry(node).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        // For undirected graph, add edge in both directions
        self.adjacency_list
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        
        self.adjacency_list
            .entry(to)
            .or_insert(Vec::new())
            .push(from);
    }

    pub fn display(&self) {
        println!("Graph structure:");
        for (node, neighbors) in &self.adjacency_list {
            println!("  {} → {:?}", node, neighbors);
        }
    }

    pub fn bfs(&self, start: &str) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        println!("\nBreadth-First Search from '{}':", start);
        
        while let Some(node) = queue.pop_front() {
            print!("  {} → ", node);

            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        println!("End");
    }
}

pub fn demonstrate() {
    println!("Graph: Network of connected nodes (vertices and edges)\n");

    // Example: Process plant equipment network
    println!("Building process plant equipment network:");
    let mut plant_network = Graph::new();

    // Add equipment nodes
    let equipment = vec![
        "Reactor",
        "Heat Exchanger",
        "Pump-A",
        "Pump-B",
        "Storage Tank",
        "Control Valve",
        "Separator",
    ];

    for item in &equipment {
        plant_network.add_node(item.to_string());
        println!("  Added: {}", item);
    }

    // Define connections (process flow)
    println!("\nAdding connections (process flow):");
    let connections = vec![
        ("Storage Tank", "Pump-A"),
        ("Pump-A", "Heat Exchanger"),
        ("Heat Exchanger", "Reactor"),
        ("Reactor", "Separator"),
        ("Separator", "Control Valve"),
        ("Control Valve", "Pump-B"),
        ("Pump-B", "Storage Tank"),
        ("Heat Exchanger", "Control Valve"),
    ];

    for (from, to) in connections {
        plant_network.add_edge(from.to_string(), to.to_string());
        println!("  {} ↔ {}", from, to);
    }

    println!();
    plant_network.display();

    // Perform BFS to show reachability
    plant_network.bfs("Reactor");

    // Another example: Communication network
    println!("\n--- Sensor Communication Network ---");
    let mut sensor_network = Graph::new();

    sensor_network.add_node("Central Hub".to_string());
    sensor_network.add_node("Sensor-A".to_string());
    sensor_network.add_node("Sensor-B".to_string());
    sensor_network.add_node("Sensor-C".to_string());
    sensor_network.add_node("Sensor-D".to_string());

    sensor_network.add_edge("Central Hub".to_string(), "Sensor-A".to_string());
    sensor_network.add_edge("Central Hub".to_string(), "Sensor-B".to_string());
    sensor_network.add_edge("Sensor-A".to_string(), "Sensor-C".to_string());
    sensor_network.add_edge("Sensor-B".to_string(), "Sensor-D".to_string());

    println!();
    sensor_network.display();
    sensor_network.bfs("Central Hub");
}
