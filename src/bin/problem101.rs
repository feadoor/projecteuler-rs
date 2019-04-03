//! [Problem 101 (Optimum polynomial)](https://projecteuler.net/problem=101)
//!
//! # Solution detail
//!
//! For starters, if we interpolate based on the first 11-or-more terms of the sequence, then we
//! will obtain the exact sequence, so there are no FITs to be considered past the 11th term.
//!
//! So the problem reduces to, in turn, interpolating the sub-sequences consisting of the first
//! one, two, three..., ten terms, using a polynomial interpolation algorithm such as 
//! [Neville's Algorithm](https://en.wikipedia.org/wiki/Neville%27s_algorithm) to in each case find
//! the next term of the sequence.
//!
//! Then simply add together these terms and there's your answer!

use projecteuler_rs::problem;

/// Find the next term of the given sequence, assuming that the sequence is produced using the
/// polynomial `f` of smallest degree for which `f(1), f(2), ...` match the terms of the sequence.
/// Uses Neville's Algorithm.
fn interpolate_next_term(sequence: &[i64]) -> i64 {

	// `n` is the length of the sequence, and x is the point at which we are interpolating.
	let n = sequence.len() as i64; 
	let x = n + 1;

	// Start with the first row of the tableau
	let mut tableau = sequence.to_vec();

	// Iterate through the layers of the tableau computing each row from the previous
	for k in 1..n {
		let mut next_row = vec![];
		for i in 0..(n - k) as i64 {
			let j = i + k;
			let numer = (j + 1 - x) * tableau[i as usize] + (x - i - 1) * tableau[(i + 1) as usize];
			next_row.push(numer / k);
		}
		tableau = next_row;
	}

	// The final value in the tableau is the answer
	tableau[0]
}

/// Calculate the nth term of the sequence given in the problem.
fn nth_term(n: i64) -> i64 {
	let mut ans = 0; let mut worker = 1;
	for _ in 0..11 {
		ans += worker;
		worker *= -n;
	}
	ans
}

/// Find the sum of the incorrect interpolated terms of all lower degrees.
fn solve() -> i64 {
	let seq: Vec<_> = (1..11).map(|n| nth_term(n)).collect();
	(1..11).map(|n| &seq[..n]).map(|slice| interpolate_next_term(slice)).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "37076114526");
