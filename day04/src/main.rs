const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const XMAS: &str = "MAS";

fn solve_part1(input: &str) -> u32 {
    let rows = input.lines().collect::<Vec<_>>();

    let x_locations: Vec<_> = rows
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.match_indices('X')
                .map(|loc| (row_index, loc.0))
                .collect::<Vec<_>>()
        })
        .flat_map(|inner_vec| inner_vec.into_iter())
        .collect();

    x_locations
        .iter()
        .map(|location| {
            DIRECTIONS
                .iter()
                .map(|dir| {
                    XMAS.chars().enumerate().all(|(i, letter)| {
                        let check = (
                            location.0 as isize + (i + 1) as isize * dir.0,
                            location.1 as isize + (i + 1) as isize * dir.1,
                        );

                        rows.iter()
                            .nth(check.0 as usize)
                            .unwrap_or(&&".")
                            .chars()
                            .nth(check.1 as usize)
                            .unwrap_or('.')
                            .eq(&letter)
                    })
                })
                .collect::<Vec<bool>>()
        })
        .flat_map(|inner| inner)
        .filter(|b| *b)
        .count() as u32
}

fn solve_part2(input: &str) -> u32 {
    todo!();
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!(
        "Part 1 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );

    let now = Instant::now();
    let result = solve_part2(input);
    println!(
        "Part 2 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );
}

#[cfg(test)]
mod tests;
