use regex::Regex;
use std::io::{self, Read};

// I'm too lazy to write a tree structure in rust
// quick solution: use a vector of nodes
struct Node {
    /// Size must be 0 for directories
    size: u32,
    parent: usize,
    children: Vec<usize>,
}
impl Node {
    pub fn new_file(size: u32, parent: usize) -> Self {
        Self {
            size,
            parent,
            children: Vec::new(),
        }
    }
    pub fn new_dir(parent: usize) -> Self {
        Self {
            size: 0,
            parent,
            children: Vec::new(),
        }
    }
    pub fn get_size(&self, nodes: &Vec<Node>) -> u32 {
        self.size
            + self
                .children
                .iter()
                .map(|child| nodes[child.clone()].get_size(nodes))
                .reduce(|sum, s| sum + s)
                .unwrap_or(0)
    }
}

fn main() {
    let re_cd = Regex::new(r"\$ cd (.+)").unwrap();
    let re_file = Regex::new(r"(\d+) (.+)").unwrap();

    let mut nodes: Vec<Node> = vec![Node::new_dir(0)];
    let mut current_id = 0;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines().skip(1) {
        if let Some(cap) = re_cd.captures(line) {
            // change directory
            let dirname = &cap[1];
            if dirname == ".." {
                let current = &nodes[current_id];
                current_id = current.parent;
            } else {
                let newdir = Node::new_dir(current_id);
                nodes.push(newdir);
                let new_id = nodes.len() - 1;

                let current = &mut nodes[current_id];
                current.children.push(new_id);
                current_id = new_id;
            }
        } else if let Some(cap) = re_file.captures(line) {
            // read file size
            let filesize: u32 = cap[1].parse().unwrap();
            let filenode = Node::new_file(filesize, current_id);
            nodes.push(filenode);
            let new_id = nodes.len() - 1;

            let current_dir = &mut nodes[current_id];
            current_dir.children.push(new_id);
        }
        // dir and ls are irrelevant
    }

    let part1 = nodes
        .iter()
        .filter_map(|node| {
            if node.size > 0 {
                None
            } else {
                let size = node.get_size(&nodes);
                if size <= 100000 {
                    Some(size)
                } else {
                    None
                }
            }
        })
        .reduce(|sum, s| sum + s)
        .unwrap();
    println!("Part 1: {}", part1);

    let used_space = nodes[0].get_size(&nodes);
    let free_space = 70000000 - used_space;
    let to_free = 30000000 - free_space;

    let part2 = nodes
        .iter()
        .filter_map(|node| {
            let childsize = node.get_size(&nodes);
            if node.size == 0 && childsize >= to_free {
                Some(childsize)
            } else {
                None
            }
        })
        .min()
        .unwrap();
    println!("Part 2: {}", part2);
}
