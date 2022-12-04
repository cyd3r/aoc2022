use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let l0: u32 = caps[1].parse().unwrap();
        let u0: u32 = caps[2].parse().unwrap();
        let l1: u32 = caps[3].parse().unwrap();
        let u1: u32 = caps[4].parse().unwrap();

        if (l0 <= l1 && u1 <= u0) || (l1 <= l0 && u0 <= u1) {
            sum_part1 += 1;
        }

        if (l0 <= l1 && l1 <= u0)
            || (l0 <= u1 && u1 <= u0)
            || (l1 <= l0 && l0 <= u1)
            || (l1 <= u0 && u0 <= u1)
        {
            sum_part2 += 1;
        }
    }
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
}
