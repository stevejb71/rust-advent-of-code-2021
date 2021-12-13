use std::collections::*;
use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day12.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(lines: &Vec<&str>) -> u64 {
  fn can_visit(visited: Vec<Cave>, here: &Cave) -> (bool, Vec<Cave>) {
    if here.is_small() {
      if visited.contains(&here) {
        (false, visited)
      } else {
        let mut visited = visited.clone();
        visited.push(here.clone());
        (true, visited)
      }
    } else {
      (true, visited)
    }
  }
  search(lines, can_visit)
}

fn cave_str_to_index(s: &str) -> u16 {
  match s {
    "start" => 0,
    "end" => 1,
    _ => {
      let mut chs = s.chars();
      let ch1 = chs.next().unwrap() as u16;
      let ch2 = chs.next().unwrap_or_default() as u16;
      ch1 * 256 + ch2    
    }
  }
}

#[derive(Clone, Default)]
struct VisitsTracker {
  visited: Vec<Cave>,
  twice: bool,
}

pub fn part2(lines: &Vec<&str>) -> u64 {
  fn can_visit(tracker: VisitsTracker, here: &Cave) -> (bool, VisitsTracker) {
    if here.is_small() {
      let first_time = !tracker.visited.contains(here);
      if first_time {
        let mut tracker = tracker.clone();
        tracker.visited.push(here.clone());
        (true, tracker) 
      } else {
        if tracker.twice {
          (false, tracker)
        } else {
          let mut tracker = tracker.clone();
          tracker.twice = true;
          (true, tracker)
        }
      }
    } else {
      (true, tracker)
    }
  }
  search(lines, can_visit)
}

fn search<V: Clone + Default>(lines: &Vec<&str>, can_visit: fn(V, &Cave) ->  (bool, V)) -> u64 {
  let cave_system = parse(lines);
  let mut q = VecDeque::new();
  q.push_front((Cave::Start, V::default()));
  let mut count = 0;
  while let Some((here, visited)) = q.pop_front() {
    if here == Cave::End {
      count += 1;
      continue;
    }
    let (can_visit, visited) = can_visit(visited, &here);
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
  Big(u16),
  Small(u16)
}

impl Cave {
  fn is_small(&self) -> bool {
    match self {
      Cave::Small(_) => true,
      _ => false,
    }
  }
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
        let index = cave_str_to_index(s);
        if ch.is_lowercase() {
          Cave::Small(index)
        } else {
          Cave::Big(index)
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

