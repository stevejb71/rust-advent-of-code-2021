extern crate nalgebra as na;
use crate::common::*;
use na::{SMatrix, SVector};

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day6.txt");
  run_part(part, input_lines, vec_of_usize_one_line, part1, part2)
}

pub fn part1(ages: &Vec<usize>) -> usize {
  predict_population(ages, 80)
}

pub fn part2(ages: &Vec<usize>) -> usize {
  predict_population(ages, 256)
}

fn predict_population(ages: &Vec<usize>, days: usize) -> usize {
  let update = pow(transition_matrix(), days);
  let mut num_by_age = SVector::<usize, 9>::zeros();
  for &age in ages {
    num_by_age[age] += 1;
  }
  let result = update * num_by_age;
  result.iter().sum()
}

fn pow(m: SMatrix<usize, 9, 9>, n: usize) -> SMatrix<usize, 9, 9> {
  let mut result = m;
  if n == 256 {
    for _ in 1..=8 {
      result *= result;
    }
  } else {
    for _ in 1..n {
      result *= m;
    }
  }
  result
}

fn transition_matrix() -> SMatrix<usize, 9, 9> {
  SMatrix::<usize, 9, 9>::from_row_slice(&[
    0, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0,
  ])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn predicts_population_after_18_gens() {
    let ages = sample();
    assert_eq!(26, predict_population(&ages, 18));
  }

  #[test]
  fn predicts_population_after_80_gens() {
    let ages = sample();
    assert_eq!(5934, predict_population(&ages, 80));
  }

  #[test]
  fn predicts_population_after_256_gens() {
    let ages = sample();
    assert_eq!(26984457539, predict_population(&ages, 256));
  }

  fn sample() -> Vec<usize> {
    vec![3, 4, 3, 1, 2]
  }
}