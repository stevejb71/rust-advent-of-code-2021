use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day19.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(inputs: &Vec<&str>) -> u64 {
  0
}

pub fn part2(inputs: &Vec<&str>) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample() -> Vec<&'static str> {
    vec_of_str(include_str!("../test_inputs/day19.txt"))
  }
}