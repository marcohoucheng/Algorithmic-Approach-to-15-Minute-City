// Read the 2 csv
// Find number of unique service types
// Add a node to connect to all locations for each service type
// Run Dijsktra from each of these nodes created
// - With Fibonacci Heap
// - Mark visited nodes (array of bool of length unique service types)
// - Stop after 15 minutes
// Return nodes where visited array is all true

use csv::Reader;
// use csv::Writer;
use std::time::{Instant, Duration};
use std::io::{self, Write};
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
use chrono::Local;
use indicatif::ProgressBar;

// https://docs.rs/petgraph/latest/petgraph/
// https://docs.rs/priority-queue/latest/priority_queue/

fn dijkstra(graph: &UnGraphMap<u64, f64>, nodes_data: &mut HashMap<u64, NodeData> , start: u64, threshold: f64, i: usize, debug_flag: bool) {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<u64, f64> = HashMap::new();

    heap.push((Reverse(NotNan::new(0.0).unwrap()), start));
    distances.insert(start, 0.0);
    // for node in graph.nodes() {
    //     distances.insert(node, f64::INFINITY);
    // }

    if debug_flag {
        println!("Source: {:?}", start);
    }
    
    while let Some(pop_node) = heap.pop() {
        let distance = pop_node.0.0.into_inner();
        let node = pop_node.1;

        if debug_flag {
            println!("Current node: {:?}, Current distance: {:?}", node, distance);
        }

        if distance > threshold {
            if debug_flag {
                println!("Threshold reached, break.");
            }
            break;
        }

        if node != start {
            let node_data = nodes_data.get_mut(&node).unwrap();
            if debug_flag {
                println!("Adding {:?} as visited.", node);
            }
            node_data.reach[i] = 1;
        }

        for edge in graph.edges(node) {
            let neighbor = edge.1;
            let weight = *edge.2;

            let new_distance = distance + weight;
            
            if debug_flag {
                println!("Neighbor: {:?}, Weight: {:?}, New distance: {:?}", neighbor, weight, new_distance);
            }

            if new_distance < *distances.get(&neighbor).unwrap_or(&f64::INFINITY) {
                if debug_flag {
                    println!("Updating distance for {:?} to {:?}", neighbor, new_distance);
                }
                distances.insert(neighbor, new_distance);
                // *distances.get_mut(&neighbor).unwrap() = new_distance;
                heap.push((Reverse(NotNan::new(new_distance).unwrap()), neighbor));
            }
        }
    }
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
    label: Option<String>,
}

#[derive(Clone)]
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
    let p: usize = service_list.len();
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

// fn write_to_csv(hashset: &HashSet<u64>, file_path: &str) -> Result<(), Box<dyn Error>> {
//     // Create a writer to write to a file
//     let mut writer = Writer::from_writer(File::create(file_path)?);

//     // Iterate over the HashSet and serialize each item
//     for record in hashset {
//         writer.serialize(record)?;
//     }

//     // Flush the writer to ensure all data is written to the file
//     writer.flush()?;
//     Ok(())
// }

fn write_to_csv(min_city: &HashSet<u64>, filename: &str) -> io::Result<()> {
    // Create a comma-separated string from the HashSet elements
    let csv_line = min_city.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(",");

    // Open the file for writing
    let mut file = File::create(filename)?;

    // Write the CSV line to the file
    file.write_all(csv_line.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: {} <f64_value> Optional(<u32_value>) Optional(<debug_flag>)", args[0]);
        std::process::exit(1);
    }

    let t: f64 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid f64 value provided.");
            std::process::exit(1);
        }
    };

    let repeat: u32 = if args.len() == 3 {
        match args[2].parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error: Invalid u32 value provided.");
                std::process::exit(1);
            }
        }
    } else {
        1
    };

    let debug_flag: bool = if args.len() == 4 {
        match args[3].parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error: Invalid debug flag provided.");
                std::process::exit(1);
            }
        }
    } else {
        false
    };

    if debug_flag {
        println!("Number of arguments: {:?}", args.len());
        println!("Threshold: {:?}", t);
        println!("Repeat: {:?}", repeat);
    }

    let edges = read_edges_from_csv("./data/edges.csv")?;
    let (nodes, unique_labels, max_id) = read_nodes_from_csv("./data/nodes.csv")?;
    let (mut graph, nodes_data_orginial) = create_graph(edges, nodes, &unique_labels);

    let mut total_duration = Duration::new(0, 0);
    
    let mut min_city: HashSet<u64> = HashSet::new();

    let p = unique_labels.len();

    println!("Number of unique service types: {:?}", p);
    println!("Number of nodes: {:?}", graph.node_count());
    println!("Number of edges: {:?}", graph.edge_count());

    let progress_bar = ProgressBar::new(repeat as u64);

    for _ in 0..repeat {

        let mut nodes_data = nodes_data_orginial.clone();

        let start_time = Instant::now(); // Start the timer

        let mut current_max_id: u64 = max_id;

        for i in 0..(p){
            // Create a new node with an ID greater than the current maximum
            current_max_id += 1;
            let new_node_id = current_max_id;
            
            if debug_flag {
                println!("New node ID: {:?}", new_node_id);
            }

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
            dijkstra(&graph, &mut nodes_data, new_node_id, t, i, debug_flag);
            // Remove the new node from the graph
            graph.remove_node(new_node_id);
        }

        min_city.clear();
        for (id, node_data) in nodes_data {
            let reach_vector = &node_data.reach;
            if reach_vector.iter().sum::<usize>() == p {
                min_city.insert(id.clone());
            }
        }

        let end_time = Instant::now();
        let duration = end_time - start_time;

        total_duration += duration;

        // Indicate that a step has been completed.
        progress_bar.inc(1);
    }

    progress_bar.finish();

    let excluded_iterations = if repeat * 5 / 100 > 10 { 10 } else { repeat * 5 / 100 };
    let adjusted_repeat = repeat - excluded_iterations;
    let adjusted_total_duration = total_duration - (total_duration / repeat * excluded_iterations);

    if adjusted_repeat > 1 {
        println!("Average execution time excluding the first {} iterations: {:?}", excluded_iterations, adjusted_total_duration / adjusted_repeat);
    } else {
        println!("Execution time: {:?}", total_duration);
    }
    // if repeat > 1 {
    //     println!("Average execution time: {:?}", total_duration / repeat);
    // } else {
    //     println!("Execution time: {:?}", total_duration);
    // }

    if debug_flag {
        println!("t-Minute City: {:?}", min_city);
    }

    // Get the current date and time
    let now = Local::now();
    
    // Format the date and time to use in the filename
    let formatted_now = now.format("%Y%m%d_%H%M%S").to_string();
    
    // Create the filename with the current date and time
    let file_name = format!("output_{}.csv", formatted_now);

    // Write the HashSet data to a CSV file
    write_to_csv(&min_city, &file_name)?;

    Ok(())
}