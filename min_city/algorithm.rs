// Read the 2 csv
// Find number of unique service types
// Add a node to connect to all locations for each service type
// Run Dijsktra from each of these nodes created
// - With Fibonacci Heap
// - Mark visited nodes (array of bool of length unique service types)
// - Stop after 15 minutes
// Return nodes where visited array is all true

use csv::Reader;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::Serialize;
use serde::Deserialize;
use pheap::PairingHeap;
use pheap::graph::simplegraph::SimpleGraph;

// https://docs.rs/rudac/latest/rudac/heap/struct.FibonacciHeap.html
// https://docs.rs/petgraph/latest/petgraph/
// https://docs.rs/pheap/latest/pheap/

// Implement Dijkstra's algorithm

fn dijkstra(graph: &SimpleGraph, start: NodeIndex, threshold: f32) -> HashSet<&str> {
    let mut distances: HashMap<u32, f32> = HashMap::new();
    let mut heap = PairingHeap::new();
    let mut min_city: HashSet<&str> = HashSet::new();

    // Initialize distances and Heap
    for node in 0..graph.num_nodes() {
        let distance = if node == start { 0 } else { f32::MAX };
        distances.insert(node, distance);
        heap.push((distance, node));
    }

    // Main loop of the algorithm
    while let Some((dist, node)) = heap.pop() {
        // If the current distance is greater than the threshold, break the loop
        if dist > threshold {
            break;
        }
        min_city.insert(node);
        // Iterate over the outgoing edges and relax them
        for edge in graph.edges_from(node) {
            let next = edge.to();
            let next_dist = distances[&node].saturating_add(edge.weight());

            if next_dist < *distances.get(&next).unwrap_or(&f32::MAX) {
                distances.insert(next, next_dist);
                heap.decrease_key(&(next_dist, next));
            }
        }
    }

    min_city
}

#[derive(Debug, Deserialize)]
struct Edge {
    source: u32,
    target: u32,
    weight: f32,
}

#[derive(Debug, Deserialize)]
struct Node {
    id: u32,
    label: Option<String>,
}

fn read_edges_from_csv(file_path: &str) -> Result<Vec<Edge>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut edges = Vec::new();
    for result in rdr.deserialize() {
        let edge: Edge = result?;
        edges.push(edge);
    }
    Ok(edges)
}

fn read_nodes_from_csv(file_path: &str) -> Result<(Vec<Node>, HashMap<String, Vec<u32>>), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut nodes = Vec::new();
    let mut labels = HashMap::new();

    let mut max_id = 0;

    for result in rdr.deserialize() {
        let node: Node = result?;
        if node.id > max_id {
            max_id = node.id;
        }
        nodes.push(node.clone());
        if let Some(label) = node.label {
            labels.entry(label).or_insert_with(Vec::new).push(node.id);
        }
    }
    Ok(nodes, labels, max_id)
}

fn create_graph(edges: Vec<Edge>, nodes: Vec<Node>) -> SimpleGraph<u32> {
    let mut graph = SimpleGraph::new();
    for node in nodes {
        graph.add_node(node.id);
    }
    for edge in edges {
        graph.add_edge(edge.source, edge.target, edge.weight);
    }
    graph
}

fn write_to_csv(hashset: &HashSet<u32>, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a writer to write to a file
    let mut writer = csv::Writer::from_writer(File::create(file_path)?);

    // Iterate over the HashSet and serialize each item
    for record in hashset {
        writer.serialize(record)?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let edges = read_edges_from_csv("edge.csv")?;
    let (nodes, unique_labels, max_id) = read_nodes_from_csv("nodes.csv")?;
    let graph = create_graph(edges, nodes);

    let mut min_city: HashSet<&u32> = nodes.iter().map(|node| node.id).collect();
    let mut current_max_id = max_id;

    for (label, node_ids) in labels.iter() {
        // Create a new node with an ID greater than the current maximum
        current_max_id += 1;
        let new_node_id = current_max_id;

        // Insert the new node into the graph
        graph.add_node(new_node_id);

        // Connect the new node to all nodes associated with the current label
        for &node_id in node_ids {
            graph.add_edge(new_node_id, node_id, 0);
        }

        // Search from each of these nodes with Dijkstra's algorithm and stops when w > 15 minutes

        let tem_set = dijkstra(graph, new_node_id, 15)

        // Find intersection of all nodes visited

        let mut min_city = min_city.intersection(&tem_set)
        // Remove the new node from the graph
        graph.remove_node(new_node_id);
    }

    // Write the HashSet data to a CSV file
    write_to_csv(&min_city, "output.csv")?;

    Ok(())
}