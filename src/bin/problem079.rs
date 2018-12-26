//! [Problem 79 (Passcode derivation)](https://projecteuler.net/problem=79)
//!
//! # Solution detail
//!
//! We are looking for an ordering of the digits that appear in any passcode attempt, such that
//! the order respects the relative order that the digits appear in any passcode attempt.
//!
//! This can be done by constructing a directed graph, with vertices corresponding to the digits,
//! and with a directed edge from `v` to `w` if digit `v` appears before digit `w` in any passcode
//! attempt.
//!
//! We then perform a topological sort on this graph, and create the passcode from the vertices in
//! order of the topological sort.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use petgraph::Graph;
use petgraph::algo::toposort;
use projecteuler_rs::problem;

/// Create a graph representing the given passcode attempts
fn attempt_graph(passcode_attempts: &[String]) -> Graph<u8, ()> {

    // Create a graph to hold the data
    let mut attempt_graph = Graph::<u8, ()>::new();
    let mut nodes = HashMap::new();

    // Procedure to get a node if it already exists, or create one if not
    macro_rules! node {
        ($v:expr) => {
            match nodes.entry($v) {
                Entry::Occupied(e) => *e.get(),
                Entry::Vacant(e) => *e.insert(attempt_graph.add_node($v)),
            }
        }
    }

    // Add edges to the graph for each attempt
    for attempt in passcode_attempts {
        let nodes: Vec<_> = attempt.bytes().map(|b| node!(b)).collect();
        for node_pair in (0..nodes.len()).combinations(2) {
            attempt_graph.add_edge(nodes[node_pair[0]], nodes[node_pair[1]], ());
        }
    }

    attempt_graph
}

/// Find the shortest passcode consistent with the given attempts
fn solve(passcode_attempts: &[String]) -> String {

    // Create a graph of attempts
    let attempt_graph = attempt_graph(passcode_attempts);

    // Perform a toposort on the attempst graph and return the result
    let sorted_vertices = toposort(&attempt_graph, None).unwrap().iter()
        .map(|&x| attempt_graph[x])
        .collect();

    String::from_utf8(sorted_vertices).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem079.txt")).unwrap();
    let reader = BufReader::new(file);
    let passcode_attempts: Vec<String> = reader.lines()
        .map(|line| line.unwrap())
        .collect();

    solve(&passcode_attempts)
}

problem!(answer, "73162890");
