use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    let mut cals: Vec<i32> = contents
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
    println!("Top: {}", cals[0]);
    println!("Top3 Sum: {}", top3_sum);
}
