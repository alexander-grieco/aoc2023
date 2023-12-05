use std::{collections::HashSet, fs};

fn get_input() -> &'static str {
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
}

fn get_seed_ranges(seed_vec: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut ret = Vec::new();
    for idx in (0..seed_vec.len()).step_by(2) {
        ret.push((seed_vec[idx], seed_vec[idx] + seed_vec[idx + 1]));
    }
    ret
}

fn get_seeds(seeds: &str) -> Vec<u64> {
    seeds
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn update_map(line: &str) -> Vec<u64> {
    line.trim()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn parse_maps(maps: &str) -> Vec<Vec<u64>> {
    maps.split_once("\n")
        .unwrap()
        .1
        .split("\n")
        .map(|line| update_map(line))
        .collect::<Vec<_>>()
}

fn main() {
    //let (seeds, maps) = get_input().split_once("\n\n").unwrap();
    let binding = fs::read_to_string("inputs/p5.txt").unwrap();
    let (seeds, maps) = binding.split_once("\n\n").unwrap();

    let seed_vec: Vec<_> = get_seed_ranges(&get_seeds(seeds))
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let mut map_vec: Vec<_> = maps.split("\n\n").map(parse_maps).collect::<Vec<_>>();
    map_vec.reverse();

    let min = (0..std::u64::MAX)
        .into_iter()
        .find(|loc| {
            let &(mut val) = loc;
            println!("loc: {}", loc);
            let starting_loc = map_vec.iter().fold(val, |_acc, mapped| {
                mapped.iter().any(|entry| {
                    let start = entry[0];
                    let end = entry[0] + entry[2];
                    if val >= start && val < end {
                        val = val + entry[1] - entry[0];
                        return true;
                    }
                    return false;
                });
                return val;
            });
            seed_vec.iter().any(|seed| {
                if starting_loc >= seed.0 && starting_loc < seed.1 {
                    return true;
                }
                return false;
            })
        })
        .unwrap();
    println!("min: {}", min);
}
