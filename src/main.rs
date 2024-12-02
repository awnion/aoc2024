use aoc2024::*;

macro_rules! elapsed {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let res = $e;
        format!("{:>10.0?} {:>15}", start.elapsed(), res)
    }};
}

fn main() {
    let days = [
        //
        aoc2024::Day01::parts,
        aoc2024::Day02::parts,
    ];

    for (day, parts_fn) in days.iter().enumerate() {
        println!("Day {}", day + 1);
        for (part, part_fn) in parts_fn().iter().enumerate() {
            println!("{:>12} {}: {}", "Part", part + 1, elapsed!(part_fn()));
        }
    }
}
