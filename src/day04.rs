use crate::day::Day;

pub struct Day04;

impl Day for Day04 {
    fn parts() -> Vec<Box<dyn Fn() -> String>> {
        let input = include_str!("../inputs/day04.txt");
        vec![Box::new(|| part1_solution(input)), Box::new(|| part2_solution(input))]
    }
}

pub fn part1_solution(input: &str) -> String {
    let map = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let n = map.len();
    let m = map.first().expect("first line is not empty").len();
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            match (j + 3 < m, i + 3 < n) {
                (true, true) => {
                    match (map[i][j], map[i + 1][j + 1], map[i + 2][j + 2], map[i + 3][j + 3]) {
                        (b'X', b'M', b'A', b'S') => count += 1,
                        (b'S', b'A', b'M', b'X') => count += 1,
                        _ => {}
                    }
                    match (map[i + 3][j], map[i + 2][j + 1], map[i + 1][j + 2], map[i][j + 3]) {
                        (b'X', b'M', b'A', b'S') => count += 1,
                        (b'S', b'A', b'M', b'X') => count += 1,
                        _ => {}
                    }
                    match (map[i][j], map[i][j + 1], map[i][j + 2], map[i][j + 3]) {
                        (b'X', b'M', b'A', b'S') => count += 1,
                        (b'S', b'A', b'M', b'X') => count += 1,
                        _ => {}
                    }
                    match (map[i][j], map[i + 1][j], map[i + 2][j], map[i + 3][j]) {
                        (b'X', b'M', b'A', b'S') => count += 1,
                        (b'S', b'A', b'M', b'X') => count += 1,
                        _ => {}
                    }
                }
                (true, false) => match (map[i][j], map[i][j + 1], map[i][j + 2], map[i][j + 3]) {
                    (b'X', b'M', b'A', b'S') => count += 1,
                    (b'S', b'A', b'M', b'X') => count += 1,
                    _ => {}
                },
                (false, true) => match (map[i][j], map[i + 1][j], map[i + 2][j], map[i + 3][j]) {
                    (b'X', b'M', b'A', b'S') => count += 1,
                    (b'S', b'A', b'M', b'X') => count += 1,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    count.to_string()
}

pub fn part2_solution(input: &str) -> String {
    let map = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let n = map.len();
    let m = map.first().expect("first line is not empty").len();
    let mut count = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if map[i][j] == b'A' {
                match (map[i - 1][j - 1], map[i - 1][j + 1], map[i + 1][j + 1], map[i + 1][j - 1]) {
                    (b'M', b'M', b'S', b'S') => count += 1,
                    (b'S', b'M', b'M', b'S') => count += 1,
                    (b'S', b'S', b'M', b'M') => count += 1,
                    (b'M', b'S', b'S', b'M') => count += 1,
                    _ => {}
                }
            }
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_test() {
        assert_eq!(part1_solution(INPUT), "18");
    }

    #[test]
    fn part1_test_input() {
        let input = include_str!("../inputs/day04.txt");
        assert_eq!(part1_solution(input), "2642");
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2_solution(INPUT), "9");
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("../inputs/day04.txt");
        assert_eq!(part2_solution(input), "1974");
    }
}
