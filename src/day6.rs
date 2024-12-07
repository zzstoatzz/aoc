use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, pos: (i32, i32)) -> (i32, i32) {
        let (row, col) = pos;
        match self {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
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

        // Find starting position and direction
        let mut start_pos = (0, 0);
        let mut start_dir = Direction::Up;

        for (row, line) in grid.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                if ch == '^' {
                    start_pos = (row as i32, col as i32);
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

    fn is_valid_pos(&self, pos: (i32, i32)) -> bool {
        let (row, col) = pos;
        row >= 0 && row < self.height && col >= 0 && col < self.width
    }

    fn is_obstacle(&self, pos: (i32, i32)) -> bool {
        if !self.is_valid_pos(pos) {
            return true;
        }
        self.grid[pos.0 as usize][pos.1 as usize] == '#'
    }
}

fn simulate_guard(lab: &Lab, start_pos: (i32, i32), start_dir: Direction) -> i32 {
    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;

    visited.insert(pos);

    loop {
        let next_pos = dir.move_forward(pos);

        if !lab.is_valid_pos(next_pos) {
            break;
        }

        if lab.is_obstacle(next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }

    visited.len() as i32
}

/// Simulate guard with a single new obstacle at `obstacle_pos`
fn simulate_guard_with_obstacle(
    lab: &Lab,
    start_pos: (i32, i32),
    start_dir: Direction,
    obstacle_pos: (i32, i32),
) -> bool {
    let mut visited_states = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;

    visited_states.insert((pos, dir));

    loop {
        let next_pos = dir.move_forward(pos);

        // Check if leaving map first
        if !lab.is_valid_pos(next_pos) {
            return false; // Guard leaves map, no loop formed
        }

        // Check if blocked by existing obstacle or new obstacle
        let blocked = next_pos == obstacle_pos || lab.is_obstacle(next_pos);

        if blocked {
            dir = dir.turn_right();
            if !visited_states.insert((pos, dir)) {
                return true; // Loop detected
            }
        } else {
            pos = next_pos;
            if !visited_states.insert((pos, dir)) {
                return true; // Loop detected
            }
        }
    }
}

pub fn part1(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let (lab, start_pos, start_dir) = Lab::new(&content);
    simulate_guard(&lab, start_pos, start_dir)
}

pub fn part2(filename: &str) -> i32 {
    let content = std::fs::read_to_string(filename).expect("Failed to read file");
    let (lab, start_pos, start_dir) = Lab::new(&content);

    let mut valid_positions = 0;

    // Check every position in the grid
    for row in 0..lab.height {
        for col in 0..lab.width {
            let pos = (row, col);
            // Skip start position and existing obstacles
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
