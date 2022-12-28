use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: u32,
}
// required for BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_shortest2(from: u32, tunnels: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    let mut cost_so_far: HashMap<u32, u32> = HashMap::new();
    let mut came_from: HashMap<u32, u32> = HashMap::new();

    to_visit.push(State { cost: 0, pos: from });
    cost_so_far.insert(from, 0);
    while let Some(current) = to_visit.pop() {
        for neighbour in tunnels[&current.pos].iter().copied() {
            let new_cost = cost_so_far[&current.pos] + 1;
            if !cost_so_far.contains_key(&neighbour) || new_cost < cost_so_far[&neighbour] {
                came_from.insert(neighbour, current.pos);
                cost_so_far.insert(neighbour, new_cost);
                to_visit.push(State {
                    cost: new_cost,
                    pos: neighbour,
                })
            }
        }
    }

    cost_so_far
}

fn get_shortest(from: u32, tunnels: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut to_visit: Vec<u32> = Vec::new();
    let mut came_from: HashMap<u32, u32> = HashMap::new();
    let mut cost_so_far: HashMap<u32, u32> = HashMap::new();

    to_visit.push(from);
    cost_so_far.insert(from, 0);

    while let Some(current) = to_visit.pop() {
        for neigh in tunnels[&current].iter().copied() {
            // faster?
            let cost = cost_so_far[&current] + 1;
            if !cost_so_far.contains_key(&neigh) || cost < cost_so_far[&neigh] {
                came_from.insert(neigh, current);
                cost_so_far.insert(neigh, cost);
                to_visit.insert(0, neigh);
            }
        }
    }

    cost_so_far.remove(&from);
    cost_so_far
}

fn with_valves(
    paths: (Vec<(u32, u32)>, Vec<(u32, u32)>),
    valve0: Option<(u32, u32)>,
    valve1: Option<(u32, u32)>,
) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut paths = paths.clone();
    if let Some(v0) = valve0 {
        paths.0.insert(0, v0);
    }
    if let Some(v1) = valve1 {
        paths.1.insert(0, v1);
    }
    paths
}

fn search2(
    current: (u32, u32),
    minute: (u32, u32),
    opened: &HashSet<u32>,
    distances: &HashMap<(u32, u32), u32>,
    flows: &HashMap<u32, u32>,
) -> (u32, (Vec<(u32, u32)>, Vec<(u32, u32)>)) {
    // TODO: is .clone here performant?
    let mut opened = opened.clone();
    // let mut opened2:HashSet<u32>=HashSet::new();
    // for op in opened {
    //     opened2.insert(op.clone());
    // }
    // let mut opened=opened2;
    let timeout = 26;

    assert!(!opened.contains(&current.0) || !opened.contains(&current.1));

    let mut cur_flow = 0;
    let mut cur0 = None;
    let mut cur1 = None;
    if !opened.contains(&current.0) {
        // path 0 was recently extended
        opened.insert(current.0);
        if minute.0 <= timeout {
            cur0 = Some((current.0, minute.0));
            cur_flow += flows[&current.0] * (timeout - minute.0);
        }
    }
    if !opened.contains(&current.1) {
        // path 1 was recently extended
        opened.insert(current.1);
        if minute.1 <= timeout {
            cur1 = Some((current.1, minute.1));
            cur_flow += flows[&current.1] * (timeout - minute.1);
        }
    }

    if minute.0 < minute.1 {
        if minute.0 >= timeout {
            // it this is the case, valve 0 and valve 1 will be outside the timeout
            assert!(minute.1 >= timeout);
            return (0, (Vec::new(), Vec::new()));
        }

        // 0 was just opened
        // only 0 moves
        let mut best = 0;
        let mut best_paths = (Vec::new(), Vec::new());
        for (valve, _flow) in flows {
            if opened.contains(valve) {
                continue;
            }
            if let Some(dist) = distances.get(&(current.0, *valve)) {
                let (subgain, paths) = search2(
                    (*valve, current.1),
                    (minute.0 + dist + 1, minute.1),
                    &opened,
                    distances,
                    flows,
                );
                if subgain > best {
                    best = subgain;
                    best_paths = paths;
                }
            }
        }

        return (best + cur_flow, with_valves(best_paths, cur0, cur1));
        // return (best + flows[&current.0] * (timeout - minute.0), best_paths);
    } else {
        if minute.1 >= timeout {
            assert!(minute.0 >= timeout);
            return (0, (Vec::new(), Vec::new()));
        }
        // 1 was just opened
        // only 1 moves
        let mut best = 0;
        let mut best_paths = (Vec::new(), Vec::new());
        for (valve, _flow) in flows {
            if opened.contains(valve) {
                continue;
            }
            if let Some(dist) = distances.get(&(current.1, *valve)) {
                let (subgain, paths) = search2(
                    (current.0, *valve),
                    (minute.0, minute.1 + dist + 1),
                    &opened,
                    distances,
                    flows,
                );

                if subgain > best {
                    best = subgain;
                    best_paths = paths;
                }
            }
        }

        return (best + cur_flow, with_valves(best_paths, cur0, cur1));
        // return (best + flows[&current.1] * (timeout - minute.1), best_paths);
    }
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
            get_shortest2(valve, tunnels)
                .iter()
                .filter_map(|(target, cost)| {
                    if flows[target] == 0 && *target != start {
                        None
                    } else {
                        Some(((valve, *target), *cost))
                    }
                }),
        );
    }

    for i in 0..10 {
        let mut written = false;
        for j in 0..10 {
            if costs.contains_key(&(i, j)) {
                written = true;
                print!("{}, ", costs[&(i, j)]);
            }
        }
        if written {
            println!();
        }
    }

    let best = search(start, 0, &HashSet::new(), &costs, flows);
    println!("Part 1: {}", best);

    let (best, best_paths) = search2((start, start), (0, 0), &HashSet::new(), &costs, flows);
    println!("Part 2: {}", best);
    println!("Paths {:?}", best_paths);
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

    println!("str to idx: {:?}", str_to_idx);
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
