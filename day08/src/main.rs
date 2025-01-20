use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solve_part1(input: &str) -> u64 {
    let mut radio_emitters: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let mut map_size: (i32, i32) = (0, 0);
    input.lines().enumerate().for_each(|(row, line)| {
        map_size.1 = row as i32;
        line.chars().enumerate().for_each(|(column, char)| {
            map_size.0 = column as i32;
            if char != '.' {
                radio_emitters
                    .entry(char)
                    .or_default()
                    .insert((row as i32, column as i32));
            }
        });
    });

    let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
    radio_emitters.iter().for_each(|(_, locations)| {
        locations.iter().combinations(2).for_each(|locations| {
            let (x1, y1) = locations.first().unwrap();
            let (x2, y2) = locations.get(1).unwrap();
            let diff: (i32, i32) = (x1 - x2, y1 - y2);

            let (possible_x, possible_y) = (x1 + diff.0, y1 + diff.1);
            if possible_x >= 0
                && possible_x <= map_size.0
                && possible_y >= 0
                && possible_y <= map_size.1
            {
                unique_locations.insert((possible_x, possible_y));
            }

            let (possible_x, possible_y) = (x2 - diff.0, y2 - diff.1);
            if possible_x >= 0
                && possible_x <= map_size.0
                && possible_y >= 0
                && possible_y <= map_size.1
            {
                unique_locations.insert((possible_x, possible_y));
            }
        });
    });

    unique_locations.len() as u64
}

fn solve_part2(input: &str) -> u64 {
    let mut radio_emitters: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let mut map_size: (i32, i32) = (0, 0);
    input.lines().enumerate().for_each(|(row, line)| {
        map_size.1 = row as i32;
        line.chars().enumerate().for_each(|(column, char)| {
            map_size.0 = column as i32;
            if char != '.' {
                radio_emitters
                    .entry(char)
                    .or_default()
                    .insert((row as i32, column as i32));
            }
        });
    });

    let mut unique_locations: HashSet<(i32, i32)> = HashSet::new();
    radio_emitters.iter().for_each(|(_, locations)| {
        locations.iter().combinations(2).for_each(|locations| {
            let (x1, y1) = locations.first().unwrap();
            let (x2, y2) = locations.get(1).unwrap();
            let diff: (i32, i32) = (x1 - x2, y1 - y2);
            let mut possible = (*x1, *y1);

            while possible.0 >= 0
                && possible.0 <= map_size.0
                && possible.1 >= 0
                && possible.1 <= map_size.1
            {
                unique_locations.insert((possible.0, possible.1));
                possible = (possible.0 + diff.0, possible.1 + diff.1);
            }

            let mut possible = (*x2, *y2);
            while possible.0 >= 0
                && possible.0 <= map_size.0
                && possible.1 >= 0
                && possible.1 <= map_size.1
            {
                unique_locations.insert((possible.0, possible.1));
                possible = (possible.0 - diff.0, possible.1 - diff.1);
            }
        });
    });

    unique_locations.len() as u64
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/day08.txt").expect("Failed to read input");
    let result = solve_part1(&input);
    println!(
        "Part 1 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );
    let now = Instant::now();

    let result = solve_part2(&input);
    println!(
        "Part 2 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );
}

#[cfg(test)]
mod tests;
