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
use std::collections::HashMap;
use std::collections::HashSet;
use serde::Deserialize;
use petgraph::graphmap::UnGraphMap;
use priority_queue::PriorityQueue;

// https://docs.rs/rudac/latest/rudac/heap/struct.FibonacciHeap.html
// https://docs.rs/pheap/latest/pheap/

// Use 
// https://docs.rs/petgraph/latest/petgraph/ with UnGraphMap so node id can be used directly
// https://docs.rs/priority-queue/latest/priority_queue/

// Implement Dijkstra's algorithm

fn dijkstra(graph: UnGraphMap<u32, f32>, nodes_data: HashMap<u32, NodeData> , start: u32, threshold: f32, i: usize) {
    let mut distances: HashMap<u32, f32> = HashMap::new();
    let mut queue = PriorityQueue::new();

    // Initialize distances and Heap
    distances.insert(start, 0.0);
    queue.push(start, std::cmp::Reverse(0.0));
    while let Some((node, std::cmp::Reverse(distance))) = queue.pop() {
        if distance > threshold {
            break;
        }

        let mut node_data = nodes_data.get_mut(&node).unwrap();
        node_data.reach[i] = 1;

        for neighbor in graph.neighbors(node) {
            let edge_weight = graph.edge_weight(node, neighbor).unwrap();
            let potential_distance = distance + edge_weight;
            priority_queue.push_increase(neighbor, std::cmp::Reverse(potential_distance));
        }
    }

    // Initialize distances and Heap
    for node in 0..graph.node_count() {
        let distance = if node == start { 0.0 } else { f32::MAX };
        distances.insert(node, distance);
        queue.push(node, distance);
    }

    // Main loop of the algorithm
    while let Some((node, dist)) = queue.pop() {
        // If the current distance is greater than the threshold, break the loop
        if dist > threshold {
            break;
        }
        // Iterate over the outgoing edges and relax them
        for edge in graph.edges(node) {
            let next = edge.target(); // node_id u32
            let next_dist = distances[&node].saturating_add(*edge.weight());

            if next_dist < *distances.get(&next).unwrap_or(&f32::MAX) {
                let mut delta = next_dist - *distances.get(&next).unwrap_or(&f32::MAX);
                distances.insert(next, next_dist);
                queue.push(next, next_dist);
            }
        }
    }
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
    label: Option<String>, // Change to array later
}

struct NodeData {
    label: Vec<usize>,
    reach: Vec<usize>,
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

fn read_nodes_from_csv(file_path: &str) -> Result<(Vec<Node>, HashSet<String>, u32), Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut nodes = Vec::new();
    let mut unique_services: HashSet<String> = HashSet::new();
    let mut max_id = 0;

    for result in rdr.deserialize() {
        let node: Node = result?;

        let label_clone = node.label.clone();
        
        if node.id > max_id {
            max_id = node.id;
        }

        nodes.push(node);

        if let Some(label) = label_clone {
            unique_services.insert(label);
        }
    }
    Ok((nodes, unique_services, max_id))
}

fn create_graph(edges: Vec<Edge>, nodes: Vec<Node>, unique_services: &HashSet<String>) -> (UnGraphMap<u32, f32>, HashMap<u32, NodeData>){
    let mut graph = UnGraphMap::new();
    let service_list: Vec<String> = unique_services.iter().cloned().collect();
    let p: usize = service_list.len();
    let mut node_vecs = HashMap::new();

    for node in nodes {
        let mut label_vector: Vec<usize> = vec![0; p];
        let mut reach_vector: Vec<usize> = vec![0, p];
        
        if let Some(label) = node.label {
            if let Some(pos) = service_list.iter().position(|x| x == &label) {
                label_vector[pos] = 1;
            }
        }
        node_vecs.insert(node.id, NodeData { label: label_vector, reach: reach_vector });
        graph.add_node(node.id);
    }
    for edge in edges {
        graph.add_edge(edge.source, edge.target, edge.weight);
    }
    (graph, node_vecs)
}

fn write_to_csv(hashset: &HashSet<u32>, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a writer to write to a file
    let mut writer = Writer::from_writer(File::create(file_path)?);

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
    let (graph, nodes_data) = create_graph(edges, nodes, &unique_labels);

    let mut current_max_id: u32 = max_id;

    let p = unique_labels.len();

    for i in 0..(p-1){
        // Create a new node with an ID greater than the current maximum
        current_max_id += 1;
        let new_node_id = current_max_id;

        // Insert the new node into the graph
        graph.add_node(new_node_id);

        // Find all nodes associated with the current label
        let service_locations: HashSet<u32> = HashSet::new();
        for (id, node_data) in &nodes_data {
            let label_vector = &node_data.label;

            // Check if the label vector matches the current label
            if label_vector[i] == 1 {
                service_locations.insert(*id);
            }
        }

        for node_id in service_locations {
            graph.add_edge(new_node_id, node_id, 0);
        }
        
        // Search from each of these nodes with Dijkstra's algorithm and stops when w > 15 minutes
        dijkstra(graph, nodes_data, new_node_id, 15.0, i);
        // Remove the new node from the graph
        graph.remove_node(new_node_id);
    }

    let mut min_city: HashSet<u32> = HashSet::new();
    for (id, node_data) in &nodes_data {
        let reach_vector = &node_data.reach;
        let mut flag = true;
        for i in 0..(p-1){
            if reach_vector[i] == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            min_city.insert(*id);
        }
    }

    // Write the HashSet data to a CSV file
    write_to_csv(&min_city, "output.csv")?;

    Ok(())
}