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
use std::fs::File;
use std::error::Error;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use serde::Deserialize;
use petgraph::graphmap::UnGraphMap;
use ordered_float::NotNan;
use std::env;

// https://docs.rs/rudac/latest/rudac/heap/struct.FibonacciHeap.html
// https://docs.rs/pheap/latest/pheap/

// Use 
// https://docs.rs/petgraph/latest/petgraph/ with UnGraphMap so node id can be used directly
// https://docs.rs/priority-queue/latest/priority_queue/

fn dijkstra(graph: &UnGraphMap<u64, f64>, nodes_data: &mut HashMap<u64, NodeData> , start: u64, threshold: f64, i: usize) {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<u64, f64> = HashMap::new();

    heap.push((Reverse(NotNan::new(0.0).unwrap()), start));
    distances.insert(start, 0.0);

    println!("Start: {}", start);

    let mut reached_solo = HashSet::new();
    
    while let Some(pop_node) = heap.pop() {
        let distance = pop_node.0.0.into_inner();
        println!("Popped Distance: {}", distance);
        let node = pop_node.1;
        println!("Start: {}, Node: {}, Distance: {}", start, node, distance);
        if distance > threshold {
            println!("Threshold reached");
            break;
        }
        if node != start {
            println!("Non start node");
            reached_solo.insert(node);
            let node_data = nodes_data.get_mut(&node).unwrap();
            node_data.reach[i] += 1;
            println!("Reachable: {:?}", node_data.reach);
        }

        for edge in graph.edges(node) {
            println!("Edge: {:?}", edge);
            let neighbor = edge.1;
            let weight = *edge.2;
            println!("Weight: {}", weight);

            let new_distance = distance + weight;

            if new_distance < *distances.get(&neighbor).unwrap_or(&f64::INFINITY) {
                distances.insert(neighbor, new_distance);
                heap.push((Reverse(NotNan::new(new_distance).unwrap()), neighbor));
            }
        }
    }
    println!("Reached Solo: {:?}", reached_solo);
}

#[derive(Debug, Deserialize)]
struct Edge {
    source: u64,
    target: u64,
    weight: f64,
}

#[derive(Debug, Deserialize)]
struct Node {
    id: u64,
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

fn read_nodes_from_csv(file_path: &str) -> Result<(Vec<Node>, HashSet<String>, u64), Box<dyn Error>> {
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

fn create_graph(edges: Vec<Edge>, nodes: Vec<Node>, unique_services: &HashSet<String>) -> (UnGraphMap<u64, f64>, HashMap<u64, NodeData>){
    let mut graph = UnGraphMap::new();
    let service_list: Vec<String> = unique_services.iter().cloned().collect();
    println!("Service List: {:?}", service_list);
    let p: usize = service_list.len();
    println!("Value of p: {}", p);
    let mut node_vecs = HashMap::new();

    for node in nodes {
        let mut label_vector: Vec<usize> = vec![0; p];
        let reach_vector: Vec<usize> = vec![0; p];
        
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

fn write_to_csv(hashset: &HashSet<u64>, file_path: &str) -> Result<(), Box<dyn Error>> {
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
    env::set_var("RUST_BACKTRACE", "1");
    // let args: Vec<String> = env::args().collect();
    // let edge_csv = &args[1];
    // let node_csv = &args[2];

    let edges = read_edges_from_csv("./data/edges.csv")?;
    let (nodes, unique_labels, max_id) = read_nodes_from_csv("./data/nodes.csv")?;
    let (mut graph, mut nodes_data) = create_graph(edges, nodes, &unique_labels);

    let mut current_max_id: u64 = max_id;

    let p = unique_labels.len();

    for i in 0..(p){
        println!("Service Type: {}", i);
        // Create a new node with an ID greater than the current maximum
        current_max_id += 1;
        let new_node_id = current_max_id;

        // Insert the new node into the graph
        graph.add_node(new_node_id);

        // Find all nodes associated with the current label
        let mut service_locations: HashSet<u64> = HashSet::new();
        for (id, node_data) in &nodes_data {
            let label_vector = &node_data.label;

            // Check if the label vector matches the current label
            if label_vector[i] == 1 {
                service_locations.insert(*id);
            }
        }

        for node_id in service_locations {
            graph.add_edge(new_node_id, node_id, 0.0);
        }
        
        // Search from each of these nodes with Dijkstra's algorithm and stops when w > 15 minutes
        dijkstra(&graph, &mut nodes_data, new_node_id, 15.0, i);
        // Remove the new node from the graph
        graph.remove_node(new_node_id);
    }

    let mut min_city: HashSet<u64> = HashSet::new();
    for (id, node_data) in nodes_data {
        let reach_vector = &node_data.reach;
        if reach_vector.iter().sum::<usize>() == p {
            min_city.insert(id.clone());
        }
        // let mut flag = true;
        // for i in 0..(p-1){
        //     if reach_vector[i] == 0 {
        //         flag = false;
        //         break;
        //     }
        // }
        // if flag {
        //     min_city.insert(id.clone());
        // }
    }

    println!("min_city: {:?}", min_city);
    // Write the HashSet data to a CSV file
    write_to_csv(&min_city, "output.csv")?;

    Ok(())
}