use std::fs;

use phf::phf_map;

static NUM_CUBE: phf::Map<&'static str, i32> = phf_map! {
    "b" => 14,
    "g" => 13,
    "r" => 12,
};

// part 1
fn parse_game_info(game_info: &str) -> bool {
    game_info.split(";").all(|cube| {
        cube.split(",").all(|info| {
            let (num, color) = info.trim().split_once(" ").unwrap();
            let num = num.parse::<i32>().unwrap();
            let color = color.chars().nth(0).unwrap();
            num <= *NUM_CUBE.get(&color.to_string()).unwrap()
        })
    })
}

// part 1
fn get_game_num(game_num: &str) -> i32 {
    game_num
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

// part 2
fn power_min_cubes(game_info: &str) -> i32 {
    let min_values = game_info
        .split(";")
        .flat_map(|game| game.split(","))
        .filter_map(|info| {
            let (num, color) = info.trim().split_once(" ").unwrap();
            let num = num.parse::<i32>().unwrap();
            let color = color.chars().nth(0).unwrap();
            Some((num, color))
        })
        .fold(
            (0, 0, 0),
            |(min_red, min_green, min_blue), (num, color)| match color {
                'r' => (min_red.max(num), min_green, min_blue),
                'g' => (min_red, min_green.max(num), min_blue),
                'b' => (min_red, min_green, min_blue.max(num)),
                _ => (min_red, min_green, min_blue),
            },
        );

    min_values.0 * min_values.1 * min_values.2
}

fn parse_lines(line: &str) -> i32 {
    // split input into game title and game info
    let (_game_num, game_info) = line.split_once(":").unwrap();

    // Part 1
    //// If the game is valid, get the game number and return it
    //if parse_game_info(game_info) {
    //    return get_game_num(game_num);
    //}
    //// Otherwise, return 0
    //0

    power_min_cubes(game_info)
}

//fn get_input() -> &'static str {
//    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
//}

fn main() {
    //let games = get_input()
    let games = fs::read_to_string("inputs/p2.txt")
        .unwrap()
        .lines()
        .map(parse_lines)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();

    println!("games: {}", games);
}
