use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug)]
struct PrintJob {
    rules: Vec<(i32, i32)>, // (before, after) pairs
    updates: Vec<Vec<i32>>, // List of updates, each with page numbers
}

fn parse_input(content: &str) -> PrintJob {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut reading_rules = true;

    for line in content.lines() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            // Parse rule like "47|53"
            let parts: Vec<&str> = line.split('|').collect();
            let before = parts[0].parse().unwrap();
            let after = parts[1].parse().unwrap();
            rules.push((before, after));
        } else {
            // Parse update like "75,47,61,53,29"
            let pages: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            updates.push(pages);
        }
    }

    PrintJob { rules, updates }
}

fn is_valid_order(pages: &[i32], rules: &[(i32, i32)]) -> bool {
    // Create a set of rules that apply to these pages
    let page_set: HashSet<_> = pages.iter().copied().collect();
    let relevant_rules: Vec<_> = rules
        .iter()
        .filter(|&(before, after)| page_set.contains(before) && page_set.contains(after))
        .collect();

    // Check each rule
    for &(before, after) in &relevant_rules {
        let before_pos = pages.iter().position(|&x| x == *before).unwrap();
        let after_pos = pages.iter().position(|&x| x == *after).unwrap();

        if before_pos > after_pos {
            return false;
        }
    }

    true
}

fn sort_pages(pages: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let page_set: HashSet<_> = pages.iter().copied().collect();

    // Build adjacency list and in-degree count
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();

    // Initialize for all pages
    for &page in pages {
        graph.entry(page).or_default();
        in_degree.insert(page, 0);
    }

    // Add edges from relevant rules
    for &(before, after) in rules {
        if page_set.contains(&before) && page_set.contains(&after) {
            graph.entry(before).or_default().push(after);
            *in_degree.entry(after).or_default() += 1;
        }
    }

    // Kahn's algorithm for topological sort
    let mut result = Vec::new();
    let mut queue: Vec<_> = in_degree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop() {
        result.push(page);

        if let Some(neighbors) = graph.get(&page) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push(next);
                }
            }
        }
    }

    result
}

fn get_middle_page(pages: &[i32]) -> i32 {
    pages[pages.len() / 2]
}

pub fn part1(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let job = parse_input(&content);

    job.updates
        .iter()
        .filter(|update| is_valid_order(update, &job.rules))
        .map(|update| get_middle_page(update))
        .sum()
}

pub fn part2(filename: &str) -> i32 {
    let content = read_to_string(filename).expect("Failed to read file");
    let job = parse_input(&content);

    job.updates
        .iter()
        .filter(|update| !is_valid_order(update, &job.rules))
        .map(|update| {
            let sorted = sort_pages(update, &job.rules);
            get_middle_page(&sorted)
        })
        .sum()
}
