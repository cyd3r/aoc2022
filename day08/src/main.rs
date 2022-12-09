use std::collections::HashMap;
use std::io::{self, Read};

fn part1_traverse(
    trees: &Vec<u32>,
    size: usize,
    visible: &mut Vec<bool>,
    coords: impl Iterator<Item = impl Iterator<Item = (usize, usize)>>,
) {
    let mut last_cell: Option<u32>;
    for group in coords {
        last_cell = None;
        for (row, col) in group {
            let current = trees[row * size + col];
            if let Some(lc) = last_cell {
                if current > lc {
                    visible[row * size + col] = true;
                    last_cell = Some(current);
                }
            } else {
                visible[row * size + col] = true;
                last_cell = Some(current);
            }
        }
    }
}
fn part1(trees: &Vec<u32>, size: usize) {
    let move_right = (0..size).map(|row| (0..size).map(move |col| (row, col)));
    let move_down = (0..size).map(|col| (0..size).map(move |row| (row, col)));
    let move_left = (0..size).map(|row| (0..size).rev().map(move |col| (row, col)));
    let move_up = (0..size).map(|col| (0..size).rev().map(move |row| (row, col)));

    let mut visible: Vec<bool> = trees.iter().map(|_| false).collect();
    part1_traverse(trees, size, &mut visible, move_right);
    part1_traverse(trees, size, &mut visible, move_down);
    part1_traverse(trees, size, &mut visible, move_left);
    part1_traverse(trees, size, &mut visible, move_up);

    println!(
        "Part 1: {}",
        visible
            .iter()
            .map(|v| if *v { 1 } else { 0 })
            .reduce(|sum, v| sum + v)
            .unwrap()
    );
}

fn part2(trees: &Vec<u32>, size: usize) {
    let mut height_idxs: HashMap<u32, usize> = HashMap::new();
    let mut view_count: Vec<usize> = trees.iter().map(|_| 1).collect();
    for row in 0..size {
        height_idxs.clear();
        for col in 0..size {
            let current = trees[row * size + col];
            let closest_blocking = (current..10)
                .filter_map(|height| height_idxs.get(&height))
                .max()
                .unwrap_or(&0);
            view_count[row * size + col] *= col - closest_blocking.clone();
            height_idxs.insert(current, col);
        }

        height_idxs.clear();
        for col in (0..size).rev() {
            let current = trees[row * size + col];
            let right_most = size - 1;
            let closest_blocking = (current..10)
                .filter_map(|height| height_idxs.get(&height))
                .min()
                .unwrap_or(&right_most);
            view_count[row * size + col] *= closest_blocking.clone() - col;

            height_idxs.insert(current, col);
        }
    }

    for col in 0..size {
        height_idxs.clear();
        for row in 0..size {
            let current = trees[row * size + col];
            let closest_blocking = (current..10)
                .filter_map(|height| height_idxs.get(&height))
                .max()
                .unwrap_or(&0);
            view_count[row * size + col] *= row - closest_blocking.clone();

            height_idxs.insert(current, row);
        }

        height_idxs.clear();
        for row in (0..size).rev() {
            let current = trees[row * size + col];
            let bottom_most = size - 1;
            let closest_blocking = (current..10)
                .filter_map(|height| height_idxs.get(&height))
                .min()
                .unwrap_or(&bottom_most);

            view_count[row * size + col] *= closest_blocking.clone() - row;

            height_idxs.insert(current, row);
        }
    }

    println!("Part 2: {}", view_count.iter().max().unwrap());
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut trees: Vec<u32> = Vec::new();
    let mut size: usize = 0;
    for line in input.lines() {
        size = line.len();
        for c in line.chars() {
            trees.push(c.to_digit(10).unwrap());
        }
    }

    part1(&trees, size);
    part2(&trees, size);
}
