// TODO: clone one stack?
use regex::Regex;
use std::io::{self, Read};

#[derive(Debug)]
struct Instruction {
    amount: u32,
    from: usize,
    to: usize,
}

// fn print_stacks(stacks: &Vec<Vec<&str>>) {
//     for (i, stack) in stacks.iter().enumerate() {
//         println!("{}: {}", i + 1, stack.join(" "));
//     }
// }

fn main() {
    let create_re = Regex::new(r"\[([A-Z])\]|    ").unwrap();
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut stacks: Vec<Vec<&str>> = Vec::new();
    let mut stacks2: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        if let Some(capture) = move_re.captures(line) {
            instructions.push(Instruction {
                amount: capture[1].parse().unwrap(),
                from: capture[2].parse().unwrap(),
                to: capture[3].parse().unwrap(),
            })
        } else if line.contains('[') {
            for (i, cargo) in create_re.captures_iter(line).enumerate() {
                if i == stacks.len() {
                    stacks.push(Vec::new());
                    stacks2.push(Vec::new());
                }
                if let Some(m) = cargo.get(1) {
                    stacks[i].insert(0, m.as_str());
                    stacks2[i].insert(0, m.as_str());
                }
            }
        }
    }

    for instruction in instructions {
        let insert_pos = stacks2[instruction.to - 1].len();
        for _ in 0..instruction.amount {
            let x = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(x);

            let x = stacks2[instruction.from - 1].pop().unwrap();
            stacks2[instruction.to - 1].insert(insert_pos, x);
        }
    }

    let lasts: Vec<&str> = stacks.iter().map(|s| s[s.len() - 1]).collect();
    println!("Part 1: {}", lasts.join(""));

    let lasts: Vec<&str> = stacks2.iter().map(|s| s[s.len() - 1]).collect();
    println!("Part 2: {}", lasts.join(""));
}
