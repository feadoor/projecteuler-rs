//! [Problem 81 (Path sum: two ways)](https://projecteuler.net/problem=81)
//!
//! # Solution detail
//!
//! Since we are looking for a shortest path, this problem is a perfect candidate for Dijkstra's
//! algorithm.
//!
//! Construct a graph, with vertices the entries of the matrix, and edges the legal moves between
//! them (weighted appropriately) and simply run Dijkstra's algorithm to find the shortest path.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use petgraph::Graph;
use petgraph::algo::dijkstra;
use projecteuler_rs::problem;

/// Find the shortest path from the top-left to the bottom-right of the given grid, moving only
/// down and to the right.
fn solve(grid: &[Vec<u64>]) -> u64 {

    // Construct a graph with a node for each entry in the grid
    let mut graph = Graph::<(), u64>::new();
    let size = grid.len();
    let nodes: Vec<Vec<_>> = (0..size)
        .map(|_| (0..size).map(|_| graph.add_node(())).collect())
        .collect();

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

    // Return the length of the shortest path, remembering to add on the cost of the starting point
    let goal_node = nodes[size - 1][size - 1];
    let minimum_costs = dijkstra(&graph, nodes[0][0], Some(goal_node), |e| *e.weight());
    grid[0][0] + minimum_costs[&goal_node]
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let parse_row = |x: &str| -> Vec<u64> {
        x.split(',').map(|x| u64::from_str_radix(x, 10).unwrap()).collect()
    };

    let file = File::open(&Path::new("inputs/problem081.txt")).unwrap();
    let reader = BufReader::new(file);
    let grid: Vec<Vec<u64>> = reader.lines()
        .map(|line| parse_row(&line.unwrap()))
        .collect();

    solve(&grid).to_string()
}

problem!(answer, "427337");
