use super::*;
#[test]
fn part1_safe_count() {
    let example_input =
    " 7 6 4 2 1
      1 2 7 8 9
      9 7 6 2 1
      1 3 2 4 5
      8 6 4 4 1
      1 3 6 7 9";
    assert_eq!(solve_part1(example_input), 2);
}

#[test]
fn part2_safe_count() {
  let example_input =
  " 7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
  assert_eq!(solve_part2(example_input), 4);
}