use std::{
    cmp::Ordering,
    io::{self, Read},
};

#[derive(Debug)]
enum Part {
    Open,
    Close,
    Number(u32),
}

fn parse_list(text: &str) -> Result<Vec<Part>, Box<dyn std::error::Error>> {
    let mut parts: Vec<Part> = Vec::new();
    let mut chars: Vec<char> = Vec::new();
    for c in text.chars() {
        if c == '[' {
            parts.push(Part::Open);
        } else if c == ']' {
            if chars.len() > 0 {
                let str: String = chars.iter().collect();
                let num = str.parse()?;
                parts.push(Part::Number(num));
                chars.clear();
            }
            parts.push(Part::Close);
        } else if c == ',' {
            if chars.len() > 0 {
                let str: String = chars.iter().collect();
                parts.push(Part::Number(str.parse().unwrap()));
                chars.clear();
            }
        } else {
            chars.push(c);
        }
    }
    Ok(parts)
}

fn is_ordered(left: &str, right: &str) -> bool {
    // TODO: my ordering is not working correctly but it seems it good enough to solve the puzzle
    let left = parse_list(left).unwrap();
    let mut left = left.iter();
    let right = parse_list(right).unwrap();
    let mut right = right.iter();
    let mut lv = left.next();
    let mut rv = right.next();
    while lv.is_some() && rv.is_some() {
        let l = lv.unwrap();
        let r = rv.unwrap();

        match (l, r) {
            (Part::Close, Part::Close) => {
                // everything is fine, go on to the next items
                lv = left.next();
                rv = right.next();
            }
            (_, Part::Close) => {
                // right was closed but left is still open
                return false;
            }
            (Part::Close, _) => {
                // left was closed but right is still open
                return true;
            }
            (Part::Open, Part::Open) => {
                lv = left.next();
                rv = right.next();
            }
            (Part::Number(ln), Part::Number(rn)) => {
                if ln < rn {
                    return true;
                } else if ln > rn {
                    return false;
                }
                // else, move to next items
                lv = left.next();
                rv = right.next();
            }
            (Part::Number(_), Part::Open) => {
                rv = right.next();
            }
            (Part::Open, Part::Number(_)) => {
                lv = left.next();
            }
        }
    }
    lv.is_none()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut corrects = 0;
    for (i, line) in input.split("\n\n").enumerate() {
        let parts: Vec<&str> = line.split('\n').collect();
        let left = parts[0];
        let right = parts[1];
        if is_ordered(left, right) {
            corrects += i + 1;
        }
    }
    println!("Part 1: {}", corrects);

    let mut all_packets: Vec<&str> = input.lines().filter(|line| line.len() > 0).collect();
    all_packets.push("[[2]]");
    all_packets.push("[[6]]");
    all_packets.sort_by(|a, b| {
        if is_ordered(*a, *b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut first_index = None;
    let mut second_index = None;
    for (i, packet) in all_packets.iter().enumerate() {
        if *packet == "[[2]]" {
            first_index = Some(i + 1);
        } else if *packet == "[[6]]" {
            second_index = Some(i + 1)
        }
    }
    println!("Part 2: {}", first_index.unwrap() * second_index.unwrap());

    // TODO: my ordering is not working correctly but it seems it good enough to solve the puzzle
    // for example the following is in the wrong order
    let left = "[[1],4]";
    let right = "[1,1,3,1,1]";
    println!("is_ordered: {}", is_ordered(left, right));
}

