use std::io::{self, Read, Write};
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn score(line: &str) -> Option<u32> {
    // TODO: validate length
    let opponent = line.chars().nth(0)?;
    let my = line.chars().nth(2)?;
    let choice_score = match my {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };
    let win_score = match (opponent, my) {
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        _ => 0,
    };
    Some(win_score + choice_score)
}

fn part1(input: &str) {
    let score_sum = input
        .split('\n')
        .map(|l| score(l).expect("invalid line"))
        .reduce(|sum, scr| sum + scr);
    if let Some(score_sum) = score_sum {
        println!("Score Sum: {}", score_sum);
    } else {
        println!("That did not work");
    }
}

fn score2(line: &str) -> u32 {
    let opponent = line.chars().nth(0).unwrap();
    let choice = match opponent {
        'A' => 0,
        'B' => 1,
        _ => 2,
    };
    let my = line.chars().nth(2).unwrap();
    let shift = match my {
        'Y' => 0,
        'Z' => 1,
        _ => 2,
    };
    let win_score = ((shift + 1) % 3) * 3;
    let choice_score = ((choice + shift) % 3) + 1;
    win_score + choice_score
}
fn part2(input: &str) {
    let score_sum = input.split('\n').map(score2).reduce(|sum, scr| sum + scr);
    println!("Sum: {}", score_sum.unwrap());
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let trimmed = input.trim();
    part1(trimmed);
    part2(trimmed);
    Ok(())
}
