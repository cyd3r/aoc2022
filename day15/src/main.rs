use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};

type Position = (i64, i64);

fn merge_intervals(intervals: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if intervals.len() == 0 {
        return Vec::new();
    }
    let mut intervals = intervals.clone();
    intervals.sort_by_key(|(start, _end)| -start.clone());
    let mut interval_stack: Vec<(i64, i64)> = Vec::new();
    interval_stack.push(intervals.pop().unwrap());

    while let Some(itvl) = intervals.pop() {
        let lasti = interval_stack.len() - 1;
        let last_itvl = interval_stack[lasti];
        if last_itvl.0 <= itvl.0 && itvl.0 <= last_itvl.1 {
            interval_stack[lasti] = (last_itvl.0, last_itvl.1.max(itvl.1));
        } else {
            interval_stack.push(itvl);
        }
    }
    interval_stack
}

fn part1(sensors: &Vec<(Position, i64)>, beacons: &HashSet<Position>) {
    // let y = 10; // example
    let y = 2000000; // puzzle input
    let mut intervals: Vec<(i64, i64)> = Vec::new();

    for (sensor, range) in sensors {
        let range = range.clone();
        let ydist = (y - sensor.1).abs();

        if ydist == range {
            intervals.push((sensor.0, sensor.0));
        } else if ydist < range {
            let left = sensor.0 - range + ydist;
            let right = range + sensor.0 - ydist;
            intervals.push((left, right));
        }
    }

    let interval_stack = merge_intervals(&intervals);

    let part1: i64 = interval_stack
        .iter()
        .map(|(start, end)| {
            // beacons coun't as free space -> subtract them
            let mut contained_beacons = 0;
            for beacon in beacons {
                if beacon.1 == y && start <= &beacon.0 && &beacon.0 <= end {
                    contained_beacons += 1;
                }
            }
            end - start + 1 - contained_beacons
        })
        .sum();
    println!("Part 1: {}", part1);
}

enum Quadrant {
    NE,
    NW,
    SW,
    SE,
}

fn edge_offset(sensor: Position, range: i64, quadrant: Quadrant) -> i64 {
    match quadrant {
        Quadrant::NE => range + sensor.0 + sensor.1,
        Quadrant::NW => range - sensor.0 + sensor.1,
        Quadrant::SW => -range + sensor.0 + sensor.1,
        Quadrant::SE => -range - sensor.0 + sensor.1,
    }
}

fn draw_ranges(sensors: &Vec<(Position, i64)>) {
    let mut covered: HashSet<Position> = HashSet::new();
    for (sensor, range) in sensors {
        let range = range.clone();
        for y in (-range)..(range + 1) {
            for x in (-range)..(range + 1) {
                if x.abs() + y.abs() <= range {
                    covered.insert((sensor.0 + x, sensor.1 + y));
                }
            }
        }
    }

    let minx = covered.iter().map(|(x, _y)| *x).min().unwrap();
    let miny = covered.iter().map(|(_x, y)| *y).min().unwrap();
    let maxx = covered.iter().map(|(x, _y)| *x).max().unwrap();
    let maxy = covered.iter().map(|(_x, y)| *y).max().unwrap();

    println!("x from {} to {}, y from {} to {}", minx, maxx, miny, maxy);

    for y in miny..(maxy + 1) {
        for x in minx..(maxx + 1) {
            if covered.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part2(sensors: &Vec<(Position, i64)>) {
    // let space = 20; // example
    // let space = 4000000; // puzzle input

    // draw the space
    draw_ranges(sensors);

    /*
    quadrants:

         /|\
     NW / | \ NE
       /  |  \
      (---s---)
       \  |  /
     SW \ | / SE
         \|/
    */

    for (i, (sensor, range)) in sensors.iter().copied().enumerate() {
        let offset_ne = edge_offset(sensor, range, Quadrant::NE);
        let offset_nw = edge_offset(sensor, range, Quadrant::NW);
        let offset_se = edge_offset(sensor, range, Quadrant::SE);
        let offset_sw = edge_offset(sensor, range, Quadrant::SW);

        let mut ne_intervals: Vec<(i64, i64)> = Vec::new();
        let mut nw_intervals: Vec<(i64, i64)> = Vec::new();
        let mut se_intervals: Vec<(i64, i64)> = Vec::new();
        let mut sw_intervals: Vec<(i64, i64)> = Vec::new();

        // find intersection points
        for j in 0..sensors.len() {
            if i == j {
                continue;
            }
            let (other, other_range) = sensors[j];

            // can these two sensors interact?

            let other_ne = edge_offset(other, other_range, Quadrant::NE);
            let other_nw = edge_offset(other, other_range, Quadrant::NW);
            let other_se = edge_offset(other, other_range, Quadrant::SE);
            let other_sw = edge_offset(other, other_range, Quadrant::SW);
            // is the sensor's border between the two parallel borders from other?

            // check north east edge
            // north east in between of other's south west or north east?
            // -1 because two parallel lines also don't allow any empty places
            println!(
                "other: ne {}, nw {}, se {}, sw {}",
                other_ne, other_nw, other_se, other_sw
            );
            if other_sw <= offset_ne + 1 && offset_ne + 1 <= other_ne {
                // is in between
                let top = other_nw.min(offset_nw + 1);
                let bottom = other_se.max(offset_se - 1);
                if bottom <= top {
                    ne_intervals.push((bottom, top));
                }
            }

            if other_se <= offset_nw + 1 && offset_nw + 1 <= other_nw {
                let top = other_ne.min(offset_ne + 1);
                let bottom = other_sw.max(offset_sw - 1);
                if bottom <= top {
                    nw_intervals.push((bottom, top));
                }
            }

            if other_ne >= offset_sw - 1 && offset_sw - 1 >= other_sw {
                let top = other_nw.min(offset_nw + 1);
                let bottom = other_se.max(offset_se - 1);
                if bottom <= top {
                    sw_intervals.push((bottom, top));
                }
            }

            if other_nw >= offset_se - 1 && offset_se - 1 >= other_se {
                let top = other_ne.min(offset_ne + 1);
                let bottom = other_sw.max(offset_sw - 1);
                if bottom <= top {
                    se_intervals.push((bottom, top));
                }
            }
        }

        let ne_intervals = merge_intervals(&ne_intervals);
        let nw_intervals = merge_intervals(&nw_intervals);
        let se_intervals = merge_intervals(&se_intervals);
        let sw_intervals = merge_intervals(&sw_intervals);
        println!(
            "Sensor:\n  ne: {:?} along {} {}\n  nw: {:?} along {} {}\n  sw: {:?} along {} {}\n  se: {:?} along {} {}",
            ne_intervals, offset_se, offset_nw,
            nw_intervals, offset_sw, offset_ne,
            sw_intervals, offset_se, offset_nw,
            se_intervals, offset_sw, offset_ne
        );
        break;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let pattern =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut beacons: HashSet<Position> = HashSet::new();
    let mut sensors: Vec<(Position, i64)> = Vec::new();

    for line in input.lines() {
        let cap = pattern.captures(line).expect("Regex should always match");
        let beacon: Position = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
        let sensor: Position = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        // manhattan distance
        let range = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
        sensors.push((sensor, range));
        beacons.insert(beacon);
    }

    part1(&sensors, &beacons);
    part2(&sensors);
}
