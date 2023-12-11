use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    row: i64,
    col: i64,
    num: i64,
    visited: bool,
}

fn expand_galaxy_columns(galaxies: &mut Vec<Point>, num_cols: &Vec<i64>) {
    num_cols.iter().rev().for_each(|col| {
        galaxies.iter_mut().for_each(|galaxy| {
            if galaxy.col > *col {
                galaxy.col += 999999;
            }
        })
    })
}

fn main() {
    let mut galaxies: Vec<Point> = Vec::new();
    let mut row_expansion = 0;
    //let input = get_input();
    let input = fs::read_to_string("inputs/p11.txt").unwrap();
    let mut galaxy_num = 1;

    let mut num_cols: Vec<i64> =
        (0..input.lines().next().unwrap().chars().count() as i64).collect();
    input.lines().enumerate().for_each(|(rdx, line)| {
        // expand empty rows
        if line.chars().find(|ch| *ch == '#').is_none() {
            row_expansion += 999999;
        }

        // create galaxy map without column expansion
        line.chars().enumerate().for_each(|(cdx, ch)| {
            if ch == '#' {
                galaxies.push(Point {
                    row: (rdx + row_expansion) as i64,
                    col: cdx as i64,
                    num: galaxy_num,
                    visited: false,
                });
                galaxy_num += 1;
                if let Some(idx) = num_cols.iter().position(|&x| x == cdx as i64) {
                    num_cols.remove(idx);
                }
            }
        })
    });

    // expand the columns
    expand_galaxy_columns(&mut galaxies, &num_cols);

    let mut inner_galaxy = galaxies.clone(); // need a copy for reasons

    // Loop over every pair of galaxies and calulate distance between them
    // Get the sum of all these distances
    let ret = galaxies
        .iter_mut()
        .enumerate()
        .fold(0, |sum, (idx, galaxy)| {
            let interim_sum = inner_galaxy.iter_mut().fold(0, |inner_sum, other_galaxy| {
                if galaxy != other_galaxy && !other_galaxy.visited {
                    return inner_sum
                        + (galaxy.row - other_galaxy.row).abs()
                        + (galaxy.col - other_galaxy.col).abs();
                }
                inner_sum
            });
            inner_galaxy[idx].visited = true;
            sum + interim_sum
        });
    println!("ret: {}", ret);
}
