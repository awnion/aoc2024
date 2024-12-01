use std::collections::HashMap;

pub fn read_input(input: &str) -> (Vec<i128>, Vec<i128>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        match line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i128>>()
            .as_slice()
        {
            &[num1, num2] => {
                list1.push(num1);
                list2.push(num2);
            }
            _ => break,
        }
    }
    (list1, list2)
}

pub fn part1_solution(input: &str) -> u128 {
    let (list1, list2) = {
        let (mut a, mut b) = read_input(input);
        a.sort();
        b.sort();
        (a, b)
    };

    list1.iter().zip(&list2).map(|(&x, &y)| x.abs_diff(y)).sum()
}

pub fn part2_solution(input: &str) -> i128 {
    let (list1, list2) = read_input(input);

    let mut counter = HashMap::new();

    for item in list2 {
        counter.entry(item).and_modify(|v| *v += 1).or_insert(1);
    }

    list1
        .iter()
        .map(|item| item * counter.get(item).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1_solution(
                "\
3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11u128
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2_solution(
                "\
3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31i128
        );
    }
}
