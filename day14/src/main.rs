use std::collections::HashSet;
use std::io::{self, Read};

type Position = (i32, i32);

fn part1(rocks: &HashSet<Position>) {
    let lowest = rocks.iter().map(|(_, y)| y).max().unwrap().clone();
    let mut rocks = rocks.clone();

    let mut came_to_rest = 0;
    loop {
        // spawn new sand unit
        let mut sand = (500, 0);
        while sand.1 <= lowest {
            if rocks.contains(&(sand.0, sand.1 + 1)) {
                if rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                    if rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                        // sand will stay here
                        rocks.insert(sand);
                        came_to_rest += 1;
                        break;
                    } else {
                        sand = (sand.0 + 1, sand.1 + 1);
                    }
                } else {
                    sand = (sand.0 - 1, sand.1 + 1);
                }
            } else {
                sand = (sand.0, sand.1 + 1);
            }
        }
        // sand is falling
        if sand.1 > lowest {
            break;
        }
    }

    println!("Part 1: {}", came_to_rest);
}

fn part2(rocks:&HashSet<Position>) {
    let lowest = rocks.iter().map(|(_, y)| y).max().unwrap().clone();
    let mut rocks = rocks.clone();

    let mut came_to_rest = 0;
    loop {
        // spawn new sand unit
        let mut sand = (500, 0);
        loop {
            if sand.1 + 1 == lowest + 2 {
                rocks.insert(sand);
                came_to_rest += 1;
                break;
            }

            if rocks.contains(&(sand.0, sand.1 + 1)) {
                // cannot move down
                if rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                    // cannot move left
                    if rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                        // cannot move right
                        // sand will stay here
                        rocks.insert(sand);
                        came_to_rest += 1;
                        break;
                    } else {
                        // can move right
                        sand = (sand.0 + 1, sand.1 + 1);
                    }
                } else {
                    // can move left
                    sand = (sand.0 - 1, sand.1 + 1);
                }
            } else {
                // can move down
                sand = (sand.0, sand.1 + 1);
            }
        }
        if sand == (500, 0) {
            break;
        }
    }

    // draw(&rocks);

    println!("Part 2: {}", came_to_rest);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut rocks: HashSet<Position> = HashSet::new();
    let mut lowest = 0;

    for line in input.lines() {
        let mut last_pos: Option<Position> = None;
        for corner in line.split(" -> ") {
            let coord: Vec<&str> = corner.split(',').collect();
            let new_corner: Position = (coord[0].parse().unwrap(), coord[1].parse().unwrap());

            lowest = lowest.max(new_corner.1);

            if let Some(prev) = last_pos {
                // store rock
                if prev.0 == new_corner.0 {
                    let from = prev.1.min(new_corner.1);
                    let to = prev.1.max(new_corner.1) + 1;
                    for y in from..to {
                        rocks.insert((new_corner.0, y));
                    }
                } else if prev.1 == new_corner.1 {
                    let from = prev.0.min(new_corner.0);
                    let to = prev.0.max(new_corner.0) + 1;
                    for x in from..to {
                        rocks.insert((x, new_corner.1));
                    }
                } else {
                    panic!("Should not happen");
                }
            }

            last_pos = Some(new_corner);
        }
    }

    part1(&rocks);
    part2(&rocks);
}
