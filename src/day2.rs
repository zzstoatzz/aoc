use std::fs::read_to_string;

fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let content = read_to_string(filename).expect("Failed to read file");
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    // Check first two numbers to determine if we expect increasing or decreasing
    let increasing = levels[1] > levels[0];

    // Check each adjacent pair
    for i in 0..levels.len() - 1 {
        let diff = levels[i + 1] - levels[i];

        // Check if difference is between 1 and 3 (inclusive)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Check if direction matches our expected direction
        if increasing && diff < 0 || !increasing && diff > 0 {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    // If it's already safe, no need to try removing numbers
    if is_safe_report(levels) {
        return true;
    }

    // Try removing each number one at a time
    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_safe_report(&modified) {
            return true;
        }
    }

    false
}

pub fn part1(filename: &str) -> i32 {
    let reports = read_input(filename);
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count() as i32
}

pub fn part2(filename: &str) -> i32 {
    let reports = read_input(filename);
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count() as i32
}
