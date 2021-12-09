use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day1.txt");
  run_part(part, input_lines, vec_of_u64, part1, part2)
}

pub fn part1(heights: &Vec<u64>) -> u64 {
  count_ascents(heights, 2)
}

pub fn part2(heights: &Vec<u64>) -> u64 {
  count_ascents(heights, 4)
}

fn count_ascents(heights: &Vec<u64>, window_size: usize) -> u64 {
  heights.windows(window_size).filter(|w| w[0] < w[window_size - 1]).count() as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn counts_heights_greater_than_the_preceeding() {
    let heights = sample();
    assert_eq!(7, part1(&heights));
  }

  #[test]
  fn counts_heights_of_windows_of_3_greater_than_the_preceeding() {
    let heights = sample();
    assert_eq!(5, part2(&heights));
  }

  fn sample() -> Vec<u64> {
    vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
  }
}
