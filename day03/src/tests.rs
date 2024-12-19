use super::*;
#[test]
fn part1_safe_count() {
    let example_input =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(solve_part1(example_input), 161);
}

#[test]
fn part2_safe_count() {
  let example_input =
  "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
  assert_eq!(solve_part2(example_input),48);
}