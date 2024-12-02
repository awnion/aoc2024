use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

pub fn read_input(input: &str) -> (Vec<i128>, Vec<i128>) {
    let nums = input.split_ascii_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<_>>();

    (
        nums.iter().skip(0).step_by(2).cloned().collect(),
        nums.iter().skip(1).step_by(2).cloned().collect(),
    )
}

pub fn part1_solution(input: &str) -> u128 {
    let (list1, list2) = {
        let (mut a, mut b) = read_input(input);
        a.sort_unstable();
        b.sort_unstable();
        (a, b)
    };

    list1.iter().zip(&list2).par_bridge().into_par_iter().map(|(&x, &y)| x.abs_diff(y)).sum()
}

pub fn part2_solution(input: &str) -> i128 {
    let (list1, list2) = read_input(input);

    let counter = Arc::new(Mutex::new(HashMap::new()));

    list2.par_iter().for_each(|&item| {
        counter.lock().unwrap().entry(item).and_modify(|v| *v += 1).or_insert(1);
    });

    list1.par_iter().map(|item| item * counter.lock().unwrap().get(item).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solution(INPUT), 11u128);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solution(INPUT), 31i128);
    }
}
