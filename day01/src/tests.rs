use super::*;
#[test]
fn part1_distance_score() {
    let example_input = "
      3   4
      4   3
      2   5
      1   3
      3   9
      3   3";
    assert_eq!(solve_part1(example_input), 11);
}
#[test]
fn part2_similarity_score() {
    let example_input = "
      3   4
      4   3
      2   5
      1   3
      3   9
      3   3";
    assert_eq!(solve_part2(example_input), 31);
}
