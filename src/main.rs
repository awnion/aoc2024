mod day1;

fn main() {
    let day1_input = include_str!("../inputs/day1.txt");
    println!("Day 1 - Part 1: {}", day1::part1_solution(day1_input));
    println!("Day 1 - Part 2: {}", day1::part2_solution(day1_input));
}
