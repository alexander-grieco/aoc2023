use std::fs;

use itertools::Itertools;

fn get_input() -> &'static str {
    "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
}

// part 1
fn get_next_item(nums: Vec<i64>) -> i64 {
    if nums.iter().all(|num| *num == 0) {
        return 0;
    }

    let differences: Vec<_> = nums.iter().tuple_windows().map(|(a, b)| b - a).collect();
    nums[nums.len() - 1] + get_next_item(differences)
}

// part 2
fn get_prev_item(nums: Vec<i64>) -> i64 {
    if nums.iter().all(|num| *num == 0) {
        return 0;
    }

    let differences: Vec<_> = nums.iter().tuple_windows().map(|(a, b)| b - a).collect();
    nums[0] - get_prev_item(differences)
}
fn main() {
    let binding = fs::read_to_string("inputs/p9.txt").unwrap();
    let ret = binding.lines().fold(0, |acc, line| {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        let interim = get_prev_item(nums);
        acc + interim
    });
    println!("{}", ret);
}
