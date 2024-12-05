use std::collections::HashMap;

use utils::parse_number_pairs;

pub fn solve_part1(input: &str) -> i32 {
    let paired = parse_number_pairs(input);
    let (mut a, mut b): (Vec<_>, Vec<_>) = paired.into_iter().unzip();
    a.sort();
    b.sort();
    a.into_iter()
        .zip(b)
        .map(|(aa, bb)| (aa - bb).abs())
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    let paired = parse_number_pairs(input);
    let ( a, b): (Vec<_>, Vec<_>) = paired.into_iter().unzip();
    let mut hashmap: HashMap<i32, (i32, i32)> = HashMap::new();

    a.iter().for_each(|v| {
        hashmap
            .entry(*v)
            .and_modify(|data| data.0 += 1)
            .or_insert((1, 0));
    });
    b.iter().for_each(|v| { 
        hashmap
            .entry(*v)
            .and_modify(|data| data.1 += 1);
    });
    hashmap.into_iter().fold(0,|acc, (key, value)|{
        acc + key * value.0 * value.1
    })

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