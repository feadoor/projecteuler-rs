//! [Problem 43 (Sub-string divisibility)](https://projecteuler.net/problem=43)
//!
//! # Problem statement
//!
//! The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the
//! digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility
//! property.
//!
//! Let d<sub>1</sub> be the 1st digit, d<sub>2</sub> be the 2nd digit, and so on. In this way, we
//! note the following:
//!
//! <ul>
//! <li>d<sub>2</sub>d</sub>3</sub>d<sub>4</sub>=406 is divisible by 2</li>
//! <li>d<sub>3</sub>d</sub>4</sub>d<sub>5</sub>=063 is divisible by 3</li>
//! <li>d<sub>4</sub>d</sub>5</sub>d<sub>6</sub>=635 is divisible by 5</li>
//! <li>d<sub>5</sub>d</sub>6</sub>d<sub>7</sub>=357 is divisible by 7</li>
//! <li>d<sub>6</sub>d</sub>7</sub>d<sub>8</sub>=572 is divisible by 11</li>
//! <li>d<sub>7</sub>d</sub>8</sub>d<sub>9</sub>=728 is divisible by 13</li>
//! <li>d<sub>8</sub>d</sub>9</sub>d<sub>10</sub>=289 is divisible by 17</li>
//! </ul>
//!
//! Find the sum of all 0 to 9 pandigital numbers with this property.
//!
//! # Solution detail
//!
//! The easiest way to solve this problem is by using a depth-first search, building up the number
//! one digit at a time, and pruning branches whenever one of the divisibility conditions is not
//! satisfied.
//!
//! In other words, we consider all those choices for the first four digits which are divisible
//! by 2, then for each of those, consider all choices for the next digit which meets the condition
//! for divisibility by 3, and so on until the whole number has been built.

/// The name of the problem.
pub const NAME: &'static str = "Problem 43";
/// A description of the problem.
pub const DESC: &'static str = "Sub-string divisibility";

// A generic iterator over some u64s
type NumIter = Box<Iterator<Item = u64>>;

/// An iterator over the numbers which form solutions to the sub-string divisibility criterion.
fn solutions() -> NumIter {

    // An inner function which checks the most recently-applicable divisibility condition.
    fn check_condition(curr_num: u64, num_digits: u64) -> bool {
        const MODULI: &'static [u64; 7] = &[2, 3, 5, 7, 11, 13, 17];
        if num_digits >= 4 {
            (curr_num % 1000) % MODULI[(num_digits - 4) as usize] == 0
        } else {
            true
        }
    }

    // An inner function which recursively calls itself to perform a depth-first traversal of the
    // space of solutions. Chains together all solutions coming from each possible choice of next
    // digit.
    fn generator(curr_num: u64, digits_used: &mut [bool; 10], num_digits: u64) -> NumIter {
        if check_condition(curr_num, num_digits) {
            if num_digits == 10 {
                Box::new(Some(curr_num).into_iter())
            } else {
                let mut solutions = Box::new(None.into_iter()) as NumIter;
                for digit in 0..10 {
                    if !digits_used[digit] {
                        digits_used[digit] = true;
                        solutions = Box::new(solutions.chain(
                            generator(10 * curr_num + digit as u64,
                                      digits_used,
                                      num_digits + 1)));
                        digits_used[digit] = false;
                    }
                }
                solutions
            }
        } else {
            Box::new(None.into_iter())
        }
    }

    generator(0, &mut [false; 10], 0)
}

/// Find the sum of all 10-digit pandigital numbers satisfying the divisibility conditions.
fn solve() -> u64 {
    solutions().sum()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem043() {
        assert_eq!(super::answer(), "16695334890");
    }
}
