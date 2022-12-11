use regex::Regex;
use std::io::{self, Read};

enum Operation {
    Mult(u64),
    Add(u64),
    Sqr,
}

fn part1(
    starting_items: &Vec<Vec<u64>>,
    ops: &Vec<Operation>,
    test_divs: &Vec<u64>,
    test_pos: &Vec<usize>,
    test_neg: &Vec<usize>,
) {
    let mut inspect_nums: Vec<u64> = starting_items.iter().map(|_| 0).collect();
    let mut items: Vec<Vec<u64>> = starting_items.clone();

    for _ in 0..20 {
        for monkey in 0..items.len() {
            inspect_nums[monkey] += items[monkey].len() as u64;
            while let Some(itm) = items[monkey].pop() {
                let new_value = match ops[monkey] {
                    Operation::Sqr => (itm * itm) / 3,
                    Operation::Add(x) => (itm + x) / 3,
                    Operation::Mult(x) => (itm * x) / 3,
                };
                if new_value % test_divs[monkey] == 0 {
                    items[test_pos[monkey]].push(new_value);
                } else {
                    items[test_neg[monkey]].push(new_value);
                }
            }
        }
    }
    inspect_nums.sort();
    println!(
        "Part 1: {}",
        inspect_nums[inspect_nums.len() - 1] * inspect_nums[inspect_nums.len() - 2]
    );
}

fn part2(
    starting_items: &Vec<Vec<u64>>,
    ops: &Vec<Operation>,
    test_divs: &Vec<u64>,
    test_pos: &Vec<usize>,
    test_neg: &Vec<usize>,
) {
    let div_prod: u64 = test_divs.iter().product();

    let mut inspect_nums: Vec<u64> = starting_items.iter().map(|_| 0).collect();
    let mut items: Vec<Vec<u64>> = starting_items.clone();

    for _ in 0..10000 {
        for monkey in 0..items.len() {
            inspect_nums[monkey] += items[monkey].len() as u64;
            while let Some(itm) = items[monkey].pop() {
                let new_value = match ops[monkey] {
                    Operation::Sqr => itm * itm,
                    Operation::Add(x) => itm + x,
                    Operation::Mult(x) => itm * x,
                };
                if new_value % test_divs[monkey] == 0 {
                    items[test_pos[monkey]].push(new_value % div_prod);
                } else {
                    items[test_neg[monkey]].push(new_value % div_prod);
                }
            }
        }
    }

    inspect_nums.sort();
    println!(
        "Part 2: {}",
        inspect_nums[inspect_nums.len() - 1] * inspect_nums[inspect_nums.len() - 2]
    );
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re_starting = Regex::new(r"  Starting items: (.+)").unwrap();
    let re_test = Regex::new(r"  Test: divisible by (\d+)").unwrap();
    let re_op = Regex::new(r"  Operation: new = old (.) (.+)").unwrap();
    let re_test_true = Regex::new(r"    If true: throw to monkey (\d)").unwrap();
    let re_test_false = Regex::new(r"    If false: throw to monkey (\d)").unwrap();

    let mut items: Vec<Vec<u64>> = Vec::new();

    let mut ops: Vec<Operation> = Vec::new();
    // test divisors
    let mut test_divs: Vec<u64> = Vec::new();
    // monkey indices for positive tests
    let mut test_pos: Vec<usize> = Vec::new();
    // monkey indices for negative tests
    let mut test_neg: Vec<usize> = Vec::new();

    for line in input.lines() {
        if let Some(caps) = re_starting.captures(line) {
            let mut starting: Vec<u64> = caps[1]
                .split(", ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            starting.reverse();
            items.push(starting);
        } else if let Some(caps) = re_op.captures(line) {
            ops.push(match (&caps[1], &caps[2]) {
                ("*", "old") => Operation::Sqr,
                ("*", _) => Operation::Mult(caps[2].parse().unwrap()),
                ("+", _) => Operation::Add(caps[2].parse().unwrap()),
                _ => panic!("Unknown operation"),
            });
        } else if let Some(caps) = re_test.captures(line) {
            test_divs.push(caps[1].parse().unwrap());
        } else if let Some(caps) = re_test_true.captures(line) {
            test_pos.push(caps[1].parse().unwrap());
        } else if let Some(caps) = re_test_false.captures(line) {
            test_neg.push(caps[1].parse().unwrap());
        }
    }

    part1(&items, &ops, &test_divs, &test_pos, &test_neg);
    part2(&items, &ops, &test_divs, &test_pos, &test_neg);
}
