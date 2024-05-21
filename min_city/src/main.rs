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
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::Deserialize;
use petgraph::graphmap::UnGraphMap;

// https://docs.rs/rudac/latest/rudac/heap/struct.FibonacciHeap.html
// https://docs.rs/pheap/latest/pheap/

// Use 
// https://docs.rs/petgraph/latest/petgraph/ with UnGraphMap so node id can be used directly
// https://docs.rs/priority-queue/latest/priority_queue/

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: f32,
    position: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: UnGraphMap<u32, f32>, nodes_data: HashMap<u32, NodeData> , start: u32, threshold: f32, i: usize) {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<u32, f32> = HashMap::new();

    heap.push(State { cost: 0.0, position: start });
    distances.insert(start, 0.0);
    
    while let Some(State { cost, position }) = heap.pop() {
        if cost > threshold {
            break;
        }

        let node_data = nodes_data.get_mut(&position).unwrap();
        node_data.reach[i] = 1;

        for neighbor in graph.neighbors(position) {
            let edge_weight = graph.edge_weight(position, neighbor).unwrap();
            let next = State { cost: cost + edge_weight, position: neighbor };
            
            if next.cost < distances[&neighbor] {
                distances.insert(neighbor, next.cost);
                heap.push(next);
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
    let (mut graph, mut nodes_data) = create_graph(edges, nodes, &unique_labels);

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