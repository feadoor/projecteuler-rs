//! [Problem 98 (Anagramic squares)](https://projecteuler.net/problem=98)
//!
//! # Solution detail
//!
//! This is not a conceptually difficult problem, just one that requires a careful implementation.
//! The following process is one way of approaching it:
//!
//!   1. For each word in the list, sort its letters into increasing order and use that as a key
//!      into a `HashMap` to store this word (and all its anagrams).
//!   
//!   2. Cast out all keys that only contain one word (they have no anagrams, so are useless).
//!
//!   3. Do the same for the string representations of all square numbers having length at most
//!      the length of the longest anagram word.
//!
//!   4. For each key (in both maps) compute the 'signature' of the word - the counts of how many
//!      times each letter appears, sorted in decreasing order. Also store all the words and
//!      squares in maps keyed by their signatures.
//!
//!   5. For each word signature, and for each word key and square key having that signature, loop
//!      over the words and squares that have those keys. If a mapping is found between the letters
//!      of the word and the digits of the square, then for each word which is an anagram of the
//!      given word, turn it into a number using the same correspondence, and if that number is
//!      square, bingo!

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

use projecteuler_rs::problem;
use std::collections::HashMap;

/// Sort the letters of the given word into alphabetical order.
fn alphabetical_string(word: &str) -> String {
    let mut chars: Vec<_> = word.chars().collect();
    chars.sort();
    chars.iter().collect()
}

/// Get the signature of the given word.
fn signature(word: &str) -> Vec<usize> {
    let mut char_counts = HashMap::new();
    for ch in word.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    let mut sorted_counts: Vec<_> = char_counts.values().map(|x| *x).collect(); 
    sorted_counts.sort();
    sorted_counts
}

/// Find, if it exists, a mapping between the two given words, assumed to be of the same length
fn mapping(from: &str, to: &str) -> Option<HashMap<char, char>> {
    let mut result = HashMap::new();
    for (from_ch, to_ch) in from.chars().zip(to.chars()) {
        match result.get(&from_ch) {
            Some(&other) => if to_ch != other { return None; },
            None => { result.insert(from_ch, to_ch); },
        }
    }
    Some(result)
}

/// A struct that represents anagrams from a given set of words, keyed in two ways. First, by the
/// set of letters appearing in each word, and second, by the 'signature' of each word, where the
/// signature is the counts of the distinct letters in a word, in decreasing order.
struct SignaturedHashMaps {
    by_letters: HashMap<String, Vec<String>>,
    by_signature: HashMap<Vec<usize>, Vec<String>>,
}

impl SignaturedHashMaps {

    /// Create a new `SignaturedHashMap` representing the given words
    pub fn new(words: &[String]) -> SignaturedHashMaps {
        let mut result = SignaturedHashMaps { 
            by_letters: HashMap::new(), by_signature: 
            HashMap::new() 
        };
        for word in words {
            result.add_word(word);
        }
        result.remove_singletons();
        result.finalize();
        result
    }

    /// Find the longest length of any word contained in these maps
    pub fn longest_word(&self) -> Option<usize> {
        self.by_letters.keys().map(|k| k.len()).max()
    }

    /// Add a word to the internal by-letters map
    fn add_word(&mut self, word: &str) {
        let letters_key = alphabetical_string(word);
        self.by_letters.entry(letters_key).or_insert(Vec::new()).push(word.to_string());
    }

    /// Remove all keys that have no anagrams
    fn remove_singletons(&mut self) {
        let singleton_keys: Vec<_> = self.by_letters.keys()
            .filter(|&key| self.by_letters.get(key).unwrap().len() <= 1)
            .map(|x| x.clone())
            .collect();
        for key in singleton_keys.iter() {
            self.by_letters.remove(key);
        }
    }

    /// Add all words to the map keyed by signatures
    fn finalize(&mut self) {
        for word in self.by_letters.keys() {
            let signature_key = signature(word);
            self.by_signature.entry(signature_key).or_insert(Vec::new()).push(word.to_string());
        }
    }
}

/// Find the largest square number which is part of an anagramic square pair from the given words.
fn solve(words: &[String]) -> u64 {
    let words_maps = SignaturedHashMaps::new(words);
    let max_length = words_maps.longest_word().unwrap();

    let square_words: Vec<_> = (1..).map(|n| (n * n).to_string())
        .take_while(|s| s.len() <= max_length)
        .collect();
    let squares_maps = SignaturedHashMaps::new(&square_words);

    let mut answer = 0;

    // Loop over all pairs of signatures and all pairs of words/squares having those signatures
    for sig in words_maps.by_signature.keys() {
        for word_key in words_maps.by_signature.get(sig).unwrap() {
            for square_key in squares_maps.by_signature.get(sig).unwrap_or(&Vec::new()) {
                for word in words_maps.by_letters.get(word_key).unwrap() {
                    for square in squares_maps.by_letters.get(square_key).unwrap() {

                        // Attempt to find a mapping between the word and the square
                        if let Some(mapping) = mapping(word, square) {
                            for anag in words_maps.by_letters.get(word_key).unwrap() {

                                // For each anagram of the word, check if the same anagram of the
                                // square is also a square number
                                if anag != word {
                                    let other_square = anag.chars().map(|ch| mapping.get(&ch).unwrap()).collect();
                                    if squares_maps.by_letters.get(square_key).unwrap().contains(&other_square) {

                                        // Check if this square is larger than the best answer seen thus far
                                        let other_square_int = other_square.parse().unwrap();
                                        if other_square_int > answer { answer = other_square_int; }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    answer
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem098.txt")).unwrap();
    let reader = BufReader::new(file);
    let words: Vec<String> = reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().trim_matches('"').to_string())
        .collect();
    solve(&words).to_string()
}

problem!(answer, "18769");
