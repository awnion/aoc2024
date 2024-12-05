use std::collections::HashSet;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::day::Day;

pub struct Day05;

impl Day for Day05 {
    fn parts() -> Vec<Box<dyn Fn() -> String>> {
        let input = include_str!("../inputs/day05.txt");
        vec![
            Box::new(|| part1_solution(input).to_string()),
            Box::new(|| part2_solution(input).to_string()),
        ]
    }
}

fn read_input(input: &str) -> (HashSet<(u64, u64)>, Vec<Vec<u64>>) {
    let mut lines = input.lines();
    (
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut it = line.split('|').filter_map(|s| s.parse::<u64>().ok());
                (it.next().unwrap(), it.next().unwrap())
            })
            .collect(),
        lines.map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect()).collect(),
    )
}

pub fn part1_solution(input: &str) -> String {
    let (ordering, books) = read_input(input);

    books
        .par_iter()
        .filter(|&book| book.is_sorted_by(|&a, &b| !ordering.contains(&(b, a))))
        .map(|book| book[book.len() / 2])
        .sum::<u64>()
        .to_string()
}

pub fn part2_solution(input: &str) -> String {
    let (ordering, mut books) = read_input(input);

    books
        .par_iter_mut()
        .filter(|book| !book.is_sorted_by(|&a, &b| !ordering.contains(&(b, a))))
        .map(|book| {
            book.sort_by(|&a, &b| ordering.contains(&(b, a)).cmp(&true));
            book[book.len() / 2]
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solution(TEST_INPUT.trim()), "143");
    }

    #[test]
    fn part1_test_input() {
        let input = include_str!("../inputs/day05.txt");
        assert_eq!(part1_solution(input), "6051");
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solution(TEST_INPUT.trim()), "123");
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("../inputs/day05.txt");
        assert_eq!(part2_solution(input), "5093");
    }
}
