# 15-Minute City

The concept of the 15-Minute City has been a popular topic in literature since the outbreak of COVID-19 pandemic. It was first proposed by Moreno in 2016 as a solution to build safer, more resilient, sustainable and inclusive cities.

The thesis aims to develop and provide a general, adaptable algorithm to identify the 15-Minute City, where a person can travel to all their needs within 15 minutes. This repo contains the following:

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

### London

<!-- ![BraTS2021_00203](https://raw.githubusercontent.com/marcohoucheng/Brain-Tumor-Segmentation-with-Computer-Vision/main/Sample%20Gifs/BraTS2021_00203.gif) -->
### Padua
