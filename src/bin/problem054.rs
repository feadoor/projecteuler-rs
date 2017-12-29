//! [Problem 54 (Poker hands)](https://projecteuler.net/problem=54)
//!
//! # Solution Detail
//!
//! There are no tricks, here; we must simply parse each pair of hands, calculate their poker value
//! and check for a winner.
//!
//! To ease comparison between two hands, they can be converted into a canonical form -  a
//! structure consisting of:
//!
//! - The type of poker hand (Straight flush, Four of a kind, Full house...)
//! - The distinct ranks of cards in the hand, sorted first by count, then by value
//!
//! Then lexicographic ordering of these structures corresponds to ordering of poker hands.

#[macro_use]
extern crate projecteuler_rs;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

/// The suits that a playing card can belong to.
#[derive(PartialEq, Eq, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    /// Convert a byte from the set 'HDCS' into a suit.
    fn from_byte(suit_byte: u8) -> Result<Suit, String> {
        match suit_byte {
            b'H' => Ok(Suit::Hearts),
            b'D' => Ok(Suit::Diamonds),
            b'C' => Ok(Suit::Clubs),
            b'S' => Ok(Suit::Spades),
            _ => Err("Invalid byte for suit".to_string()),
        }
    }
}

/// The ranks that a playing car can hold.
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
enum Rank {
    AceLow = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Rank {
    /// Convert a byte from the set '23456789TJQKA' into a rank.
    fn from_byte(rank_byte: u8) -> Result<Rank, String> {
        match rank_byte {
            b'2' => Ok(Rank::Two),
            b'3' => Ok(Rank::Three),
            b'4' => Ok(Rank::Four),
            b'5' => Ok(Rank::Five),
            b'6' => Ok(Rank::Six),
            b'7' => Ok(Rank::Seven),
            b'8' => Ok(Rank::Eight),
            b'9' => Ok(Rank::Nine),
            b'T' => Ok(Rank::Ten),
            b'J' => Ok(Rank::Jack),
            b'Q' => Ok(Rank::Queen),
            b'K' => Ok(Rank::King),
            b'A' => Ok(Rank::Ace),
            _ => Err("Invalid byte for rank".to_string()),
        }
    }
}

/// A structure holding a single playing card.
#[derive(PartialEq, Eq, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Get the rank of this card.
    fn get_rank(&self) -> Rank {
        self.rank
    }

    /// Get the suit of thi card.
    fn get_suit(&self) -> Suit {
        self.suit
    }
}

impl FromStr for Card {
    type Err = String;

    /// Get a `Card` from a string such as '8H', 'TD' or 'AS'.
    fn from_str(card_str: &str) -> Result<Card, String> {
        let bytes = card_str.as_bytes();
        if bytes.len() != 2 {
            Err("A card string must consist of exactly two bytes".to_string())
        } else {
            Ok(Card {
                rank: Rank::from_byte(bytes[0])?,
                suit: Suit::from_byte(bytes[1])?,
            })
        }
    }
}

/// The different classes of poker hand that are available.
#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum PokerHandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

/// A structure representing a poker hand in a canonical form that means two hands can be
/// compared easily.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct PokerHand {
    hand_type: PokerHandType,
    distinct_ranks: Vec<Rank>,
}

impl PokerHand {
    /// Convert an array of 5 cards into a `PokerHand`.
    fn from_cards(cards: &[Card]) -> Result<PokerHand, String> {
        // Check that we have the expected number of cards.
        if cards.len() != 5 {
            Err("A poker hand must consist of exactly five cards".to_string())
        } else {
            // Get the sorted ranks of the given cards, useful for checking for a straight.
            let mut sorted_ranks: Vec<_> = cards.iter().map(|card| card.get_rank()).collect();
            sorted_ranks.sort();
            if sorted_ranks.as_slice() ==
               [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace] {
                sorted_ranks = vec![Rank::AceLow, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
            }

            // Determine if we have a flush and/or a straight.
            let is_flush = cards.iter().all(|card| card.get_suit() == cards[0].get_suit());
            let is_straight = sorted_ranks.iter()
                .enumerate()
                .all(|(ix, rank)| *rank as usize == sorted_ranks[0] as usize + ix);

            // Get a count for how many times each card appears in this hand.
            let mut counter: HashMap<Rank, usize> = HashMap::new();
            for rank in sorted_ranks {
                *counter.entry(rank).or_insert(0) += 1;
            }

            // Get the distinct ranks of the cards, sorted first by how many times that rank
            // appears, then by the value of the rank.
            let mut distinct_ranks: Vec<_> = counter.keys().map(|key| (*key).clone()).collect();
            distinct_ranks.sort_by(|a, b| {
                match counter.get(b).unwrap().cmp(counter.get(a).unwrap()) {
                    Ordering::Equal => b.cmp(a),
                    other => other,
                }
            });

            // Get the counts of the distinct ranks, sorted in reverse order.
            let mut counts: Vec<_> = counter.values().collect();
            counts.sort_by(|a, b| b.cmp(a));

            // Find the best poker hand type that fits the given cards.
            let hand_type = if is_flush && is_straight {
                PokerHandType::StraightFlush
            } else if counts.as_slice() == [&4, &1] {
                PokerHandType::FourOfAKind
            } else if counts.as_slice() == [&3, &2] {
                PokerHandType::FullHouse
            } else if is_flush {
                PokerHandType::Flush
            } else if is_straight {
                PokerHandType::Straight
            } else if counts.as_slice() == [&3, &1, &1] {
                PokerHandType::ThreeOfAKind
            } else if counts.as_slice() == [&2, &2, &1] {
                PokerHandType::TwoPair
            } else if counts.as_slice() == [&2, &1, &1, &1] {
                PokerHandType::Pair
            } else {
                PokerHandType::HighCard
            };

            Ok(PokerHand {
                hand_type: hand_type,
                distinct_ranks: distinct_ranks,
            })
        }
    }
}

/// Find the number of games of poker from the given hands that were won by the first player.
fn solve<I>(deals: I) -> Result<usize, String>
    where I: Iterator<Item = Vec<Card>>
{
    let mut count = 0;
    for cards in deals {
        let hand1 = PokerHand::from_cards(&cards[..5])?;
        let hand2 = PokerHand::from_cards(&cards[5..])?;
        if hand1 > hand2 {
            count += 1;
        }
    }
    Ok(count)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem054.txt")).unwrap();
    let reader = BufReader::new(file);
    let deals = reader.lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(' ').map(|card_str| Card::from_str(card_str).unwrap()).collect());

    solve(deals).unwrap().to_string()
}

problem!(answer, "376");
