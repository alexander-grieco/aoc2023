use std::fs;

static NUMBER_MAPPING: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_lines(line: &str) -> i32 {
    let mut parsed_numbers = Vec::new();
    let mut current_number = String::new();

    // iterate over the line and check if the current char is a number or is a number word
    for c in line.chars() {
        current_number.push(c);

        let mut matched = false;

        // Sees if the word is an actual number word
        for (word, value) in NUMBER_MAPPING {
            if current_number.ends_with(word) {
                parsed_numbers.push(value);
                current_number.clear();
                matched = true;
                break;
            }
        }

        // If the character is not a part of a word, check if its a digit
        if !matched {
            if c.is_digit(10) {
                parsed_numbers.push(c.to_digit(10).unwrap() as i32);
                current_number.clear();
            }
        }
    }

    (parsed_numbers[0].to_string() + &parsed_numbers[parsed_numbers.len() - 1].to_string())
        .parse::<i32>()
        .unwrap()
}

fn main() {
    let sum = fs::read_to_string("inputs/p1.txt")
        .unwrap()
        .lines()
        .map(parse_lines)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();

    println!("sum: {:?}", sum);
}

