use std::collections::*;
use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day12.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(lines: &Vec<&str>) -> u64 {
  fn can_visit(visited: &mut HashSet<Cave>, here: &Cave) -> bool {
    match here {
      Cave::Small(_) => visited.insert(here.clone()),
      _ => true,
    }
  }
  search(lines, can_visit)
}

#[derive(Clone, Default)]
struct VisitsTracker {
  visited: HashSet<Cave>,
  twice: bool,
}

pub fn part2(lines: &Vec<&str>) -> u64 {
  fn can_visit(tracker: &mut VisitsTracker, here: &Cave) -> bool {
    match here {
      Cave::Small(_) => {
        let first_time = tracker.visited.insert(here.clone());
        if first_time {
          true 
        } else {
          if tracker.twice {
            false
          } else {
            tracker.twice = true;
            true
          }
        }
      }
      _ => true,
    }
  }
  search(lines, can_visit)
}

fn search<V: Clone + Default>(lines: &Vec<&str>, can_visit: fn(&mut V, &Cave) -> bool) -> u64 {
  let cave_system = parse(lines);
  let mut q = VecDeque::new();
  q.push_front((Cave::Start, V::default()));
  let mut count = 0;
  while let Some((here, mut visited)) = q.pop_front() {
    if here == Cave::End {
      count += 1;
      continue;
    }
    let can_visit = can_visit(&mut visited, &here);
    if can_visit {
      let next_caves = cave_system.next_caves(&here);
      let next_caves_with_visits = next_caves.iter()
        .map(|c| (c.clone(),  visited.clone()));
      q.extend(next_caves_with_visits);
    }
  }
  count
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Cave {
  Start,
  End,
  Big(String),
  Small(String)
}

#[derive(Default)]
struct CaveSystem {
  paths: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
  fn add(&mut self, from: Cave, to: Cave) {
    if to != Cave::Start {
      let to_paths = self.paths.entry(from.clone()).or_default();
      to_paths.push(to.clone());
    }
    if from != Cave::Start {
      let from_paths = self.paths.entry(to).or_default();
      from_paths.push(from);
    }
  }

  fn next_caves(&self, from: &Cave) -> &Vec<Cave> {
    &self.paths[&from]
  }
}

fn parse(lines: &Vec<&str>) -> CaveSystem {
  let mut cave_system = CaveSystem::default();
  for line in lines {
    let (from, to) = parse_line(line);
    cave_system.add(from, to);
  }
  cave_system
}

fn parse_line(line: &str) -> (Cave, Cave) {
  fn parse_cave(s: &str) -> Cave {
    match s {
      "start" => Cave::Start,
      "end" => Cave::End,
      _ => {
        let ch = s.chars().nth(0).unwrap();
        if ch.is_lowercase() {
          Cave::Small(s.to_owned())
        } else {
          Cave::Big(s.to_owned())
        }
      }
    }
  }
  let mut split = line.split("-");
  let from = split.next().expect(&format!("from? {}", line));
  let to = split.next().expect(&format!("to? {}", line));
  (parse_cave(from), parse_cave(to))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_finds_10_paths_in_example1() {
    let sample = vec_of_str(include_str!("../test_inputs/day12-example1.txt"));
    let paths = part1(&sample);

    assert_eq!(10, paths);
  }

  #[test]
  fn part1_finds_19_paths_in_example2() {
    let sample = vec_of_str(include_str!("../test_inputs/day12-example2.txt"));
    let paths = part1(&sample);

    assert_eq!(19, paths);
  }

  #[test]
  fn part1_finds_226_paths_in_example3() {
    let sample = vec_of_str(include_str!("../test_inputs/day12-example3.txt"));
    let paths = part1(&sample);

    assert_eq!(226, paths);
  }

  #[test]
  fn part2_finds_36_paths_in_example1() {
    let sample = vec_of_str(include_str!("../test_inputs/day12-example1.txt"));
    let paths = part2(&sample);

    assert_eq!(36, paths);
  }
}
