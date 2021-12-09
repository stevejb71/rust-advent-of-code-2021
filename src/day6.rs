use crate::common::*;

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day6.txt");
  run_part(part, input_lines, vec_of_usize, part1, part2)
}

pub fn part1(ages: &Vec<usize>) -> usize {
  predict_population(ages, 80)
}

pub fn part2(ages: &Vec<usize>) -> usize {
  predict_population(ages, 256)
}

fn predict_population(ages: &Vec<usize>, days: usize) -> usize {
  let mut num_by_age = vec![0; 9];
  for &age in ages {
    num_by_age[age] += 1;
  }
  for day in 1..=days {
    let new_fish = num_by_age[(day - 1) % 9];
    num_by_age[(day + 6) % 9] += new_fish;
    num_by_age[(day + 8) % 9] = new_fish;
  }
  num_by_age.iter().sum()
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