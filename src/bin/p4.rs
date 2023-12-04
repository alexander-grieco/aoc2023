use std::fs;

fn get_input() -> &'static str {
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
}

fn parse_lines(idx: usize, game: &str, counts: &mut Vec<i32>) -> i32 {
    let (_, cards) = game.split_once(":").unwrap();

    let (winning_cards, my_cards) = cards.split_once("|").unwrap();

    let my_cards: Vec<_> = my_cards.split(" ").filter(|s| *s != "").collect();
    let winning_cards = winning_cards
        .split(" ")
        .filter(|s| *s != "")
        .filter(|card| my_cards.contains(card))
        .count();

    for i in 1..=winning_cards {
        counts[idx + i] += counts[idx];
    }
    counts[idx]
}

fn main() {
    //let binding = get_input();
    let binding = fs::read_to_string("inputs/p4.txt").unwrap();

    let mut card_count: Vec<i32> = Vec::with_capacity(binding.lines().clone().count());
    for _ in 0..binding.lines().clone().count() {
        card_count.push(1);
    }

    let sum = binding
        .lines()
        .enumerate()
        .map(|(idx, game)| parse_lines(idx, game, &mut card_count))
        .sum::<i32>();

    println!("sum: {:?}", sum);
}
