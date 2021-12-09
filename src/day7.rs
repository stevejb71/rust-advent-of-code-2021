use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day7.txt");
  run_part(part, input_lines, vec_of_u64_one_line, part1, part2)
}

pub fn part1(xs: &Vec<u64>) -> u64 {
  sum_map(xs, |x| abs_diff(x, median(xs)))
}

/// This finds the m that minimizes:
/// ΣΣ|x(i)-m|
/// By setting the derivative w.r.t. m to 0, it can be shown
/// that the minimal m is within 0.5 of the mean.
/// This function finds the minimal value for m between mean +/- 1,
/// which is (more than) enough.
pub fn part2(xs: &Vec<u64>) -> u64 {
  let guess = |m| {
    sum_map(xs, |x| sum_to(abs_diff(x, m)))
  };
  let mean = xs.iter().sum::<u64>() / xs.len() as u64;
  (mean-1..=mean+1).map(guess).min().unwrap()
}

fn sum_map(xs: &Vec<u64>, f: impl Fn(u64) -> u64) -> u64 {
  xs.iter().map(|&x| f(x)).sum()
}

fn sum_to(n: u64) -> u64 {
  n * (n + 1) / 2
}

fn median(xs: &Vec<u64>) -> u64 {
  let slice: &mut [u64] = &mut xs.clone();
  let (_, median, _) = slice.select_nth_unstable(xs.len() / 2);
  *median
}

// Equivalent fn is only in nightly
fn abs_diff(x: u64, y: u64) -> u64 {
  if x > y {
    x - y
  } else {
    y - x
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculates_fuel_usage_1() {
    let xs = sample();
    assert_eq!(37, part1(&xs));
  }

  #[test]
  fn calculates_fuel_usage_2() {
    let xs = sample();
    assert_eq!(168, part2(&xs));
  }

  fn sample() -> Vec<u64> {
    vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
  }
}