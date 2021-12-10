use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day10.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(lines: &Vec<&str>) -> u64 {
  0
}

pub fn part2(_lines: &Vec<&str>) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_find_the_syntax_error_score() {

  }

  fn sample() -> Vec<&str> {
    let lines = include_str!("../test_inputs/day10.txt");
    vec_of_str(&lines)    
  }
}