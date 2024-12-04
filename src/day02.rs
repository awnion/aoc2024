use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

use crate::day::Day;

pub struct Day02;

impl Day for Day02 {
    fn parts() -> Vec<Box<dyn Fn() -> String>> {
        let input = include_str!("../inputs/day02.txt");
        vec![Box::new(|| part1_solution(input)), Box::new(|| part2_solution(input))]
    }
}

pub fn read_input(input: &str) -> impl ParallelIterator<Item = Vec<u64>> + '_ {
    input.par_lines().map(|line| -> Vec<u64> {
        line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>()
    })
}

fn is_safe<'a>(mut it: impl Iterator<Item = &'a u64>) -> bool {
    let mut x = {
        let Some(&x) = it.next() else { return true };
        x
    };
    for &y in it {
        if y > x + 3 || y <= x {
            return false;
        }
        x = y;
    }
    true
}

pub fn part1_solution(input: &str) -> String {
    fn check(list: &[u64]) -> u64 {
        if is_safe(list.iter()) || is_safe(list.iter().rev()) {
            1
        } else {
            0
        }
    }
    read_input(input).map(|list| check(&list)).sum::<u64>().to_string()
}

pub fn part2_solution(input: &str) -> String {
    fn check(list: &mut [u64]) -> u64 {
        for i in 0..list.len() {
            let iter = list.iter().take(i).chain(list.iter().skip(i + 1));
            if is_safe(iter.clone()) || is_safe(iter.rev()) {
                return 1;
            }
        }
        0
    }
    read_input(input).map(|mut list| check(&mut list)).sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solution(INPUT), "2");
    }

    #[test]
    fn part1_test_input() {
        let input = include_str!("../inputs/day02.txt");
        assert_eq!(part1_solution(input), "624");
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solution(INPUT), "4");
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("../inputs/day02.txt");
        assert_eq!(part2_solution(input), "658");
    }
}
