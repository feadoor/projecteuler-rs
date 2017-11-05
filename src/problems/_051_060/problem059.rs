//! [Problem 59 (XOR decryption)](https://projecteuler.net/problem=59)
//!
//! # Problem statement
//!
//! Each character on a computer is assigned a unique code and the preferred standard is ASCII
//! (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk
//! (*) = 42, and lowercase k = 107.
//!
//! A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each
//! byte with a given value, taken from a secret key. The advantage with the XOR function is that
//! using the same encryption key on the cipher text, restores the plain text; for example, 65 XOR
//! 42 = 107, then 107 XOR 42 = 65.
//!
//! For unbreakable encryption, the key is the same length as the plain text message, and the key is
//! made up of random bytes. The user would keep the encrypted message and the encryption key in
//! different locations, and without both "halves", it is impossible to decrypt the message.
//!
//! Unfortunately, this method is impractical for most users, so the modified method is to use a
//! password as a key. If the password is shorter than the message, which is likely, the key is
//! repeated cyclically throughout the message. The balance for this method is using a sufficiently
//! long password key for security, but short enough to be memorable.
//!
//! Your task has been made easy, as the encryption key consists of three lower case characters.
//! Using cipher.txt (right click and 'Save Link/Target As...'), a file containing the encrypted
//! ASCII codes, and the knowledge that the plain text must contain common English words, decrypt
//! the message and find the sum of the ASCII values in the original text.
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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

/// The name of the problem.
pub const NAME: &'static str = "Problem 59";
/// A description of the problem.
pub const DESC: &'static str = "XOR decryption";

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
pub fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem059.txt")).unwrap();
    let reader = BufReader::new(file);
    let ciphertext: Vec<_> = reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().trim().parse::<u8>().unwrap())
        .collect();

    solve(&ciphertext, 3).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem059() {
        assert_eq!(super::answer(), "107359");
    }
}
