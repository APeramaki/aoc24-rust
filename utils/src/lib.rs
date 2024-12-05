pub fn parse_number_pairs(input: &str) -> Vec<(i32, i32)> {
    input
      .lines()
      .map(|l| {
         let mut nums = l
               .split_whitespace()
               .filter_map(|num| num.parse::<i32>().ok());

         (nums.next().unwrap_or(0), nums.next().unwrap_or(0))
      })
      .collect()
}

pub fn parse_number_lists(input: &str) -> Vec<Vec<i32>> {
   let t: Vec<Vec<i32>> = input
      .lines()
      .map(|line| {
         line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("failed to parse number"))
            .collect()
      })
      .collect();
   t
}
