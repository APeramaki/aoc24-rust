// I'm not proud
use std::collections::HashSet;

const DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn solve_part1(input: &str) -> usize {
    let mut guard_location = (0, 0);
    let mut i: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row_number, line)| {
            line.chars()
                .enumerate()
                .map(|(x_loc, c)| {
                    match c {
                        c if c == '^' => guard_location = (row_number, x_loc),
                        _ => {}
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut current_direction = 0;
    let mut changed_tiles = 0;
    loop {
        i[guard_location.0] = i
            .get(guard_location.0)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(x_loc, c)| {
                if x_loc != guard_location.1 {
                    return *c;
                }
                match c {
                    'X' => return *c,
                    _ => {
                        changed_tiles += 1;
                        return 'X';
                    }
                }
            })
            .collect();

        if let Some(row) =
            i.get((guard_location.0 as i32 + DIRECTION[current_direction].0) as usize)
        {
            if let Some(x) =
                row.get((guard_location.1 as i32 + DIRECTION[current_direction].1) as usize)
            {
                if x == &'#' {
                    current_direction = (current_direction + 1) % 4;
                } else {
                    guard_location = (
                        ((guard_location.0 as i32 + DIRECTION[current_direction].0) as usize),
                        ((guard_location.1 as i32 + DIRECTION[current_direction].1) as usize),
                    );
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    changed_tiles
}


fn solve_part2(input: &str) -> usize {
    let mut guard_location = (0, 0);
    let i: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row_number, line)| {
            line.chars()
                .enumerate()
                .map(|(x_loc, c)| {
                    match c {
                        c if c == '^' => guard_location = (row_number, x_loc),
                        _ => {}
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut current_direction = 0;
    let mut count = 0;

    // Didn't end up using parallelism.
    // let mut is_foreverloop: Arc<Mutex<HashMap<(usize, usize), bool>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    checked.insert(guard_location);
    loop {

        if let Some(row) =
            i.get((guard_location.0 as i32 + DIRECTION[current_direction].0) as usize)
        {
            if let Some(x) =
                row.get((guard_location.1 as i32 + DIRECTION[current_direction].1) as usize)
            {
                if x == &'#' {
                    current_direction = (current_direction + 1) % 4;
                } else {
                    
                    // data.push((guard_location, current_direction));
                    let next_step = (
                        ((guard_location.0 as i32 + DIRECTION[current_direction].0) as usize),
                        ((guard_location.1 as i32 + DIRECTION[current_direction].1) as usize),
                    );
                    
                    if checked.insert(next_step) && check_for_loop(guard_location, current_direction, next_step, i.clone() ) {
                        count += 1;
                    }
                    guard_location = next_step;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    count
}


fn check_for_loop(start_loc: (usize, usize), dir: usize, added_wall: (usize, usize), mut map: Vec<Vec<char>>) -> bool {
    if let Some(row) = map.get_mut(added_wall.0){
        if let Some(cell) = row.get_mut(added_wall.1) {
            *cell = '#';
        }
    }
    let mut guard_location = start_loc;
    let mut current_direction = dir;

    // I'd like to do this properly tomorrow, next wekk or preferrably yesterday.
    // I likely won't
    for _ in 0..6500  {
        if let Some(row) =
            map.get((guard_location.0 as i32 + DIRECTION[current_direction].0) as usize)
        {
            if let Some(x) =
                row.get((guard_location.1 as i32 + DIRECTION[current_direction].1) as usize)
            {
                if x == &'#' {
                    current_direction = (current_direction + 1) % 4;
                } else {
                    let next_step = (
                        ((guard_location.0 as i32 + DIRECTION[current_direction].0) as usize),
                        ((guard_location.1 as i32 + DIRECTION[current_direction].1) as usize),
                    );
                    guard_location = next_step;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

// for debug purposes
fn _print_map(i: &Vec<Vec<char>>) {
    for row in i {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
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
