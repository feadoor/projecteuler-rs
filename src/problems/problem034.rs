//! [Problem 34 (Digit factorials)](https://projecteuler.net/problem=34)
//!
//! # Problem statement
//!
//! 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//!
//! Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//!
//! Note: as 1! = 1 and 2! = 2 are not sums they are not included.
//!
//! # Solution detail
//!
//! We can find the solutions to this problem using a backtracking-style algorithm, building up
//! the solutions from left-to-right one digit at a time. Note that since 8 × 9! has
//! only 7 digits, there cannot be any solutions with more than 7 digits. This means that our
//! search tree only has depth 7.
//!
//! We can also prune the search tree before reaching the bottom by calculating the maximum and
//! minimum values that can be obtained by by the sum of factorials after extending the number
//! to length 7, and seeing if the number itself can ever fall within that range.
//!
//! For example, no matter what three digits we add to the end of 211, the sum of factorials can
//! never be larger than 2! + 1! + 1! + 9! + 9! + 9! + 9!, which is equal to 1451524. On the
//! other hand, the number itself after adding four digits will always be at least 2110000, so
//! there cannot possibly be a solution.
//!
//! Finally, remember to subtract 3 at the end, since 1 and 2 are not counted as solutions.

/// The name of the problem.
pub const NAME: &'static str = "Problem 34";
/// A description of the problem.
pub const DESC: &'static str = "Digit factorials";

// A custom type for a generic iterator over some u64s
type SolIter = Box<Iterator<Item = u64>>;

// A constant, representing the length in digits of any solution.
const SOL_LEN: u64 = 7;

// The factorials of single digits.
const FACTORIAL: &'static [u64; 10] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

/// Check if the given number can possibly be extended to a solution.
fn may_be_extended(value: u64, factorial_sum: u64, length: u64) -> bool {

    // Calculate the maximum and minimum values taken by the factorial sum after extending to
    // SOL_LEN digits long.
    let min_factorial_sum = factorial_sum;
    let max_factorial_sum = factorial_sum + (SOL_LEN - length) * FACTORIAL[9];

    // Calculate the maximum and minimum values taken by the actual number after extending to
    // SOL_LEN digits long.
    let mut min_value = value;
    let mut max_value = value + 1;
    for _ in 0..SOL_LEN - length {
        min_value *= 10;
        max_value *= 10;
    }

    // Check if the two ranges overlap, otherwise there is definitely no solution.
    min_factorial_sum < max_value && max_factorial_sum >= min_value
}

/// Find all numbers which are equal to the sum of the factorials of their digits, and which can
/// be formed by appending digits to the given number.
fn extensions(curr_value: u64, curr_factorial_sum: u64, curr_length: u64) -> SolIter {

    // There are only extensions if we have used fewer than SOL_LEN digits so far.
    if curr_length < SOL_LEN && may_be_extended(curr_value, curr_factorial_sum, curr_length) {

        // A closure to find the solutions which come from a specific choice of next digit.
        let next_sols = |next_digit: u64| {
            let next_value = 10 * curr_value + next_digit;
            let next_factorial_sum = match next_value {
                0 => 0,
                _ => curr_factorial_sum + FACTORIAL[next_digit as usize],
            };
            let next_length = curr_length + 1;

            extensions(next_value, next_factorial_sum, next_length)
        };

        // Chain together all the solutions from each choice of next digit.
        (0..10).fold(Box::new(None.into_iter()),
                     |acc, x| Box::new(acc.chain(next_sols(x))))

    } else if curr_length == SOL_LEN && curr_value == curr_factorial_sum {
        Box::new(Some(curr_value).into_iter())
    } else {
        Box::new(None.into_iter())
    }
}

/// Find the sum of the numbers which are equal to the sum of the fifth powers of their digits.
fn solve() -> u64 {
    extensions(0, 0, 0).sum::<u64>() - 3
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem034() {
        assert_eq!(super::answer(), "40730");
    }
}
