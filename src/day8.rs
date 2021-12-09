use crate::common::*;

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day8.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

#[derive(Debug)]
struct Entry {
  signals: Vec<String>,
  outputs: Vec<String>,
}

pub fn part1(entries: &Vec<&str>) -> usize {
  let entries = entries.iter().map(|x| parse(&x)).collect::<Vec<_>>();
  entries.iter()
    .flat_map(|e| &e.outputs)
    .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7)
    .count()
}

pub fn part2(entries: &Vec<&str>) -> usize {
  let entries = entries.iter().map(|x| parse(&x)).collect::<Vec<_>>();
  entries.iter().map(|entry| {
    let pick_one = |len| {
      let matches = entry.signals.iter().filter(|s| s.len() == len).collect::<Vec<_>>();
      assert_eq!(1, matches.len());
      matches[0]
    };
    let one = pick_one(2);
    let four = pick_one(4);
    let digits = entry.outputs.iter().map(|output| {
      let count_overlaps = |d: &String| {
        output.chars().filter(|&co| d.contains(co)).count()
      };
      match output.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        5 => {
          if count_overlaps(one) == 2 {
            3
          } else if count_overlaps(four) == 2 {
            2
          } else {
            5
          }
        },
        6 => {
          if count_overlaps(four) == 4 {
            9
          } else if count_overlaps(one) == 2 {
            0
          } else {
            6
          }
        }
        _ => panic!("shouldn't happen")
      }
    }).collect::<Vec<_>>();
    digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
  }).sum()
}

fn parse(e: &str) -> Entry {
  let mut split = e.split("|");
  let signals = split.next().unwrap().split_ascii_whitespace().map(|x| x.to_string()).collect::<Vec<_>>();
  let outputs = split.next().unwrap().split_ascii_whitespace().map(|x| x.to_string()).collect::<Vec<_>>();
  Entry { signals, outputs }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_finds_easy_entries() {
    let entries = sample();
    assert_eq!(26, part1(&entries));
  }

  #[test]
  fn part2_finds_all_entries() {
    let entries = sample();
    assert_eq!(61229, part2(&entries));
  }

  fn sample() -> Vec<&'static str> {
    include_str!("../test_inputs/day8.txt").lines().collect()
  }
}

