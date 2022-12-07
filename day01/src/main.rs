use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut cals: Vec<i32> = input
        .split("\n\n")
        .map(|elf_group| {
            elf_group
                .split("\n")
                .map(|cal| cal.parse::<i32>().unwrap_or(0))
                .reduce(|sum, c| sum + c)
                .unwrap_or(0)
        })
        .collect();

    cals.sort();
    cals.reverse();

    let mut top3_sum = 0;
    for i in 0..3 {
        top3_sum += cals[i];
    }
    println!("Part 1: {}", cals[0]);
    println!("Part 2: {}", top3_sum);
}
