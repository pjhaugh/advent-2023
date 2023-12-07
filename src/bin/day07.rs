use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::iter::zip;

use anyhow::Result;
use itertools::Itertools;
use nom::Parser;

use crate::Score::{FiveOfAKind, FourOfAKind, FullHouse, High, Pair, ThreeOfAKind, TwoPair};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Score {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, Eq)]
struct Hand {
    score: Score,
    cards: Vec<u64>,
    bid: u64,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && zip(&self.cards, &other.cards).all(|(a, b)| a == b)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score != other.score {
            return Some(self.score.cmp(&other.score));
        }
        for (a, b) in zip(&self.cards, &other.cards) {
            if a < b {
                return Some(Ordering::Less);
            }
            if a > b {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }
}

fn get_score(cards: &Vec<u64>) -> Score {
    let mut map = HashMap::new();

    *map.entry(cards[0]).or_insert(0) += 1;
    *map.entry(cards[1]).or_insert(0) += 1;
    *map.entry(cards[2]).or_insert(0) += 1;
    *map.entry(cards[3]).or_insert(0) += 1;
    *map.entry(cards[4]).or_insert(0) += 1;
    if map.values().contains(&5) {
        FiveOfAKind
    } else if map.values().contains(&4) {
        FourOfAKind
    } else if map.values().contains(&3) && map.values().contains(&2) {
        FullHouse
    } else if map.values().contains(&3) {
        ThreeOfAKind
    } else if map.values().filter(|v| **v == 2).count() == 2 {
        TwoPair
    } else if map.values().contains(&2) {
        Pair
    } else {
        High
    }
}

fn value_hand(cards: &str, bid: &str) -> Result<Hand> {
    let cards = cards
        .chars()
        .filter_map(|c| match c {
            '2'..='9' => Some(c.to_string().parse().unwrap()),
            'T' => Some(10),
            'J' => Some(11),
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None,
        })
        .collect::<Vec<_>>();

    let score = get_score(&cards);

    Ok(Hand { score, cards, bid: bid.parse().unwrap() })
}

fn value_hand_2(cards: &str, bid: &str) -> Result<Hand> {
    let cards = cards
        .chars()
        .filter_map(|c| match c {
            '2'..='9' => Some(c.to_string().parse().unwrap()),
            'T' => Some(10),
            'J' => Some(1),
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut highest = High;

    for x in 2..=14 {
        let working_cards = cards.iter().map(|v| if *v == 1 { x } else { *v }).collect::<Vec<_>>();
        let score = get_score(&working_cards);
        highest = max(highest, score);
    }

    Ok(Hand { score: highest, cards, bid: bid.parse().unwrap() })
}


fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-07-2023.txt");
    // let input = include_str!("../../inputs/test-07.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn part_one(input: &'static str) -> Result<u64> {
    let hands = input.lines().map(|l| {
        let (cards, bid) = l.split_whitespace().take(2).collect_tuple().unwrap();
        value_hand(cards, bid).unwrap()
    }).sorted().collect::<Vec<_>>();
    Ok(hands.iter().enumerate().map(|(i, h)| (i + 1) as u64 * h.bid).sum())
}

fn part_two(input: &'static str) -> Result<u64> {
    let hands = input.lines().map(|l| {
        let (cards, bid) = l.split_whitespace().take(2).collect_tuple().unwrap();
        value_hand_2(cards, bid).unwrap()
    }).sorted().collect::<Vec<_>>();
    Ok(hands.iter().enumerate().map(|(i, h)| (i + 1) as u64 * h.bid).sum())
}
