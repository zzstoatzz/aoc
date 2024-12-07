// Detecting Guard Loops with a Single Additional Obstacle

// Re-implemented the loop detection as a finite state traversal problem,
// using boolean arrays instead of hash sets to quickly detect repeated states.
// Treated each (position, direction) as a unique state and indexing directly into arrays.

use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(self, (r, c): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (r - 1, c),
            Direction::Right => (r, c + 1),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c - 1),
        }
    }

    fn to_int(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

struct Lab {
    grid: Vec<Vec<char>>,
    height: i32,
    width: i32,
}

impl Lab {
    fn new(input: &str) -> (Self, (i32, i32), Direction) {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let mut start_pos = (0, 0);
        let mut start_dir = Direction::Up;

        for (r, line) in grid.iter().enumerate() {
            for (c, &ch) in line.iter().enumerate() {
                if ch == '^' {
                    start_pos = (r as i32, c as i32);
                    start_dir = Direction::Up;
                }
            }
        }

        (
            Lab {
                grid,
                height,
                width,
            },
            start_pos,
            start_dir,
        )
    }

    #[inline]
    fn is_valid_pos(&self, (r, c): (i32, i32)) -> bool {
        r >= 0 && r < self.height && c >= 0 && c < self.width
    }

    #[inline]
    fn is_obstacle(&self, (r, c): (i32, i32)) -> bool {
        if !self.is_valid_pos((r, c)) {
            return true; // Treat out-of-bounds as non-traversable
        }
        self.grid[r as usize][c as usize] == '#'
    }
}

fn simulate_guard(lab: &Lab, start_pos: (i32, i32), start_dir: Direction) -> i32 {
    let mut visited = vec![false; (lab.height * lab.width) as usize];
    let mut count = 0;

    let mut pos = start_pos;
    let mut dir = start_dir;

    // Mark visited
    {
        let idx = pos_to_index(pos, lab.width);
        if !visited[idx] {
            visited[idx] = true;
            count += 1;
        }
    }

    loop {
        let next_pos = dir.move_forward(pos);

        // Leave map?
        if !lab.is_valid_pos(next_pos) {
            break;
        }

        if lab.is_obstacle(next_pos) {
            // Turn right
            dir = dir.turn_right();
        } else {
            pos = next_pos;
            let idx = pos_to_index(pos, lab.width);
            if !visited[idx] {
                visited[idx] = true;
                count += 1;
            }
        }
    }

    count
}

fn simulate_guard_with_obstacle(
    lab: &Lab,
    start_pos: (i32, i32),
    start_dir: Direction,
    obstacle_pos: (i32, i32),
) -> bool {
    // visited_states: height * width * 4 states
    let total_states = (lab.height * lab.width * 4) as usize;
    let mut visited_states = vec![false; total_states];

    let mut pos = start_pos;
    let mut dir = start_dir;

    visited_states[state_index(pos, dir, lab.width)] = true;

    loop {
        let next_pos = dir.move_forward(pos);

        // Check leaving map
        if !lab.is_valid_pos(next_pos) {
            return false;
        }

        let blocked = next_pos == obstacle_pos || lab.is_obstacle(next_pos);

        if blocked {
            dir = dir.turn_right();
            let sidx = state_index(pos, dir, lab.width);
            if visited_states[sidx] {
                return true; // loop
            }
            visited_states[sidx] = true;
        } else {
            pos = next_pos;
            let sidx = state_index(pos, dir, lab.width);
            if visited_states[sidx] {
                return true; // loop
            }
            visited_states[sidx] = true;
        }
    }
}

#[inline]
fn pos_to_index((r, c): (i32, i32), width: i32) -> usize {
    (r as usize) * (width as usize) + (c as usize)
}

#[inline]
fn state_index(pos: (i32, i32), dir: Direction, width: i32) -> usize {
    pos_to_index(pos, width) * 4 + dir.to_int()
}

pub fn part1(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let (lab, start_pos, start_dir) = Lab::new(&content);
    simulate_guard(&lab, start_pos, start_dir)
}

fn run_simulation(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let (lab, start_pos, start_dir) = Lab::new(&content);

    let mut valid_positions = 0;
    for r in 0..lab.height {
        for c in 0..lab.width {
            let pos = (r, c);
            if pos == start_pos || lab.is_obstacle(pos) {
                continue;
            }
            if simulate_guard_with_obstacle(&lab, start_pos, start_dir, pos) {
                valid_positions += 1;
            }
        }
    }

    valid_positions
}

pub fn part2(filename: &str) -> i32 {
    let time_it = env::var("TIME_IT").unwrap_or_default() == "1";
    let iterations = 10;

    if time_it {
        let start = Instant::now();
        let mut result = 0;
        for i in 0..iterations {
            result += run_simulation(filename);
            println!(
                "finished iteration {} at {:?}",
                i,
                Instant::now().duration_since(start)
            );
        }
        let duration = start.elapsed();
        println!("Time taken for {} iterations: {:?}", iterations, duration);
        result / iterations
    } else {
        run_simulation(filename)
    }
}
