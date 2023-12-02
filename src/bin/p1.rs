use std::fs;

static NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_earliest_number_word_no_whitespace(text: &str) -> (Option<usize>, usize) {
    let mut min_index = text.len();
    let mut min_word = 0;

    for (idx, word) in NUMBER_WORDS.iter().enumerate() {
        if let Some(index) = text.find(word) {
            if index < min_index {
                min_index = index;
                min_word = idx + 1;
            }
        }
    }
    if min_index < text.len() {
        return (Some(min_index), min_word);
    }

    (None, 0)
}

fn find_latest_number_word_no_whitespace(text: &str) -> (Option<usize>, usize) {
    let mut max_index = 0;
    let mut max_word = 0;

    for (idx, word) in NUMBER_WORDS.iter().enumerate() {
        if let Some(index) = text.rfind(word) {
            if index > max_index {
                max_index = index;
                max_word = idx + 1;
            }
        }
    }
    if max_index > 0 {
        return (Some(max_index), max_word);
    }

    (None, 0)
}

fn parse_line(line: &str) -> i32 {
    let first_num_idx = line.find(char::is_numeric).unwrap_or(line.len());
    let second_num_idx = line.rfind(char::is_numeric).unwrap_or(0);
    let mut num_str = String::new();

    let (min_idx, min_word) = find_earliest_number_word_no_whitespace(line);
    let (max_idx, max_word) = find_latest_number_word_no_whitespace(line);

    if min_idx != None && min_idx.unwrap() < first_num_idx {
        num_str.push(min_word.to_string().chars().nth(0).unwrap());
    } else {
        num_str.push(line.chars().nth(first_num_idx).unwrap());
    }

    if max_idx != None && max_idx.unwrap() > second_num_idx {
        num_str.push(max_word.to_string().chars().nth(0).unwrap());
    } else {
        num_str.push(line.chars().nth(second_num_idx).unwrap());
    }

    num_str.parse::<i32>().unwrap()
}

fn main() {
    //let sum = get_input()
    let sum = fs::read_to_string("inputs/p1.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();

    println!("sum: {}", sum);
}
