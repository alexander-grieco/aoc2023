use std::fs;

use aoc2023::camel::cards::{Card, Hand, HandRank};

fn get_input() -> &'static str {
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
}

fn determine_hand_rank(hand: &str) -> HandRank {
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();
    let mut counts = [0; 128];
    let mut jokers = 0;
    for c in chars {
        match c {
            'J' => jokers += 1,
            _ => counts[c as usize] += 1,
        }
    }
    let mut final_counts: Vec<usize> = counts.into_iter().filter(|&c| c != 0).collect();
    final_counts.sort();
    if jokers == 5 {
        return HandRank::FiveOfAKind;
    }
    if final_counts.len() != 0 {
        let idx = final_counts.len() - 1;
        final_counts.get_mut(idx).map(|c| *c += jokers);
    }
    match final_counts.as_slice() {
        [5] => HandRank::FiveOfAKind,
        [1, 4] => HandRank::FourOfAKind,
        [2, 3] => HandRank::FullHouse,
        [1, 1, 3] => HandRank::ThreeOfAKind,
        [1, 2, 2] => HandRank::TwoPair,
        [1, 1, 1, 2] => HandRank::Pair,
        _ => HandRank::HighCard,
    }
}

fn parse_hand(input: &str) -> Hand {
    let mut cards = [Card::Value('0'); 5];
    for (i, c) in input.chars().enumerate() {
        cards[i] = Card::Value(c);
    }
    let hand_rank = determine_hand_rank(input);
    Hand { hand_rank, cards }
}

fn main() {
    let binding = fs::read_to_string("inputs/p7.txt").unwrap();
    let mut input = binding
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_at(5);
            let bid = bid.trim().parse::<usize>().unwrap();
            let hand = parse_hand(hand);
            (hand, bid)
        })
        .collect::<Vec<_>>();
    input.sort_by_key(|&(hand, _)| hand);

    let tot = input.into_iter().enumerate().fold(0, |acc, (i, (_, bid))| {
        let score = (i + 1) * bid;
        acc + score
    });
    println!("{}", tot);
}
