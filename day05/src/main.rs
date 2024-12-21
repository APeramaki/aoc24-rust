use std::collections::{HashMap, HashSet};

fn solve_part1(input: &str) -> u32 {
    let mut ordering_rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut rows = input.lines();
    let regex_rules = Regex::new(r"[0-9]{2}").unwrap();
    for row in &mut rows {
        let mut numbers = regex_rules.find_iter(row);
        
        match &numbers.next() {
            Some(x) => ordering_rules
                .entry(x.as_str().parse::<usize>().unwrap())
                                        .or_insert(HashSet::new())
                                        .insert(numbers.next().unwrap().as_str().parse::<usize>().unwrap()),
            None => break,
        };
    }
    let mut sum: usize = 0;
    for row in rows {
        let page_nums: Vec<_> = row
            .split(',')
            .map(|num| num.trim().parse::<usize>().unwrap())
            .collect();
        let order_right = !page_nums.iter().enumerate().any(|(index, num)| {
                let binding = HashSet::new();
                let page_ruleset = ordering_rules.get(&num).unwrap_or(&binding);
            page_nums[..index]
                .iter()
                .any(|previous| page_ruleset.contains(previous))
            });
        match order_right {
            true => sum += page_nums.get((page_nums.len() - 1) / 2).unwrap_or(&0),
            false => {}
        }
    }
    sum as u32
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

