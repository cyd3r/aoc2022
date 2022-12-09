use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<((i32, i32), i32)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let steps = parts[1];
            let steps: i32 = steps.to_string().parse().unwrap();
            let dir = match parts[0] {
                "R" => (0, 1),
                "L" => (0, -1),
                "D" => (1, 0),
                "U" => (-1, 0),
                _ => panic!("unknown direction"),
            };
            (dir, steps)
        })
        .collect();

    let mut visited_first: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_last: HashSet<(i32, i32)> = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tails: Vec<(i32, i32)> = (0..9).map(|_| (0, 0)).collect();
    visited_first.insert(tails[0]);
    visited_last.insert(tails[tails.len() - 1]);
    for (dir, steps) in instructions {
        for _ in 0..steps {
            head_pos.0 += dir.0;
            head_pos.1 += dir.1;

            let mut prev = head_pos;
            for i in 0..9 {
                let current = tails[i];
                // tail follows
                let dist = (prev.0 - current.0, prev.1 - current.1);
                if dist.0.abs() > 1 || dist.1.abs() > 1 {
                    // move at most one step in each direction (signum)
                    tails[i].0 = current.0 + dist.0.signum();
                    tails[i].1 = current.1 + dist.1.signum();
                }
                prev = tails[i];
            }
            // keep track of visited points
            visited_last.insert(tails[tails.len() - 1]);
            visited_first.insert(tails[0]);
        }
    }
    println!("Part 1: {}", visited_first.len());
    println!("Part 2: {}", visited_last.len());
}
