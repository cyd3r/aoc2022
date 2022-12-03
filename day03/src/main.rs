use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn char_score(char: char) -> Option<u32> {
    let codepoint = char as u32;
    if codepoint >= 97 && codepoint <= 122 {
        return Some(codepoint - 96);
    }
    if codepoint >= 65 && codepoint <= 90 {
        return Some(codepoint - 64 + 26);
    }
    None
}

fn get_prio(line: &str) -> Option<u32> {
    let mut char_set: HashSet<char> = HashSet::new();
    let middle = line.len() / 2;
    for (i, c) in line.chars().enumerate() {
        if i >= middle {
            if char_set.contains(&c) {
                return Some(char_score(c).unwrap());
            }
        } else {
            char_set.insert(c);
        }
    }
    None
}

fn part1(input: &str) {
    let sum = input
        .lines()
        .map(|l| get_prio(l).unwrap())
        .reduce(|sum, p| sum + p);
    println!("Part 1: {}", sum.unwrap());
}

fn line_to_set(line: &str) -> HashSet<char> {
    HashSet::from_iter(line.chars())
}
fn part2(input: &str) {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let elf1 = line_to_set(lines[i]);
        let elf2 = line_to_set(lines[i + 1]);
        let elf3 = line_to_set(lines[i + 2]);
        let inter = &(&elf1 & &elf2) & &elf3;
        let char = inter.into_iter().next().unwrap();
        sum += char_score(char).unwrap();
    }
    println!("Part 2: {}", sum);
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input);
    part2(&input);
    Ok(())
}
