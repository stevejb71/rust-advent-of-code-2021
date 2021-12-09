use crate::common::*;

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day7.txt");
  run_part(part, input_lines, vec_of_usize, part1, part2)
}

pub fn part1(xs: &Vec<usize>) -> usize {
  sum_map(xs, |x| abs_diff(x, median(xs)))
}

/// This finds the m that minimizes:
/// ΣΣ|x(i)-m|
/// By setting the derivative w.r.t. m to 0, it can be shown
/// that the minimal m is within 0.5 of the mean.
/// This function finds the minimal value for m between mean +/- 1,
/// which is (more than) enough.
pub fn part2(xs: &Vec<usize>) -> usize {
  let guess = |m| {
    sum_map(xs, |x| sum_to(abs_diff(x, m)))
  };
  let mean = xs.iter().sum::<usize>() / xs.len();
  (mean-1..=mean+1).map(guess).min().unwrap()
}

fn sum_map(xs: &Vec<usize>, f: impl Fn(usize) -> usize) -> usize {
  xs.iter().map(|&x| f(x)).sum()
}

fn sum_to(n: usize) -> usize {
  n * (n + 1) / 2
}

fn median(xs: &Vec<usize>) -> usize {
  let slice: &mut [usize] = &mut xs.clone();
  let (_, median, _) = slice.select_nth_unstable(xs.len() / 2);
  *median
}

// Equivalent fn is only in nightly
fn abs_diff(x: usize, y: usize) -> usize {
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

  fn sample() -> Vec<usize> {
    vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
  }
}