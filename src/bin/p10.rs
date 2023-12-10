use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Node {
    c: char,
    neighbor1: Point,
    neighbor2: Point,
    point: Point,
    visited: bool,
}

fn build_node(c: char, row: i32, col: i32) -> Node {
    let mut neighbor1 = Point { row: 0, col: 0 };
    let mut neighbor2 = Point { row: 0, col: 0 };
    match c {
        '|' => {
            neighbor1 = Point { row: row - 1, col };
            neighbor2 = Point { row: row + 1, col };
        }
        '-' => {
            neighbor1 = Point { row, col: col - 1 };
            neighbor2 = Point { row, col: col + 1 };
        }
        '7' => {
            neighbor1 = Point { row: row + 1, col };
            neighbor2 = Point { row, col: col - 1 };
        }
        'J' => {
            neighbor1 = Point { row: row - 1, col };
            neighbor2 = Point { row, col: col - 1 };
        }
        'L' => {
            neighbor1 = Point { row: row - 1, col };
            neighbor2 = Point { row, col: col + 1 };
        }
        'F' => {
            neighbor1 = Point { row: row + 1, col };
            neighbor2 = Point { row, col: col + 1 };
        }
        _ => {}
    }

    Node {
        c,
        neighbor1,
        neighbor2,
        point: Point { row, col },
        visited: false,
    }
}

// part 1
fn get_path_len(node_map: &mut HashMap<Point, Node>, start_point: &Point) -> i32 {
    let mut curr_node: Node = *node_map.get(&start_point).unwrap();
    if curr_node.c == 'S' && curr_node.visited {
        return 0;
    }
    curr_node.visited = true;
    node_map.insert(*start_point, curr_node);

    if node_map.get(&curr_node.neighbor1).unwrap().visited {
        return 1 + get_path_len(node_map, &curr_node.neighbor2);
    } else {
        return 1 + get_path_len(node_map, &curr_node.neighbor1);
    }
}

// part 2
fn get_path_vec(node_map: &mut HashMap<Point, Node>, start_point: &Point) -> Vec<Point> {
    let mut path = vec![];
    let mut curr_node: Node = *node_map.get(&start_point).unwrap();
    if curr_node.c == 'S' && curr_node.visited {
        return path;
    }
    path.push(curr_node.point);
    curr_node.visited = true;
    node_map.insert(*start_point, curr_node);

    if node_map.get(&curr_node.neighbor1).unwrap().visited {
        path.extend(get_path_vec(node_map, &curr_node.neighbor2));
        return path;
    } else {
        path.extend(get_path_vec(node_map, &curr_node.neighbor1));
        return path;
    }
}

// ray marching algorithm
fn is_inside(point: Point, polygon: &Vec<Point>) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        if (polygon[i].row > point.row) != (polygon[j].row > point.row)
            && point.col
                < (polygon[j].col - polygon[i].col) * (point.row - polygon[i].row)
                    / (polygon[j].row - polygon[i].row)
                    + polygon[i].col
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn main() {
    let mut node_map: HashMap<Point, Node> = HashMap::new();
    let mut start_map = Point { row: 0, col: 0 };
    let binding = fs::read_to_string("inputs/p10.txt").unwrap();
    let _ = binding
        .lines()
        .enumerate()
        .map(|(rdx, line)| {
            line.chars()
                .enumerate()
                .map(|(cdx, c)| {
                    let mut node = build_node(c, rdx as i32, cdx as i32);
                    let point = Point {
                        row: rdx as i32,
                        col: cdx as i32,
                    };
                    let neighbor1: &mut Node =
                        node_map.get_mut(&node.neighbor1).unwrap_or(&mut node);
                    if neighbor1.c == 'S' {
                        neighbor1.neighbor1 = point;
                        start_map = neighbor1.point;
                    }
                    let neighbor2: &mut Node =
                        node_map.get_mut(&node.neighbor2).unwrap_or(&mut node);
                    if neighbor2.c == 'S' {
                        neighbor2.neighbor2 = point;
                        start_map = neighbor2.point;
                    }
                    node_map.insert(point, node);
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // part 1
    // let path_len = get_path_len(&mut node_map, &start_map) / 2;
    //println!("{}", path_len);

    //part 2
    let polygon = get_path_vec(&mut node_map, &start_map);

    // determine the number of points in the polygon using the "ray marching" algorithm
    let mut num_inside = 0;
    for rdx in 0..binding.lines().collect::<Vec<_>>().len() as i32 {
        for cdx in 0..binding.lines().next().unwrap().len() as i32 {
            let node = node_map.get(&Point { row: rdx, col: cdx }).unwrap();
            if !node.visited && is_inside(node.point, &polygon) {
                num_inside += 1;
            }
        }
    }
    println!("{}", num_inside);
}
