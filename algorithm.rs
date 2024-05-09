// Read the 2 csv
// Find number of unique service types
// Add a node to connect to all locations for each service type
// Run Dijsktra from each of these nodes created
// - With Fibonacci Heap
// - Mark visited nodes (array of bool of length unique service types)
// - Stop after 15 minutes
// Return nodes where visited array is all true

use csv;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use rudac::heap::FibonacciHeap;
use petgraph::Graph;

// https://docs.rs/rudac/0.8.3/rudac/heap/struct.FibonacciHeap.html
// https://docs.rs/petgraph/0.6.5/petgraph/

// Implement Dijkstra's algorithm

fn dijkstra(graph: &Graph, start: &str) -> HashMap<String, f64> {
    let mut distances: HashMap<String, f64> = HashMap::new();
    let mut heap = FibonacciHeap::init_min();

    // Initialize distances and heap
    for (node_id, _) in &graph.nodes {
        distances.insert(node_id.clone(), f64::INFINITY);
        heap.push(f64::INFINITY, node_id.clone());
    }

    // Set the distance for the start node to 0
    distances.insert(start.to_string(), 0.0);
    heap.decrease_key(&start.to_string(), 0.0);

    while let Some((_, u)) = heap.pop() {
        if let Some(edges) = graph.edges.get(&u) {
            for (v, &weight) in edges {
                let alt = distances[&u] + weight;
                if alt < distances[v] {
                    distances.insert(v.clone(), alt);
                    heap.decrease_key(v, alt);
                }
            }
        }
    }

    distances
}

fn main() {

    // Load graph from CSV files

    let mut graph = Graph::<&str, u32>::new_undirected();

    // Hash Map to connect node_id (csv) to node_index (Rust)
    let mut node_indices = HashMap::new();

    // Read nodes.csv and add nodes to the graph
    let mut rdr = csv::Reader::from_path("nodes.csv")?;
    for result in rdr.records() {
        let record = result?;
        let node_id = &record[0];
        let label = &record[3];

        let node_index = graph.add_node(label);
        node_indices.insert(node_id.to_string(), node_index);
    }

    // Read edges.csv and add edges to the graph
    let mut rdr = csv::Reader::from_path("edges.csv")?;
    for result in rdr.records() {
        let record = result?;
        let source_id = &record[0];
        let target_id = &record[1];
        let weight: u32 = record[2].parse()?;
        
        // Get node_index from the correct node_id
        let source_index = *node_indices.get(source_id).expect("Source node not found");
        let target_index = *node_indices.get(target_id).expect("Target node not found");

        graph.add_edge(source_index, target_index, weight);
    }

    // We need a list of node_index for each service type

    // Collect unique labels
    let mut unique_labels: HashSet<&str> = HashSet::new();
    for node_index in graph.node_indices() {
        let label = graph[node_index];
        unique_labels.insert(label);
    }

    let label_to_find = "specific_label"; // Replace with the actual label
    let mut nodes_with_specific_label = Vec::new();

    for node_index in graph.node_indices() {
        if graph[node_index] == label_to_find {
            nodes_with_specific_label.push(node_index);
        }
    }

    // Then add a node connecting to each of these nodes

    // Search from each of these nodes with Dijkstra's algorithm and stops when w > 15 minutes

    // Find intersection of all nodes visited

    let mut 15MC: HashSet<&str> = HashSet::new();

}
