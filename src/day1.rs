use crate::common::*;

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day1.txt");
  run_part(part, input_lines, vec_of_usize, part1, part2)
}

pub fn part1(heights: &Vec<usize>) -> usize {
  count_ascents(heights, 2)
}

pub fn part2(heights: &Vec<usize>) -> usize {
  count_ascents(heights, 4)
}

fn count_ascents(heights: &Vec<usize>, window_size: usize) -> usize {
  heights.windows(window_size).filter(|w| w[0] < w[window_size - 1]).count()
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

  fn sample() -> Vec<usize> {
    vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
  }
}
