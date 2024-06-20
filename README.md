# 15-Minute City

The 15-Minute City is an urban planning concept introduced in the last decade that promotes accessibility. It emphasizes that residents should be able to meet basic needs—such as groceries, education, healthcare, and leisure—within a 15-minute travel time from their homes. The concept aims to deliver environmental, social, and economic benefits by reducing reliance on automobiles, encouraging active transportation, and enhancing residents' quality of life through easy access to essential services and amenities. The concept has gained traction during the COVID-19 pandemic, which highlighted the significance of local services and amenities in urban settings.

The 15-Minute City concept has been explored across various research fields, including urban planning, transportation, and environmental science. Within the field of Computer Science, although methodologies have been developed for the topic, a generalised purpose algorithmic approach to identify a 15-Minute City is still lacking. Most existing studies are data-driven, focusing on specific cities with solutions that are often neither algorithmic nor generalised.

This thesis aims to develop a general, adaptive, and efficient algorithm to identify city areas that can be classified as a 15-Minute City. It examines several existing algorithms for graph data structures, such as Breadth-First Search, Dijkstra’s algorithm, Johnson’s algorithm, and their variations. The proposed 15-Minute City algorithm synthesises ideas and techniques from these algorithms to offer a comprehensive and efficient solution for determining 15-Minute City areas.

This repo contains the following:

1. Latest version of the thesis
2. Rust and Python implementations of the 15-Minute City algorithm.
3. Python notebooks to obtain map data from OpenStreetMap API and export the required csv files as a graph object.

## Rust Algorithm

The rust algorithm requires 2 input files, `nodes.csv` and `edges.csv`.

- `nodes.csv` contains 2 columns, `id` and `labels`. `id` are integer identifiers of the nodes and `labels` are strings can be empty.
- `edges.csv` contains all edges of the graph, with `source`, `target` and `weight`. `source` and `target` are integers corresponding to the integer identifiers and `weight` are of float `f64` type.

The algorithm returns the nodes id which belong to the $t$-Minute City, which is a set of nodes that can travel to at least one location of each label within $t$ minutes.

The algorithm has been tested on Rust version 1.79.

## Examples

The examples below visualise results of the 30-Minute City of London and Padua, where the streets with darker colour means one can travel to essential services with less time.

### London

![London 30-Minute City Heatmap](https://raw.githubusercontent.com/marcohoucheng/Algorithmic-Approach-to-the-15-Minute-City/main/images/London_30tMC.jpeg)

### Padua

![Padua 30-Minute City Heatmap](https://raw.githubusercontent.com/marcohoucheng/Algorithmic-Approach-to-the-15-Minute-City/main/images/Padua_30tMC.jpg)
