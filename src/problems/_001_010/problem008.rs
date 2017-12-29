//! [Problem 8 (Largest product in a series)](https://projecteuler.net/problem=8)
//!
//! # Solution detail
//!
//! Simply iterate over all groups of thirteen consecutive digits, calculating the products and
//! keeping track of the best.

#[macro_use]
extern crate projecteuler_rs;

/// Find the largest product of `n` consecutive digits in the given array of digits.
fn solve(digits: &[u8], n: usize) -> u64 {
    let mut best = 0;
    for idx in 0..digits.len() - n {
        let product = digits[idx..idx + n]
            .iter()
            .fold(1, |prod, &x| prod * x as u64);
        if product > best {
            best = product
        }
    }
    best
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let digits_str = "\
        73167176531330624919225119674426574742355349194934\
        96983520312774506326239578318016984801869478851843\
        85861560789112949495459501737958331952853208805511\
        12540698747158523863050715693290963295227443043557\
        66896648950445244523161731856403098711121722383113\
        62229893423380308135336276614282806444486645238749\
        30358907296290491560440772390713810515859307960866\
        70172427121883998797908792274921901699720888093776\
        65727333001053367881220235421809751254540594752243\
        52584907711670556013604839586446706324415722155397\
        53697817977846174064955149290862569321978468622482\
        83972241375657056057490261407972968652414535100474\
        82166370484403199890008895243450658541227588666881\
        16427171479924442928230863465674813919123162824586\
        17866458359124566529476545682848912883142607690042\
        24219022671055626321111109370544217506941658960408\
        07198403850962455444362981230987879927244284909188\
        84580156166097919133875499200524063689912560717606\
        05886116467109405077541002256983155200055935729725\
        71636269561882670428252483600823257530420752963450";

    let digits: Vec<u8> = digits_str.as_bytes()
        .iter()
        .map(|x| x - b'0')
        .collect();
    solve(&digits, 13).to_string()
}

problem!(answer, "23514624000");
