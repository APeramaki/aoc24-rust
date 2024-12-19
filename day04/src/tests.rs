use super::*;
#[test]
fn part1_num_of_xmas() {
    let example_input =
    "
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX";
    assert_eq!(solve_part1(example_input), 18);
}

#[test]
fn part2_num_of_x_mas() {
  let example_input =
  "
  MMMSXXMASM
  MSAMXMSMSA
  AMXSXMAAMM
  MSAMASMSMX
  XMASAMXAMM
  XXAMMXXAMA
  SMSMSASXSS
  SAXAMASAAA
  MAMMMXMMMM
  MXMXAXMASX";
  assert_eq!(solve_part2(example_input), 9);
}