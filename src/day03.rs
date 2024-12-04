use regex::Regex;

use crate::day::Day;

thread_local! {
    static RE: Regex = Regex::new(r"^\((\d+),(\d+)\)").unwrap();
}

pub struct Day03;

impl Day for Day03 {
    fn parts() -> Vec<Box<dyn Fn() -> String>> {
        let input = include_str!("../inputs/day03.txt");
        vec![Box::new(|| part1_solution(input)), Box::new(|| part2_solution(input))]
    }
}

fn sum_of_muls(input: &str) -> i64 {
    input
        .split("mul")
        .filter_map(|s| {
            RE.with(|re| {
                re.captures(s).map(|c| c.extract()).map(|(_, [a, b])| {
                    match (a.parse::<i64>(), b.parse::<i64>()) {
                        (Ok(a), Ok(b)) => a * b,
                        _ => panic!("Error parsing mul expression"),
                    }
                })
            })
        })
        .sum::<i64>()
}

pub fn part1_solution(input: &str) -> String {
    sum_of_muls(input).to_string()
}

pub fn part2_solution(input: &str) -> String {
    input
        .split("do()")
        .filter_map(|s| s.split("don't()").next())
        .map(sum_of_muls)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const INPUT2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        ";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solution(INPUT), "161");
    }

    #[test]
    fn part1_test_input() {
        let input = include_str!("../inputs/day03.txt");
        assert_eq!(part1_solution(input), "189600467");
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solution(INPUT2), "48");
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("../inputs/day03.txt");
        assert_eq!(part2_solution(input), "107069718");
    }
}
