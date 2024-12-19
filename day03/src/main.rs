use regex::Regex;
fn solve_part1(input: &str) -> u32 {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let num_regex = Regex::new(r"[0-9]{1,3}").unwrap();
    regex
        .find_iter(input)
        .map(|mul| {
            num_regex
                .find_iter(mul.as_str())
                .map(|nums| nums.as_str().parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum::<u32>()
}

fn solve_part2(input: &str) -> u32 {
    let regex = Regex::new(r"(don't\(\))|(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))").unwrap();
    let num_regex = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut mul_enabled = true;
    regex
        .find_iter(input)
        .map(|needle| match needle.as_str() {
            "do()" => {
                mul_enabled = true;
                0
            }
            "don't()" => {
                mul_enabled = false;
                0
            }
            _ if mul_enabled => num_regex
                .find_iter(needle.as_str())
                .map(|needle| needle.as_str().parse::<u32>().unwrap())
                .product::<u32>(),
            _ => return 0,
        })
        .sum::<u32>()
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
