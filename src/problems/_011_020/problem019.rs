//! [Problem 19 (Counting Sundays)](https://projecteuler.net/problem=19)
//!
//! # Solution detail
//!
//! Given that 1 Jan 1900 was a Monday, we can work out which day the following months began on by
//! keeping track of the current day, and advancing it according to how many days were in the
//! month which just passed, modulo 7. All this requires is being able to work out how many days
//! were in a given month in a given year.
//!
//! Then simply do this over a given range of months and count how many times the day at the start
//! of the month was Sunday.

#[macro_use]
extern crate projecteuler_rs;

/// Returns how many days were in the given month in the given year, where 0 -> January,
/// 1 -> February and so on.
fn days_in_month(month: usize, year: u64) -> u64 {
    // The number of days (usually) in each month.
    const DAYS: &'static [u64; 12] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // If February and a leap year, remember to add a day.
    if (month == 1) && (year % 4 == 0) && (year % 100 != 0 || (year / 100) % 4 == 0) {
        DAYS[month] + 1
    } else {
        DAYS[month]
    }
}

/// Find the number of months from 1901 to the given year which began on the given day, where
/// 0 -> Monday, 1 -> Tuesday and so on.
fn solve(day: u64, end_year: u64) -> u64 {
    assert!(1900 <= end_year);

    // 1 Jan 1901 was a Tuesday.
    let mut curr_day = 1;

    // For each month, check the current day and then advance it by the right amount.
    let mut ans = 0;
    for year in 1901..end_year + 1 {
        for month in 0..12 {
            if curr_day == day {
                ans += 1;
            }
            curr_day = (curr_day + days_in_month(month, year)) % 7;
        }
    }

    ans
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(6, 2_000).to_string()
}

problem!(answer, "171");
