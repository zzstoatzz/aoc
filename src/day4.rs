use std::fs::read_to_string;

fn find_xmas(grid: &[Vec<char>]) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // All possible directions to search: right, down-right, down, down-left,
    // left, up-left, up, up-right
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for row in 0..height {
        for col in 0..width {
            // Try each direction from this starting position
            for &(dy, dx) in &directions {
                let mut valid = true;
                let word = "XMAS";

                // Check if we can fit the word in this direction
                for (i, target) in word.chars().enumerate() {
                    let new_row = row as i32 + dy * i as i32;
                    let new_col = col as i32 + dx * i as i32;

                    if new_row < 0
                        || new_row >= height as i32
                        || new_col < 0
                        || new_col >= width as i32
                    {
                        valid = false;
                        break;
                    }

                    if grid[new_row as usize][new_col as usize] != target {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_x_mas(grid: &[Vec<char>]) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Check each possible center point of the X
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            // The center must be 'A'
            if grid[row][col] != 'A' {
                continue;
            }

            // Check all possible combinations of MAS in the X shape
            let diagonals = [
                // top-left to bottom-right, top-right to bottom-left
                (
                    (row - 1, col - 1),
                    (row + 1, col + 1),
                    (row - 1, col + 1),
                    (row + 1, col - 1),
                ),
            ];

            for &(tl, br, tr, bl) in &diagonals {
                // Check if we can form MAS in both diagonals (in either direction)
                let top_left = [grid[tl.0][tl.1], grid[row][col], grid[br.0][br.1]];
                let top_right = [grid[tr.0][tr.1], grid[row][col], grid[bl.0][bl.1]];

                // Check all combinations of forward/backward MAS
                let is_mas = |chars: &[char]| chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M'];

                if is_mas(&top_left) && is_mas(&top_right) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part1(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    find_xmas(&grid)
}

pub fn part2(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    find_x_mas(&grid)
}
