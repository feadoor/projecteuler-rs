//! [Problem 59 (XOR decryption)](https://projecteuler.net/problem=59)
//!
//! # Solution detail
//!
//! The key can be found quite easily, by using the fact that the most common character in the
//! plaintext will be, by far, the space, `0x20`.
//!
//! To find the key, split the ciphertext characters into three groups depending on which character
//! of the key they have been encrypted with. In each group, find the most common character, and
//! XOR it with space (`0x20`) to find the key character for that group.
//!
//! The rest of the problem is then simply a case of XORing each character with the key again and
//! finding the sum of the bytes.

use projecteuler_rs::problem;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

/// Find the sum of the bytes in a plaintext, given the ciphertext and the length of the key that
/// has been used to encrypt the text using and XOR cipher.
fn solve(ciphertext: &[u8], key_length: usize) -> u64 {

    // Find the frequency of each character, grouping bytes by which byte of the key they will
    // have been encrypted with.
    let mut frequencies = vec![[0; 256]; key_length];
    for (idx, &byte) in ciphertext.iter().enumerate() {
        frequencies[idx % key_length][byte as usize] += 1;
    }

    // Find the key by taking the most common character from each group and assuming that it
    // is actually a space in plaintext.
    let key: Vec<u8> = frequencies.iter()
        .map(|freqs| freqs.iter().enumerate().max_by_key(|&(_, item)| item).unwrap())
        .map(|(byte, _)| (byte ^ 0x20) as u8)
        .collect();

    // Decrypt the ciphertext and find the sum of bytes
    ciphertext.iter()
        .zip(key.iter().cycle())
        .map(|(c_byte, k_byte)| (c_byte ^ k_byte) as u64)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem059.txt")).unwrap();
    let reader = BufReader::new(file);
    let ciphertext: Vec<_> = reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().trim().parse::<u8>().unwrap())
        .collect();

    solve(&ciphertext, 3).to_string()
}

problem!(answer, "107359");
