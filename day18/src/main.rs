use std::collections::HashSet;
use std::io::{self, Read};

type Position = (i32, i32, i32);

fn neighbour_sides(normal: Position) -> Vec<Position> {
    if normal.0 != 0 {
        return vec![(0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
    }
    if normal.1 != 0 {
        return vec![(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];
    }
    if normal.2 != 0 {
        return vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0)];
    }
    panic!("Not a 3-tuple anymore");
}

fn add(a: Position, b: Position) -> Position {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn invert(normal: Position) -> Position {
    (-normal.0, -normal.1, -normal.2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (mut minx, mut maxx) = (None, None);
    let (mut miny, mut maxy) = (None, None);
    let (mut minz, mut maxz) = (None, None);

    let mut cubes: HashSet<Position> = HashSet::new();
    let mut contacts = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        let z = parts[2].parse().unwrap();

        if minx.is_none() || x < minx.unwrap() {
            minx = Some(x);
        }
        if maxx.is_none() || x > maxx.unwrap() {
            maxx = Some(x);
        }
        if miny.is_none() || y < miny.unwrap() {
            miny = Some(y);
        }
        if maxy.is_none() || y > maxy.unwrap() {
            maxy = Some(y);
        }
        if minz.is_none() || z < minz.unwrap() {
            minz = Some(z);
        }
        if maxz.is_none() || z > maxz.unwrap() {
            maxz = Some(z);
        }

        cubes.insert((x, y, z));
        if cubes.contains(&(x - 1, y, z)) {
            contacts += 1;
        }
        if cubes.contains(&(x + 1, y, z)) {
            contacts += 1;
        }
        if cubes.contains(&(x, y - 1, z)) {
            contacts += 1;
        }
        if cubes.contains(&(x, y + 1, z)) {
            contacts += 1;
        }
        if cubes.contains(&(x, y, z - 1)) {
            contacts += 1;
        }
        if cubes.contains(&(x, y, z + 1)) {
            contacts += 1;
        }
    }
    let minx = minx.unwrap();
    let maxx = maxx.unwrap();
    let miny = miny.unwrap();
    let maxy = maxy.unwrap();
    let minz = minz.unwrap();
    let maxz = maxz.unwrap();

    println!("Part 1: {}", 6 * cubes.len() - 2 * contacts);

    // find neighbour sides
    let mut outside: HashSet<(Position, Position)> = HashSet::new();

    let mut to_visit: Vec<(Position, Position)> = Vec::new();
    for (x, y, z) in cubes.iter().copied() {
        if x == minx {
            to_visit.push(((x, y, z), (-1, 0, 0)));
        }
        if x == maxx {
            to_visit.push(((x, y, z), (1, 0, 0)));
        }
        if y == miny {
            to_visit.push(((x, y, z), (0, -1, 0)));
        }
        if y == maxy {
            to_visit.push(((x, y, z), (0, 1, 0)));
        }
        if z == minz {
            to_visit.push(((x, y, z), (0, 0, -1)));
        }
        if z == maxz {
            to_visit.push(((x, y, z), (0, 0, 1)));
        }
    }

    while let Some((pos, normal)) = to_visit.pop() {
        let normal_offset = add(pos, normal);

        if cubes.contains(&normal_offset) {
            panic!("{:?} should have a free normal", (pos, normal));
        }

        // find neighbours and add them to to_visit
        for neigh_normal in neighbour_sides(normal) {
            let diagonal = add(normal_offset, neigh_normal);
            let parallel = add(pos, neigh_normal);

            let next = if cubes.contains(&diagonal) {
                (diagonal, invert(neigh_normal))
            } else if cubes.contains(&parallel) {
                (parallel, normal)
            } else {
                (pos, neigh_normal)
            };

            if !outside.contains(&next) && !to_visit.contains(&next) {
                to_visit.push(next);
            }
        }

        outside.insert((pos, normal));
    }

    println!("Part 2: {}", outside.len());
}
