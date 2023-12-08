use num::Integer;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_input() -> &'static str {
    "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
}

fn parse_map(map: &str) -> (HashMap<&str, Vec<&str>>, HashSet<&str>) {
    let mut ret_map = HashMap::new();
    let mut start = HashSet::new();
    for line in map.lines() {
        let (key, value) = line.split_once(" = ").unwrap();

        // Collect all keys that end with A
        if key.ends_with('A') {
            start.insert(key);
        }
        let value = value
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .collect();
        ret_map.insert(key, value);
    }
    (ret_map, start)
}

fn main() {
    let binding = fs::read_to_string("inputs/p8.txt").unwrap();
    let (path, map) = binding.split_once("\n\n").unwrap();

    // get HashMap of the mappings and the list of starting nodes
    let (map, start) = parse_map(map);

    // set curr to list of all the starting nodes
    let mut curr = start.into_iter().collect::<Vec<_>>();

    // nums will represent the minimum number of steps to reach the end of each path
    let mut nums = vec![];

    // Loop over each starting node and determin how long it takes to reach a node ending in Z for
    // each starting point. Store that in value in nums
    (0..curr.len()).into_iter().for_each(|start_idx| {
        let mut idx = 1 as u64; // the current step
        let mut chars = path.chars().cycle(); // the path to loop over
        loop {
            let c = chars.next().unwrap();
            if c == 'L' {
                // go left
                curr[start_idx] = map.get(curr[start_idx]).unwrap()[0];
            } else {
                // go right
                curr[start_idx] = map.get(curr[start_idx]).unwrap()[1];
            }

            // If the node ends with Z, break out of the loop
            if curr[start_idx].ends_with('Z') {
                nums.push(idx);
                break;
            }

            // increment the step count
            idx += 1;
        }
    });

    // Calculate the LCM of all the numbers in nums. This is the answer
    let mut result = 1 as u64;
    for &num in nums.iter() {
        result = result.lcm(&num);
    }

    println!("Result: {:?}", result);
}
