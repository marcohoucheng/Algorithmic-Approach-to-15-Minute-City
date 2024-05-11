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

fn export_to_csv(set: HashSet<String>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(File::create(file_path)?);

    for element in &set {
        writer.write_record(&[element])?;
    }

    writer.flush()?;
    Ok(())
}

fn dijkstra(graph: Graph<f32, f32>, start: NodeIndex, threshold: f32) -> HashMap<NodeIndex, f32> {
    let mut distances: HashMap<String, f64> = HashMap::new();
    let mut heap = FibonacciHeap::init_min();

    // Initialize distances and insert the start node into the Fibonacci Heap
    for node in graph.node_indices() {
        let distance = if node == start { 0.0 } else { f32::INFINITY };
        distances.insert(node, distance);
        fib_heap.push((distance, node));
    }

    // Main loop of the algorithm
    while let Some((dist, node)) = fib_heap.pop() {
        // If the current distance is greater than the threshold, break the loop
        if dist > threshold {
            break;
        }
        // Iterate over the outgoing edges and relax them
        for edge in graph.edges(node) {
            let next = edge.target();
            let next_dist = dist + *edge.weight();

            // Check if the next node's distance can be improved
            if next_dist < *distances.get(&next).unwrap_or(&f32::INFINITY) {
                // Update the distance and adjust the position in the Fibonacci Heap
                distances.insert(next, next_dist);
                fib_heap.decrease_key(&(next_dist, next)); // This will need a custom implementation
                // look at https://docs.rs/pheap/latest/pheap/ for a possible implementation
            }
        }
    }

    // Convert keys to a HashSet
    let distances_set: HashSet<&str> = distances.keys().cloned().collect();
    distances_set
}

fn main() {

    // time threshold
    let threshold = 15.0;

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

    let mut 15MC: HashSet<&str> = HashSet::new();

    for label in unique_labels.iter() {
        let mut services = Vec::new(); // nodes of the service 'label'

        // Identify all nodes with the service label
        for node_index in graph.node_indices() {
            if graph[node_index] == label {
                services.push(node_index);
            }
        }

        // Create a new node
        let new_node = graph.add_node(label); // node_index new_node

        // Connect to all nodes of the service
        for node_index in services {
            graph.add_edge(new_node, node_index, 0);
        }

        // Search from each of these nodes with Dijkstra's algorithm and stops when w > 15 minutes

        let tem_set = dijkstra(graph: &Graph, start: &str, )

        // Find intersection of all nodes visited

        let mut 15MC = 15MC.intersection(&tem_set)

    }

    let 15MC_vec: Vec<String> = 15MC.into_iter().collect();

    // Output the nodes

    export_to_csv(15MC_vec, "set_elements.csv")?;

    Ok(())

}
