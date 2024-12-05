use std::collections::HashMap;
use std::collections::HashSet;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::day::Day;

pub struct Day05;

impl Day for Day05 {
    fn parts() -> Vec<Box<dyn Fn() -> String>> {
        let input = include_str!("../inputs/day05.txt");
        vec![Box::new(|| part1_solution(input)), Box::new(|| part2_solution(input))]
    }
}

fn read_input(input: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let mut lines = input.lines().into_iter();

    // read page map
    let mut page_map: HashMap<u64, HashSet<u64>> = HashMap::new();

    lines.by_ref().take_while(|line| !line.trim().is_empty()).for_each(|line| {
        let mut iter = line.split('|').filter_map(|s| s.parse::<u64>().ok());
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            page_map.entry(a).or_default().insert(b);
        }
    });

    // read books
    let pages = lines.map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect()).collect();

    (page_map, pages)
}

fn is_correct(book: &[u64], page_map: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut visited = HashSet::new();
    for &page in book.iter() {
        if let Some(pages_should_be_after) = page_map.get(&page) {
            if visited.intersection(pages_should_be_after).count() > 0 {
                return false;
            }
        }
        visited.insert(page);
    }
    true
}

pub fn part1_solution(input: &str) -> String {
    let (page_map, books) = read_input(input);

    books
        .par_iter()
        .filter(|book| is_correct(book, &page_map))
        .map(|book| book[book.len() / 2])
        .sum::<u64>()
        .to_string()
}

pub fn part2_solution(input: &str) -> String {
    let (page_map, books) = read_input(input);

    books
        .par_iter()
        .filter(|book| !is_correct(book, &page_map))
        .map(|book| {
            let mut new_book = book.to_vec();
            let mut i = 0;
            'w: while i < new_book.len() {
                for j in (i + 1..new_book.len()).rev() {
                    if let Some(set) = page_map.get(&new_book[j]) {
                        if set.contains(&new_book[i]) {
                            new_book[i..=j].rotate_left(1);
                            continue 'w;
                        }
                    }
                }
                i += 1;
            }
            new_book[new_book.len() / 2]
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
