use std::collections::HashMap;
use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day14.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

type Bigram = (char, char);

pub fn part1(lines: &Vec<&str>) -> u64 {
  run_steps(lines, 10)
}

pub fn part2(lines: &Vec<&str>) -> u64 {
  run_steps(lines, 40)
}

fn run_steps(lines: &Vec<&str>, count: usize) -> u64 {
  let (polymer_template, insertion_rules) = parse(lines);
  let mut bigrams_counts = init_bigrams_histogram(polymer_template);
  for _ in 0..count {
    step(&mut bigrams_counts, &insertion_rules);
  }
  let mut counts = letter_frequency_map(&bigrams_counts);
  let first_char = polymer_template.chars().nth(0).unwrap();
  *counts.get_mut(&first_char).unwrap() += 1;
  let last_char = polymer_template.chars().last().unwrap();
  *counts.get_mut(&last_char).unwrap() += 1;
  let max = counts.values().max().unwrap();
  let min = counts.values().min().unwrap();
  max - min
}

fn letter_frequency_map(bigrams_counts: &HashMap<Bigram, u64>) -> HashMap<char, u64> {
  let mut counting_map = HashMap::new();
  for (bigram, count) in bigrams_counts {
    *counting_map.entry(bigram.0).or_insert(0) += count;
    *counting_map.entry(bigram.1).or_insert(0) += count;
  }
  counting_map.iter().map(|x| (*x.0, *x.1 / 2)).collect()
}

fn step(bigram_counts: &mut HashMap<Bigram, u64>, insertion_rules: &HashMap<Bigram, char>) {
  let mut insertions = vec![];
  let mut deletions = vec![];
  for (bigram, count) in bigram_counts.clone() {
    if let Some(insertion_char) = insertion_rules.get(&bigram) {
      insertions.push(((bigram.0, *insertion_char), count));
      insertions.push(((*insertion_char, bigram.1), count));
      deletions.push(bigram);
    }
  }
  for bigram in deletions {
    *bigram_counts.entry(bigram).or_insert(0) = 0
  }
  for (bigram, count) in insertions {
    *bigram_counts.entry(bigram).or_insert(0) += count;
  }
}

fn init_bigrams_histogram(template: &str) -> HashMap<Bigram, u64> {
  template.chars().collect::<Vec<_>>()
    .windows(2)
    .map(|cs| ((cs[0], cs[1]), 1))
    .collect::<HashMap<_, _>>()
}

fn parse<'a>(lines: &'a Vec<&str>) -> (&'a str, HashMap<Bigram, char>) {
  let polymer_template = lines[0];
  let insertion_rules = &lines[2..].iter()
    .map(|l| {
      let (lhs, rhs) = l.split_once(" -> ").unwrap();
      let lhs = lhs.chars().collect::<Vec<_>>();
      ((lhs[0], lhs[1]), rhs.chars().nth(0).unwrap())
    })
    .collect::<HashMap<_, _>>();
  (polymer_template, insertion_rules.clone())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_finds_most_minus_least_common_elt_count_for_10_steps() {
    let sample = vec_of_str(include_str!("../test_inputs/day14.txt"));
    let count = part1(&sample);
    assert_eq!(1588, count);
  }

  #[test]
  fn part2_finds_most_minus_least_common_elt_count_for_40_steps() {
    let sample = vec_of_str(include_str!("../test_inputs/day14.txt"));
    let count = part2(&sample);
    assert_eq!(2188189693529, count);
  }
}