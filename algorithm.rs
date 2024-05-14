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

fn dijkstra(graph: &SimpleGraph, start: NodeIndex, threshold: f32) -> HashMap<NodeIndex, f32> {
    let mut distances: HashMap<String, f32> = HashMap::new();
    let mut heap = PairingHeap::new();

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

    dist
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

fn read_nodes_from_csv(file_path: &str) -> Result<Vec<Node>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut nodes = Vec::new();
    for result in rdr.deserialize() {
        let node: Node = result?;
        nodes.push(node);
    }
    Ok(nodes)
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

fn export_to_csv(set: HashSet<String>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(File::create(file_path)?);

    for element in &set {
        writer.write_record(&[element])?;
    }

    writer.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct Record {
    field1: String,
    field2: u64,
}

fn write_hashset_to_csv(hashset: &HashSet<Record>) -> Result<(), Box<dyn Error>> {
    // Create a writer to write to a file
    let file = File::create("output.csv")?;
    let mut writer = Writer::from_writer(file);

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
    let nodes = read_nodes_from_csv("nodes.csv")?;
    let graph = create_graph(edges, nodes);

    let mut records = HashSet::new();
    // Write the HashSet data to a CSV file
    write_hashset_to_csv(&records)?;

    Ok(())
}