use std::io::{self, Read};

fn draw_part2_step(step: i32, buffer: i32) {
    let width = 40;
    if step >= width * 6 {
        return;
    }
    let sprite_centre = step % width;
    if (buffer - sprite_centre).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if sprite_centre == width - 1 {
        println!();
    }
}
fn handle_part1(step: i32, buffer: i32) -> i32 {
    if (step - 19) % 40 == 0 {
        buffer * (step + 1)
    } else {
        0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut buffer = 1;
    let mut step = 0;
    let mut part1_sum: i32 = 0;

    draw_part2_step(step, buffer);

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let command = parts[0];
        match command {
            "addx" => {
                let value: i32 = parts[1].parse().unwrap();

                // check intermediate step
                step += 1;
                part1_sum += handle_part1(step, buffer);
                draw_part2_step(step, buffer);

                // increase buffer and step
                buffer += value;
                step += 1;
            }
            "noop" => {
                // just increase step
                step += 1;
            }
            _ => panic!("Unknown instruction"),
        };
        // check value after step
        part1_sum += handle_part1(step, buffer);
        draw_part2_step(step, buffer);
    }

    println!("Part 1: {}", part1_sum);
}
