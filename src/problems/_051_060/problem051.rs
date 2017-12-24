//! [Problem 51 (Prime digit replacements)](https://projecteuler.net/problem=51)
//!
//! # Problem statement
//!
//! By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible
//! values: 13, 23, 43, 53, 73, and 83, are all prime.
//!
//! By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the
//! first example having seven primes among the ten generated numbers, yielding the family:
//!
//! 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
//!
//! Consequently 56003, being the first member of this family, is the smallest prime with this
//! property.
//!
//! Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits)
//! with the same digit, is part of an eight prime value family.
//!
//! # Solution detail
//!
//! This problem can be solved by generating the possible templates, in order of length, and then
//! checking, for each template, whether replacing the wildcards leads to an 8-prime family. We only
//! need to consider templates with 3, 6, 9... wildcards, since otherwise the number resulting from
//! the replacement would be divisible by 3 in at least three cases, meaning a 7-prime family is the
//! best we can do in this case.
//!
//! The templates of a particular length can be generated using a depth-first search - each template
//! is simply an ordered group of symbols coming from the digits 0-9 and a wildcard symbol.
//!
//! It is then just a case of checking all templates of a given length for an 8-prime family,
//! increasing the length until one is found.

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;
extern crate number_theory;
extern crate utils;

use primesieve::Sieve;
use number_theory::{integer_sqrt, pow};
use utils::search::{DepthFirstTree, Pruning};

/// A single symbol, either a digit or a wildcard, in a template string.
enum Symbol {
    /// A fixed digit.
    Digit(u64),
    /// A wildcard, which can be substituted later for any digit.
    Wildcard,
}

/// A structure representing a template.
#[derive(Clone)]
struct Template {
    /// The value that is represented only by the digits in non-wildcard positions, i.e. if all
    /// wildcards were set to 0.
    concrete_value: u64,
    /// The value that would result from setting all wildcards to 1 and all other digits to 0.
    wildcard_value: u64,
    /// The length of the template.
    length: usize,
    /// The number of wildcards in the template.
    wildcards: usize,
}

impl Template {
    /// A new, empty template consisting of zero symbols.
    fn empty() -> Template {
        Template {
            concrete_value: 0,
            wildcard_value: 0,
            length: 0,
            wildcards: 0,
        }
    }

    /// Add a symbol to the end of the template.
    fn add_symbol(&mut self, symbol: &Symbol) {
        self.concrete_value *= 10;
        self.wildcard_value *= 10;
        self.length += 1;
        match *symbol {
            Symbol::Digit(d) => self.concrete_value += d,
            Symbol::Wildcard => {
                self.wildcard_value += 1;
                self.wildcards += 1;
            }
        }
    }

    /// Remove a symbol from the end of the template.
    fn remove_symbol(&mut self, symbol: &Symbol) {
        self.concrete_value /= 10;
        self.wildcard_value /= 10;
        self.length -= 1;
        if let Symbol::Wildcard = *symbol {
            self.wildcards -= 1;
        }
    }

    /// Get the value which results from substituting the wildcard with the given digit.
    fn get_substitution(&self, digit: u64) -> u64 {
        self.concrete_value + digit * self.wildcard_value
    }

    /// Get all substitutions resulting from this template which are prime, in order.
    fn get_prime_substitutions(&self, sieve: &Sieve) -> Vec<u64> {
        let digits_to_try = if self.concrete_value > self.wildcard_value {
            0..10
        } else {
            1..10
        };
        digits_to_try.map(|d| self.get_substitution(d))
            .filter(|&x| sieve.is_prime(x).unwrap())
            .collect()
    }
}

/// A description of a step that can be taken in the search tree.
struct TemplateTreeStep {
    next_symbol: Symbol,
}

/// The information that is held about the current state during the tree search.
struct TemplateTree {
    template: Template,
    required_length: usize,
}

impl TemplateTree {
    /// Construct a new `TemplateTree` which will search for templates of the given length.
    fn with_required_length(length: usize) -> TemplateTree {
        TemplateTree {
            template: Template::empty(),
            required_length: length,
        }
    }
}

/// Generate all possible valid templates of a given length in a depth-first manner.
impl DepthFirstTree for TemplateTree {
    type Step = TemplateTreeStep;
    type Output = Template;

    /// All possible choices of the next symbol to put in the current template.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        let mut steps: Vec<_> =
            (0..10).map(|d| Self::Step { next_symbol: Symbol::Digit(d) }).collect();
        steps.push(Self::Step { next_symbol: Symbol::Wildcard });
        steps
    }

    /// Don't go any deeper in the tree than the required template length.
    fn should_prune(&mut self) -> Pruning {
        if self.template.length == self.required_length {
            Pruning::Below
        } else {
            Pruning::None
        }
    }

    /// Add the next symbol to the end of the current template.
    fn apply_step(&mut self, step: &Self::Step) {
        self.template.add_symbol(&step.next_symbol);
    }

    /// Remove the last symbol from the current template.
    fn revert_step(&mut self, step: &Self::Step) {
        self.template.remove_symbol(&step.next_symbol);
    }

    /// Output the current template, if it is of the right length.
    fn output(&mut self) -> Option<Self::Output> {
        if self.template.length == self.required_length {
            Some(self.template.clone())
        } else {
            None
        }
    }
}

/// Find the smallest member of an 8-prime digit replacement family
fn solve() -> u64 {
    // Try all lengths of template, from 4 upwards
    for length_to_try in 4.. {

        // Keep track of the smallest prime that is a member of an 8-prime family, and create a
        // sieve to test for primality.
        let mut smallest_prime: Option<u64> = None;
        let sieve_limit = integer_sqrt(pow(10, length_to_try));
        let sieve = Sieve::to_limit(sieve_limit);

        // Iterate through the possible templates with 3, 6, 9... wildcards
        for template in TemplateTree::with_required_length(length_to_try as usize).into_iter() {
            if template.wildcards > 0 && template.wildcards % 3 == 0 {

                // Check if we get an 8-prime family from this template.
                let primes = template.get_prime_substitutions(&sieve);
                if primes.len() >= 8 {
                    match smallest_prime {
                        None => smallest_prime = Some(primes[0]),
                        Some(x) if x > primes[0] => smallest_prime = Some(primes[0]),
                        _ => {}
                    }
                }
            }
        }

        if let Some(x) = smallest_prime {
            return x;
        }
    }

    unreachable!()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "121313");
