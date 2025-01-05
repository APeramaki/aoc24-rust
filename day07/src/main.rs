mod calibration;
use calibration::Calibration;
use rayon::prelude::*;

fn solve_part1(input: &str) -> u64 {
    let calibrations: Vec<Calibration> = input
        .lines()
        .map(|line| Calibration::new(line, false))
        .collect();
    calibrations
        .par_iter()
        .filter_map(|c| c.test_all())
        .sum()
    
}

fn solve_part2(input: &str) -> u64 {
    let calibrations: Vec<Calibration> = input
        .lines()
        .map(|line| Calibration::new(line, true))
        .collect();
    calibrations
        .par_iter()
        .filter_map(|c| c.test_all())
        .sum()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("Part 1 solution: {}, time taken {:.2?}", result, now.elapsed());

    let now = Instant::now();

    let input = include_str!("../input.txt");
    let result = solve_part2(input);
    println!("Part 2 solution: {}, time taken {:.2?}", result, now.elapsed());
}


#[cfg(test)]
mod tests;
