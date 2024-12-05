use std::io;
use std::io::Write;

use aoc2024::Day;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

const PARALLEL: bool = false;

macro_rules! elapsed {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let res = $e;
        format!("{:>10.0?} {:>15}", start.elapsed(), res)
    }};
}

fn main() {
    let days = [
        aoc2024::Day01::parts,
        aoc2024::Day02::parts,
        aoc2024::Day03::parts,
        aoc2024::Day04::parts,
        aoc2024::Day05::parts,
    ];

    let handler_day = |day, parts: Vec<Box<dyn Fn() -> String>>| {
        let mut res = Vec::new();
        writeln!(&mut res, "Day {}", day + 1).unwrap();
        for (part, part_fn) in parts.iter().enumerate() {
            writeln!(&mut res, "{:>12} {}: {}", "Part", part + 1, elapsed!(part_fn())).unwrap();
        }
        res
    };
    let res: Vec<Vec<u8>> = {
        if PARALLEL {
            days.par_iter().enumerate().map(|(day, parts)| handler_day(day, parts())).collect()
        } else {
            days.iter().enumerate().map(|(day, parts)| handler_day(day, parts())).collect()
        }
    };

    for s in res {
        io::stdout().write_all(&s).unwrap();
    }
}
