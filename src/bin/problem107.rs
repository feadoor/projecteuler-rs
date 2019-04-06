//! [Problem 107 (Minimal network)](https://projecteuler.net/problem=107)
//!
//! # Solution detail
//!
//! This is a simple problem in graph theory. We are simply asked to find a minimum spanning tree
//! to the graph, which can be done, for example, using
//! [Kruskal's algorithm](https://en.wikipedia.org/wiki/Kruskal%27s_algorithm)

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use petgraph::{Graph, Undirected};
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;

use projecteuler_rs::problem;

/// Create a graph representing the given adjacency matrix
fn network_graph(adjacency_list: &[Vec<Option<usize>>]) -> Graph<usize, usize, Undirected> {

    // Create an empty graph
    let size = adjacency_list.len();
    let mut network_graph = Graph::<usize, usize, Undirected>::with_capacity(size, size * size);

    // Add nodes to the graph for each entry in the list
    let nodes: Vec<_> = (0..size).map(|n| network_graph.add_node(n)).collect();

    // Add edges to the graph according to the adjacency matrix
    for (ix, row) in adjacency_list.iter().enumerate() {
        for (jx, edge) in row.iter().enumerate() {
            if ix < jx {
                if let Some(weight) = edge {
                    network_graph.add_edge(nodes[ix], nodes[jx], *weight);
                }
            }
        }
    }

    network_graph
}

/// Find the largest possible saving that can be achieved by removing weighted edges from the graph
/// given by the specified adjacency list without sacrificing connectedness.
fn solve(adjacency_list: &[Vec<Option<usize>>]) -> usize {
    let graph = network_graph(adjacency_list);
    let total_weight: usize = graph.edge_references().map(|e| e.weight()).sum();

    let spanning_tree = Graph::<usize, usize, Undirected>::from_elements(min_spanning_tree(&graph));
    let spanning_tree_weight: usize = spanning_tree.edge_references().map(|e| e.weight()).sum();

    total_weight - spanning_tree_weight
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem107.txt")).unwrap();
    let reader = BufReader::new(file);
    let adjacency_list: Vec<_> = reader.lines()
        .map(|line| line.unwrap().split(',').map(|x| x.parse::<usize>().ok()).collect::<Vec<_>>())
        .collect();

    solve(&adjacency_list).to_string()
}

problem!(answer, "259679");
