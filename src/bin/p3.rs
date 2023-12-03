use num::abs;
use std::fs;

fn get_input() -> &'static str {
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
}

fn get_numbers(r_idx: i32, input: Vec<char>) -> Vec<(i32, i32, i32, i32)> {
    let mut numbers_with_index: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut start_idx = None;
    let mut end_idx = None;

    for (idx, &c) in input.iter().enumerate() {
        if c.is_digit(10) {
            start_idx = start_idx.or(Some(idx)); // Update start index if it's not set
            end_idx = Some(idx); // Update end index for the current number
        } else if let Some(start) = start_idx {
            if let Some(end) = end_idx {
                let num_string: String = input[start..=end].iter().collect();
                if let Ok(number) = num_string.parse::<i32>() {
                    let length = end - start + 1;
                    numbers_with_index.push((number, r_idx, start as i32, length as i32));
                }
            }
            start_idx = None; // Reset start index
            end_idx = None; // Reset end index
        }
    }

    // Check if the last character(s) form a number
    if let Some(start) = start_idx {
        if let Some(end) = end_idx {
            let num_string: String = input[start..=end].iter().collect();
            if let Ok(number) = num_string.parse::<i32>() {
                let length = end - start + 1;
                numbers_with_index.push((number, r_idx, start as i32, length as i32));
            }
        }
    }

    numbers_with_index
}

fn get_symbols(r_idx: i32, line: &str) -> Vec<(char, i32, i32)> {
    line.chars()
        .enumerate()
        .filter_map(|(c_idx, c)| {
            if c.is_numeric() {
                None
            } else if c == '.' {
                None
            } else {
                Some((c, r_idx as i32, c_idx as i32))
            }
        })
        .collect()
}

// part 1
fn get_sum(symbol_map: &Vec<(char, i32, i32)>, numbers_map: &Vec<(i32, i32, i32, i32)>) -> i32 {
    numbers_map
        .iter()
        .filter(|(_, r_idx, c_idx, length)| {
            symbol_map.iter().any(|(_, r_idx2, c_idx2)| {
                if abs(*r_idx - *r_idx2) <= 1
                    && *c_idx2 <= *c_idx + *length
                    && *c_idx2 >= *c_idx - 1
                {
                    return true;
                }
                false
            })
        })
        .fold(0, |sum, numbers_map| sum + numbers_map.0)
}

// part 2
fn get_gear_ratios(
    symbol_map: &Vec<(char, i32, i32)>,
    numbers_map: &Vec<(i32, i32, i32, i32)>,
) -> i32 {
    symbol_map
        .iter()
        .filter(|(symbol, _, _)| {
            if *symbol == '*' {
                return true;
            }
            false
        })
        .fold(0, |sum, symbol_map| {
            let mut product = 1;
            let num_length = numbers_map
                .iter()
                .filter(|(num, r_idx2, c_idx2, length)| {
                    if abs(symbol_map.1 - *r_idx2) <= 1
                        && (abs(*c_idx2 + *length - 1 - symbol_map.2) <= 1
                            || abs(*c_idx2 - symbol_map.2) <= 1)
                    {
                        product *= *num;
                        return true;
                    }
                    false
                })
                .collect::<Vec<_>>()
                .len();
            if num_length > 1 {
                return sum + product;
            }
            return sum;
        })
}

fn main() {
    //let grid = get_input().lines().enumerate();
    let binding = fs::read_to_string("inputs/p3.txt").unwrap();
    let grid = binding.lines().enumerate();
    let symbol_map: Vec<_> = grid
        .clone()
        .collect::<Vec<_>>()
        .iter()
        .map(|(idx, c)| get_symbols(*idx as i32, &c))
        .filter(|symbols| !symbols.is_empty())
        .flatten()
        .collect();

    let numbers_map = grid
        .map(|(line_num, line)| {
            let chars: Vec<char> = line.chars().collect();
            get_numbers(line_num as i32, chars)
        })
        .filter(|numbers| !numbers.is_empty())
        .flatten()
        .collect::<Vec<_>>();

    let sum = get_sum(&symbol_map, &numbers_map);
    let gear_sum = get_gear_ratios(&symbol_map, &numbers_map);

    println!("sum: {:?}", sum);
    println!("gear_sum: {:?}", gear_sum);
}
