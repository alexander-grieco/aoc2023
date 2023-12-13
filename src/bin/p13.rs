use std::fs;

use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

fn get_input() -> &'static str {
    "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
}

fn one_diff(line1: &str, line2: &str) -> i32 {
    let mut diff = 0;
    for (c1, c2) in line1.chars().zip(line2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }
    return diff;
}

fn find_vertical(grid: &str) -> (bool, usize) {
    let lines = grid
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if grid == "" {
        return (false, 0);
    }
    for idx in 0..(lines[0].len() - 1) as isize {
        let mut done = true;
        let col1: &str = &lines
            .iter()
            .map(|&s| &s[(idx as usize)..(idx + 1) as usize])
            .collect::<Vec<_>>()
            .join("");
        let col2: &str = &lines
            .iter()
            .map(|&s| &s[(idx + 1) as usize..(idx + 2) as usize])
            .collect::<Vec<_>>()
            .join("");
        let mut smudge = false;
        let dif = one_diff(col1, col2);
        if dif == 1 {
            smudge = true;
        }
        if dif <= 1 {
            let mut c1 = idx + 2;
            let mut c2 = idx * 2 + 1 - c1;
            while c2 >= 0 && c1 < lines[0].len() as isize {
                let col = &lines
                    .iter()
                    .map(|&s| &s[(c1 as usize)..(c1 + 1) as usize])
                    .collect::<Vec<_>>()
                    .join("");
                let col2 = &lines
                    .iter()
                    .map(|&s| &s[(c2 as usize)..(c2 + 1) as usize])
                    .collect::<Vec<_>>()
                    .join("");
                let dif = one_diff(col, col2);
                if dif > 1 || (dif == 1 && smudge) {
                    done = false;
                    break;
                } else if dif == 1 {
                    smudge = true;
                }
                c1 += 1;
                c2 -= 1;
            }
            if done && smudge {
                return (true, (idx + 1) as usize);
            }
        }
    }

    return (false, 0);
}

fn find_horizontal(grid: &str) -> (bool, usize) {
    let lines = grid.split("\n").collect::<Vec<_>>();
    for i in 0..(lines.len() - 1) as isize {
        let mut done = true;
        let dif = one_diff(lines[i as usize], lines[(i + 1) as usize]);
        let mut smudge = false;
        if dif == 1 {
            smudge = true;
        }
        if dif <= 1 {
            let mut l1 = (i + 2) as isize;
            let mut l2 = (i * 2 + 1 - l1) as isize;
            while l2 >= 0 && l1 < lines.len() as isize {
                let dif = one_diff(lines[l1 as usize], lines[l2 as usize]);
                if dif > 1 || (dif == 1 && smudge) {
                    done = false;
                    break;
                } else if dif == 1 {
                    smudge = true;
                }
                l1 += 1;
                l2 -= 1;
            }
            if done && smudge {
                return (true, (i + 1) as usize);
            }
        }
    }
    return (false, 0);
}

// part 1
fn hash_line(line: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    line.hash(&mut hasher);
    hasher.finish()
}

// part 1
fn line_cmp(line1: &str, line2: &str) -> bool {
    let hash_value_line1 = hash_line(line1);
    let hash_value_line2 = hash_line(line2);
    hash_value_line1 == hash_value_line2
}

fn map_grid(grid: &str) -> i32 {
    if let (true, x) = find_horizontal(grid) {
        return 100 * x as i32;
    } else if let (true, x) = find_vertical(grid) {
        return x as i32;
    } else {
        panic!("No match found");
    }
}

fn main() {
    //let input = get_input();
    let input = fs::read_to_string("inputs/p13.txt").unwrap();

    let ret = input
        .split("\n\n")
        .fold(0, |acc, grid| acc + map_grid(grid));
    println!("p1: {}", ret);
}
