//! [Problem 82 (Path sum: three ways)](https://projecteuler.net/problem=82)
//!
//! # Solution detail
//!
//! Since we are looking for a shortest path, this problem is a perfect candidate for Dijkstra's
//! algorithm.
//!
//! Construct a graph, with vertices the entries of the matrix, and edges the legal moves between
//! them (weighted appropriately) and simply run Dijkstra's algorithm to find the shortest path.
//!
//! Since we are able to start anywhere in the left column, and end anywhere in the right column,
//! we will add two dummy nodes for the start and end - the start dummy can move into any cell in
//! the left column, and the end dummy can be hit from any cell in the right column.

#[macro_use]
extern crate projecteuler_rs;
extern crate petgraph;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use petgraph::Graph;
use petgraph::algo::dijkstra;

/// Find the shortest path from the left to the right of the given grid, moving only up, down and
/// to the right.
fn solve(grid: &[Vec<u64>]) -> u64 {

    // Construct a graph with a node for each entry in the grid
    let mut graph = Graph::<(), u64>::new();
    let size = grid.len();
    let nodes: Vec<Vec<_>> = (0..size)
        .map(|_| (0..size).map(|_| graph.add_node(())).collect())
        .collect();

    // Dummy start and end nodes
    let dummy_start = graph.add_node(());
    let dummy_end = graph.add_node(());

    // Add the dummy start edges
    for row in 0..size {
        graph.add_edge(dummy_start, nodes[row][0], grid[row][0]);
    }

    // Add the dummy end edges
    for row in 0..size {
        graph.add_edge(nodes[row][size - 1], dummy_end, 0);
    }

    // Add the edges for moving to the right
    for row in 0..size {
        for col in 0..size - 1 {
            graph.add_edge(nodes[row][col], nodes[row][col + 1], grid[row][col + 1]);
        }
    }

    // Add the edges for moving down
    for row in 0..size - 1 {
        for col in 0..size {
            graph.add_edge(nodes[row][col], nodes[row + 1][col], grid[row + 1][col]);
        }
    }

    // Add the edges for moving up
    for row in 1..size {
        for col in 0..size {
            graph.add_edge(nodes[row][col], nodes[row - 1][col], grid[row - 1][col]);
        }
    }

    // Return the length of the shortest path
    dijkstra(&graph, dummy_start, Some(dummy_end), |e| *e.weight())[&dummy_end]
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let parse_row = |x: &str| -> Vec<u64> {
        x.split(',').map(|x| u64::from_str_radix(x, 10).unwrap()).collect()
    };

    let file = File::open(&Path::new("inputs/problem082.txt")).unwrap();
    let reader = BufReader::new(file);
    let grid: Vec<Vec<u64>> = reader.lines()
        .map(|line| parse_row(&line.unwrap()))
        .collect();

    solve(&grid).to_string()
}

problem!(answer, "260324");
