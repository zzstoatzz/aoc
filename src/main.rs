mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
fn main() {
    println!("Day 1:");
    println!("  Part 1: {}", day1::part1("day1.txt"));
    println!("  Part 2: {}", day1::part2("day1.txt"));

    println!("Day 2:");
    println!("  Part 1: {}", day2::part1("day2.txt"));
    println!("  Part 2: {}", day2::part2("day2.txt"));

    println!("Day 3:");
    println!("  Part 1: {}", day3::part1("day3.txt"));
    println!("  Part 2: {}", day3::part2("day3.txt"));

    println!("Day 4:");
    println!("  Part 1: {}", day4::part1("day4.txt"));
    println!("  Part 2: {}", day4::part2("day4.txt"));

    println!("Day 5:");
    println!("  Part 1: {}", day5::part1("day5.txt"));
    println!("  Part 2: {}", day5::part2("day5.txt"));

    println!("Day 6:");
    println!("  Part 1: {}", day6::part1("day6.txt"));
    println!("  Part 2: {}", day6::part2("day6.txt"));
}
