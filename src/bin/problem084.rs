//! [Problem 84 (Monopoly odds)](https://projecteuler.net/problem=84)
//!
//! # Solution detail
//!
//! An approximate answer will do! The exact probabilities can be calculated using Markov chain
//! techniques, but since we are only interested in the relative order of the squares, not the exact
//! probabilities, some Monte Carlo simulation will suffice.
//!
//! That means that we just need to code up the rules of movement, simulate running around the board
//! for a large (10^6 or 10^7) number of turns, and count the squares that we landed on the most.

use projecteuler_rs::problem;
use std::collections::HashMap;
use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

struct MonopolyBoard<'a> {
    squares: Vec<&'a str>,
    square_indices: HashMap<&'a str, usize>,
}

impl <'a> MonopolyBoard<'a> {

    pub fn new() -> MonopolyBoard<'a> {
        let mut squares = Vec::new();
        let mut square_indices = HashMap::new();

        for &square in ["GO",  "A1", "CC1", "A2",  "T1", "R1", "B1",  "CH1", "B2",  "B3", "JAIL",
                        "C1",  "U1", "C2",  "C3",  "R2", "D1", "CC2", "D2",  "D3",  "FP", "E1",
                        "CH2", "E2", "E3",  "R3",  "F1", "F2", "U2",  "F3",  "G2J", "G1", "G2",
                        "CC3", "G3", "R4",  "CH3", "H1", "T2", "H2"].iter() {
            square_indices.insert(square, squares.len());
            squares.push(square);
        }

        MonopolyBoard {
            squares: squares,
            square_indices: square_indices,
        }
    }

    pub fn size(&self) -> usize {
        self.squares.len()
    }

    pub fn get_square_name(&self, position: usize) -> &str {
        self.squares[position]
    }

    pub fn get_position(&self, name: &str) -> usize {
        self.square_indices[name]
    }

    pub fn is_chance(&self, position: usize) -> bool {
        &self.squares[position][..2] == "CH"
    }

    pub fn is_community_chest(&self, position: usize) -> bool {
        &self.squares[position][..2] == "CC"
    }

    pub fn is_rail(&self, position: usize) -> bool {
        &self.squares[position][..1] == "R"
    }

    pub fn is_utility(&self, position: usize) -> bool {
        &self.squares[position][..1] == "U"
    }
}

struct Roller {
    rng: ThreadRng,
    range: Uniform<usize>,
}

impl Roller {

    pub fn new(range_size: usize) -> Roller {
        Roller {
            rng: thread_rng(),
            range: Uniform::new(1, range_size + 1),
        }
    }

    pub fn next_roll(&mut self) -> usize {
        self.range.sample(&mut self.rng)
    }
}

struct MonopolySimulator<'a> {
    board: MonopolyBoard<'a>,
    dice_roller: Roller,
    card_roller: Roller,
    doubles: usize,
    current_position: usize,
    counts: HashMap<usize, usize>,
}

#[derive(PartialEq, Eq)]
enum StepResult {
    Continue,
    Done,
}

impl <'a> MonopolySimulator<'a> {

    pub fn new(dice_size: usize) -> MonopolySimulator<'a> {
        MonopolySimulator {
            board: MonopolyBoard::new(),
            dice_roller: Roller::new(dice_size),
            card_roller: Roller::new(16),
            doubles: 0,
            current_position: 0,
            counts: HashMap::new(),
        }
    }

    /// Simulate the result of a single roll
    pub fn step(&mut self) {
        let roll = (self.dice_roller.next_roll(), self.dice_roller.next_roll());

        macro_rules! do_step {
            ($step:expr) => {
                if $step == StepResult::Done {
                    break;
                }
            }
        }

        for _ in 0..1 {
            do_step!(self.handle_doubles(roll));
            do_step!(self.handle_moving(roll));
            do_step!(self.handle_go_to_jail());
            do_step!(self.handle_chance());
            do_step!(self.handle_community_chest());
        }

        *self.counts.entry(self.current_position).or_insert(0) += 1;
    }

    pub fn squares_by_popularity(&self) -> Vec<usize> {
        let mut squares: Vec<_> = self.counts.keys().map(|x| *x).collect();
        squares.sort_by(|k1, k2| self.counts[k2].cmp(&self.counts[k1]));
        squares
    }

    /// Handle tracking doubles as part of a turn, and go to JAIL if necessary
    fn handle_doubles(&mut self, roll: (usize, usize)) -> StepResult {
        if roll.0 == roll.1 {
            self.doubles += 1;
        } else {
            self.doubles = 0;
        }

        if self.doubles == 3 {
            self.current_position = self.board.get_position("JAIL");
            self.doubles = 0;
            StepResult::Done
        } else {
            StepResult::Continue
        }
    }

    /// Handle actually moving the piece as a result of the roll
    fn handle_moving(&mut self, roll: (usize, usize)) -> StepResult {
        self.current_position = (self.current_position + roll.0 + roll.1) % self.board.size();
        StepResult::Continue
    }

    /// Handle landing on Go To Jail
    fn handle_go_to_jail(&mut self) -> StepResult {
        match self.board.get_square_name(self.current_position) {
            "G2J" => {
                self.current_position = self.board.get_position("JAIL");
                StepResult::Done
            },
            _ => StepResult::Continue,
        }
    }

    fn handle_chance(&mut self) -> StepResult {
        if self.board.is_chance(self.current_position) {
            match self.card_roller.next_roll() {
                1 => self.current_position = self.board.get_position("GO"),
                2 => self.current_position = self.board.get_position("JAIL"),
                3 => self.current_position = self.board.get_position("C1"),
                4 => self.current_position = self.board.get_position("E3"),
                5 => self.current_position = self.board.get_position("H2"),
                6 => self.current_position = self.board.get_position("R1"),
                7..=8 => while !self.board.is_rail(self.current_position) {
                    self.current_position = (self.current_position + 1) % self.board.size();
                },
                9 => while !self.board.is_utility(self.current_position) {
                    self.current_position = (self.current_position + 1) % self.board.size();
                },
                10 => self.current_position = (self.current_position - 3) % self.board.size(),
                _ => {},
            }
            StepResult::Done
        } else {
            StepResult::Continue
        }
    }

    fn handle_community_chest(&mut self) -> StepResult {
        if self.board.is_community_chest(self.current_position) {
            match self.card_roller.next_roll() {
                1 => self.current_position = self.board.get_position("GO"),
                2 => self.current_position = self.board.get_position("JAIL"),
                _ => {},
            }
            StepResult::Done
        } else {
            StepResult::Continue
        }
    }
}

/// Find the three most common squares in a simulation using a dice with the given number of sides.
fn solve(dice_size: usize, iterations: usize) -> Vec<usize> {
    let mut simulator = MonopolySimulator::new(dice_size);
    for _ in 0..iterations {
        simulator.step();
    }
    simulator.squares_by_popularity()[..3].to_vec()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(4, 1_000_000).iter().map(|x| x.to_string()).collect()
}

problem!(answer, "101524");
