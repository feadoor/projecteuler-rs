//! [Problem 96 (Su Doku)](https://projecteuler.net/problem=96)
//!
//! # Solution detail
//!
//! Ignoring the boilerplate of reading in the puzzles from file, the meat of this problem is in
//! quickly finding the solution to a given Sudoku puzzle.
//!
//! Depth-first search is a prime candidate for this - given a puzzle to solve, fill in the empty
//! cells in order, branching on all digits that are allowed to go into that cell, and backtracking
//! when an impossibility is encountered.
//!
//! After solving all of the puzzles, adding up to get the solution is a trivial matter.

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use itertools::iproduct;
use projecteuler_rs::problem;
use search::{DepthFirstTree, Pruning};

type Grid = [[usize; 9]; 9];
type Coords = (usize, usize);

/// The state of the current puzzle during a depth-first solve
struct SudokuTree {
    /// The values currently held in the grid
    grid: Grid
}

/// A description of a step that can be taken in the search tree.
struct SudokuTreeStep {
    /// The coordinates of the cell to assign a value to
    cell: Coords,
    /// The value to assign to this cell
    value: usize,
}

impl SudokuTree {

    /// Construct a new `SudokuTree`, initialized with the given partial grid.
    fn new(grid: Grid) -> SudokuTree {
        SudokuTree { grid: grid }
    }

    /// Find the coordinates of the first empty cell in the current grid.
    fn empty_cell(&self) -> Option<Coords> {
        iproduct!(0..9, 0..9).find(|&(x, y)| self.grid[x][y] == 0)
    }

    /// Find all values that are allowed in the given cell.
    fn allowed_values(&self, cell: Coords) -> Vec<usize> {
        let mut allowed = [true; 10];
        for value in self.values_in_neighbourhood(cell) {
            allowed[value] = false;
        }
        (1..10).filter(|&n| allowed[n]).collect()
    }

    /// Find all values in the neighbourhood of the given cell
    fn values_in_neighbourhood(&self, cell: Coords) -> impl Iterator<Item = usize> + '_ {
        self.values_in_row(cell).chain(self.values_in_col(cell)).chain(self.values_in_region(cell))
    }

    /// Find all values that already exist in the row of the given cell
    fn values_in_row(&self, cell: Coords) -> impl Iterator<Item = usize> + '_ {
        let (x, _) = cell;
        (0..9).map(move |j| self.grid[x][j])
    }

    /// Find all values that already exist in the column of the given cell
    fn values_in_col(&self, cell: Coords) -> impl Iterator<Item = usize> + '_ {
        let (_, y) = cell;
        (0..9).map(move |i| self.grid[i][y])
    }

    /// Find all values that already exist in the region of the given cell
    fn values_in_region(&self, cell: Coords) -> impl Iterator<Item = usize> + '_ {
        let (x, y) = (3 * (cell.0 / 3), 3 * (cell.1 / 3));
        iproduct!(0..3, 0..3).map(move |(i, j)| self.grid[x + i][y + j])
    }
}

impl DepthFirstTree for SudokuTree {
    type Step = SudokuTreeStep;
    type Output = Grid;

    /// Return all possible choices for the next value to place in the grid.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        self.empty_cell()
            .map(|cell| self.allowed_values(cell).iter().map(|&value| SudokuTreeStep { cell, value }).collect())
            .unwrap_or(Vec::new())
    }

    /// Never prune this tree - backtracking will happen automatically when there are no available
    /// placements to be made.
    fn should_prune(&mut self) -> Pruning {
        Pruning::None
    }

    /// Add the next digit to the partially-solved grid
    fn apply_step(&mut self, step: &Self::Step) {
        let (x, y) = step.cell;
        self.grid[x][y] = step.value;
    }

    /// Remove the last digit from the sequence
    fn revert_step(&mut self, step: &Self::Step) {
        let (x, y) = step.cell;
        self.grid[x][y] = 0;
    }

    /// If the grid is solved, then output the solution
    fn output(&mut self) -> Option<Self::Output> {
        if self.empty_cell().is_none() {
            Some(self.grid.clone())
        } else {
            None
        }
    }
}

/// Find the solution to a single Sudoku puzzle
fn solve_sudoku(grid: &Grid) -> Option<Grid> {
    let tree = SudokuTree::new(grid.clone());
    tree.into_iter().next()
}

/// Solve all of the given Sudoku puzzles and sum the three-digit numbers found in the top-left
/// of each solved puzzle.
fn solve(grids: &[Grid]) -> usize {
    let solved_grids = grids.iter().map(|g| solve_sudoku(g).unwrap());
    solved_grids.map(|g| 100 * g[0][0] + 10 * g[0][1] + g[0][2]).sum()
}

fn read_grid<T: Read>(reader: &mut BufReader<T>) -> Option<Grid> {

    // Ignore a single line which contains the number of the puzzle
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap() == 0 {
        return None;
    }

    // Read the next nine lines and store the digits in a grid
    let mut grid = [[0; 9]; 9];
    for (x, line) in reader.lines().enumerate().take(9) {
        for (y, c) in line.unwrap().chars().enumerate().take(9) {
            grid[x][y] = c.to_digit(10).unwrap() as usize;
        }
    }

    Some(grid)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem096.txt")).unwrap();
    let mut reader = BufReader::new(file);

    let mut grids = Vec::new();
    while let Some(grid) = read_grid(&mut reader) {
        grids.push(grid);
    }

    solve(&grids).to_string()
}

problem!(answer, "24702");
