use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

type Position = (i32, i32);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: Position,
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

struct Grid {
    cells: Vec<u32>,
    height: usize,
    width: usize,
}
impl Grid {
    /// Checks wether `pos` is a valid position
    fn is_valid(&self, pos: Position) -> bool {
        let h = self.height as i32;
        let w = self.width as i32;
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < h && pos.1 < w
    }
    fn get_height(&self, pos: Position) -> Option<u32> {
        if self.is_valid(pos) {
            let w = self.width as i32;
            Some(self.cells[(pos.0 * w + pos.1) as usize])
        } else {
            None
        }
    }
    fn new(cells: Vec<u32>, height: usize) -> Self {
        Self {
            width: cells.len() / height,
            cells,
            height,
        }
    }
    fn get_neighbours(&self, pos: Position) -> Vec<Position> {
        let npos = vec![
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
        ];
        let npos = npos.iter().filter(|p| self.is_valid(**p)).copied();
        // don't use Vec would be better?
        npos.collect()
    }
}

fn count_steps(
    came_from: &HashMap<Position, Position>,
    start_pos: Position,
    goal_pos: Position,
) -> u32 {
    let mut steps = 0;
    let mut current = goal_pos;
    loop {
        steps += 1;
        current = came_from[&current];
        if current == start_pos {
            break;
        }
    }
    steps
}

fn astar_search(grid: &Grid, start_pos: Position, end_pos: Position) -> Option<u32> {
    // BinaryHeap works like a priority queue
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut cost_so_far: HashMap<Position, i32> = HashMap::new();

    to_visit.push(State {
        cost: 0,
        pos: start_pos,
    });
    cost_so_far.insert(start_pos, 0);
    while let Some(current) = to_visit.pop() {
        if current.pos == end_pos {
            return Some(count_steps(&came_from, start_pos, end_pos));
        }

        // neighbours
        for neighbour in grid.get_neighbours(current.pos) {
            let height_diff = grid.get_height(neighbour).unwrap() as i32
                - grid.get_height(current.pos).unwrap() as i32;
            if height_diff > 1 {
                continue;
            }

            let move_cost = 1;
            let total_move_score = cost_so_far[&current.pos] + move_cost;

            if cost_so_far
                .get(&neighbour)
                .and_then(|g| Some(total_move_score < *g))
                .unwrap_or(true)
            {
                came_from.insert(neighbour, current.pos);
                cost_so_far.insert(neighbour, total_move_score);

                // (hypothetical) lowest possible cost to move from neighbour to the end
                let heuristic = (neighbour.0 - end_pos.0).abs() + (neighbour.1 - end_pos.1).abs();
                to_visit.push(State {
                    pos: neighbour,
                    cost: total_move_score + heuristic,
                });
            }
        }
    }
    None
}

fn dijkstra_search(grid: &Grid, end_pos: Position, start_height: u32) -> Vec<u32> {
    // BinaryHeap works like a priority queue
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut cost_so_far: HashMap<Position, i32> = HashMap::new();

    to_visit.push(State {
        cost: 0,
        pos: end_pos,
    });
    cost_so_far.insert(end_pos, 0);
    let mut steps: Vec<u32> = Vec::new();
    while let Some(current) = to_visit.pop() {
        if grid.get_height(current.pos).unwrap() == start_height {
            steps.push(count_steps(&came_from, end_pos, current.pos));
        }

        // neighbours
        for neighbour in grid.get_neighbours(current.pos) {
            let height_diff = grid.get_height(neighbour).unwrap() as i32
                - grid.get_height(current.pos).unwrap() as i32;
            // for part 2, calculate the path backwards (=> flip height difference compared to part 1)
            if -height_diff > 1 {
                continue;
            }

            let move_cost = 1;
            let total_move_score = cost_so_far[&current.pos] + move_cost;

            if cost_so_far
                .get(&neighbour)
                .and_then(|g| Some(total_move_score < *g))
                .unwrap_or(true)
            {
                came_from.insert(neighbour, current.pos);
                cost_so_far.insert(neighbour, total_move_score);
                to_visit.push(State {
                    pos: neighbour,
                    // no heuristic here compared to A*
                    cost: total_move_score,
                });
            }
        }
    }
    steps
}

fn main() {
    // read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // parse input
    let mut grid: Vec<u32> = Vec::new();
    let mut height = 0;
    let mut start_pos: Position = (0, 0);
    let mut end_pos: Position = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                grid.push(0);
                start_pos = (row as i32, col as i32);
            } else if c == 'E' {
                grid.push(25);
                end_pos = (row as i32, col as i32);
            } else {
                grid.push((c as u32) - 97);
            }
        }
        height = row + 1;
    }
    let grid = Grid::new(grid, height);

    // solve problems
    println!(
        "Part 1: {}",
        astar_search(&grid, start_pos, end_pos).unwrap()
    );
    println!(
        "Part 2: {}",
        dijkstra_search(&grid, end_pos, 0)
            .iter()
            .copied()
            .min()
            .unwrap()
    );
}
