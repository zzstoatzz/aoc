use itertools::Itertools;
use std::fs::read_to_string;

fn read_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let content = read_to_string(filename).expect("Failed to read file");

    content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split_whitespace();
            let first = nums.next().unwrap().parse::<i32>().unwrap();
            let second = nums.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip()
}

pub fn part1(filename: &str) -> i32 {
    let (mut a, mut b) = read_input(filename);

    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .map(|(a, b): (&i32, &i32)| (a - b).abs())
        .sum()
}

pub fn part2(filename: &str) -> i32 {
    // Count how often each number from the right list appears.
    // For each number in the left list, multiply that number by how many times it appears in the right list.
    // Sum all these products to get the similarity score.
    let (a, b) = read_input(filename);

    let b_counts = b.iter().counts();

    a.iter()
        .map(|&a| a * (*b_counts.get(&a).unwrap_or(&0) as i32))
        .sum()
}
