use std::collections::HashSet;
use std::io::{self, Read};

fn find_marker(input: &str, msg_len: usize) -> Option<usize> {
    let mut marker: Vec<char> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        marker.insert(0, c);
        if marker.len() == msg_len + 1 {
            marker.pop();
            let unique: HashSet<&char> = (&marker).into_iter().collect();
            if unique.len() == msg_len {
                return Some(i + 1);
            }
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", find_marker(&input, 4).unwrap());
    println!("Part 2: {}", find_marker(&input, 14).unwrap());
}
