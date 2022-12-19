use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};


fn get_shortest(from: u32, tunnels: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut to_visit: Vec<u32> = Vec::new();
    let mut came_from: HashMap<u32, u32> = HashMap::new();
    let mut cost_so_far: HashMap<u32, u32> = HashMap::new();

    to_visit.push(from);
    cost_so_far.insert(from, 0);

    while let Some(current) = to_visit.pop() {
        for neigh in tunnels[&current].iter().copied() {
            // faster?
            let cost = cost_so_far[&current];
            if !cost_so_far.contains_key(&neigh) || cost + 1 < cost_so_far[&neigh] {
                came_from.insert(neigh, current);
                cost_so_far.insert(neigh, cost + 1);
                to_visit.insert(0, neigh);
            }
        }
    }

    cost_so_far.remove(&from);
    cost_so_far
}

fn search(
    current: u32,
    minute: u32,
    opened: &HashSet<u32>,
    distances: &HashMap<(u32, u32), u32>,
    flows: &HashMap<u32, u32>,
) -> u32 {
    if minute >= 30 {
        return 0;
    }

    let mut best = 0;
    // TODO: is this performnat?
    let mut opened = opened.clone();
    opened.insert(current);
    for (valve, flow) in flows {
        if opened.contains(valve) {
            continue;
        }
        let valve = *valve;
        let flow = *flow;
        if let Some(distance) = distances.get(&(current, valve)) {
            if flow == 0 {
                continue;
            }
            let subgain = search(valve, minute + distance + 1, &opened, distances, flows);
            best = best.max(subgain);
        }
    }

    let my_gain = flows[&current] * (30 - minute);
    best + my_gain
}

fn part1(flows: &HashMap<u32, u32>, tunnels: &HashMap<u32, Vec<u32>>, start: u32) {
    // find shortest distance to every other valve
    let mut costs: HashMap<(u32, u32), u32> = HashMap::new();
    for (valve, flow) in flows.iter() {
        let valve = *valve;
        if valve != start && *flow == 0 {
            continue;
        }
        costs.extend(
            get_shortest(valve, tunnels)
                .iter()
                .filter_map(|(target, cost)| {
                    if flows[target] == 0 {
                        None
                    } else {
                        Some(((valve, *target), *cost))
                    }
                }),
        );
    }

    let best = search(start, 0, &HashSet::new(), &costs, flows);
    println!("Part 1: {}", best);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let re =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let mut flows_str: HashMap<String, u32> = HashMap::new();
    let mut tunnels_str: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let valve = cap[1].to_string();
        let flow_rate: u32 = cap[2].parse().unwrap();
        let leadsto: Vec<String> = cap[3].split(", ").map(|s| s.to_string()).collect();

        tunnels_str.insert(valve.clone(), leadsto);
        flows_str.insert(valve, flow_rate);
    }

    // I prefer to work with u32 instead of String
    let mut str_to_idx: HashMap<&String, u32> = HashMap::new();
    let mut keys: Vec<&String> = flows_str.keys().collect();
    keys.sort();
    for (i, valve) in keys.iter().enumerate() {
        str_to_idx.insert(valve, i as u32);
    }

    let start_idx = str_to_idx[&String::from("AA")];

    let flows: HashMap<u32, u32> = flows_str
        .iter()
        .map(|(valve, flow)| (str_to_idx[valve], *flow))
        .collect();

    let tunnels: HashMap<u32, Vec<u32>> = tunnels_str
        .iter()
        .map(|(valve, others)| {
            (
                str_to_idx[valve],
                others.iter().map(|o| str_to_idx[o]).collect(),
            )
        })
        .collect();

    part1(&flows, &tunnels, start_idx);
}
