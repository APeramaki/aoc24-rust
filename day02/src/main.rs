use utils::parse_number_lists;
fn is_unsafe(reading: &Vec<i32>) -> Option<()> {
    if reading.len() < 2 {
        println!("reading len < 2");
        return None;
    }
    let is_rising = reading[0] < reading[1];
    if reading.windows(2)
        .any(|pair|
            (pair[0] < pair[1]) != is_rising ||
            (pair[0] == pair[1] ) ||
            (pair[0] - pair[1]).abs() > 3) {
                return None;
        }
    return Some(());
}
pub fn solve_part1(input: &str) -> i32{
    let num_vectors = parse_number_lists(input);
    num_vectors
        .iter()
        .filter_map(|readings| is_unsafe(readings))
        .fold(0, |acc, _f| acc + 1 )
    
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("Part 1 solution: {}, time taken {:.2?}", result, now.elapsed());
}


#[cfg(test)]
mod tests;