use std::fs;

fn str_to_vals(line: &str) -> u128 {
    line.trim()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u128>()
        .unwrap()
}

fn main() {
    let binding = fs::read_to_string("inputs/p6.txt").unwrap();
    let (time, dist) = binding.split_once("\n").unwrap();

    let time = str_to_vals(time);
    let dist = str_to_vals(dist);

    let first = (0..time)
        .into_iter()
        .find(|hold_time| (time - hold_time) * hold_time > dist)
        .unwrap();
    let last = (0..time)
        .into_iter()
        .rev()
        .find(|hold_time| (time - hold_time) * hold_time > dist)
        .unwrap();
    println!("ret: {:?}", last - first + 1);
}
