use crate::common::*;
use std::collections::HashSet;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day19.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(inputs: &Vec<&str>) -> u64 {
  let point_sets = parse(inputs);
  println!("{:?}", point_sets);
  0
}

pub fn part2(inputs: &Vec<&str>) -> u64 {
  0
}

type Point = (i16, i16, i16);
type Vector = (i16, i16, i16);
#[derive(Clone, Debug)]
struct PointSet {
  ps: HashSet<Point>
}

impl PointSet {
  fn new(ps: &Vec<Point>) -> PointSet {
    PointSet { ps: ps.iter().cloned().collect::<HashSet<_>>() }
  }

  fn first(&self) -> &Point {
    self.ps.iter().next().unwrap()
  }

  fn map(&self, f: &impl Fn(&Point) -> Point) -> PointSet {
    let ps = self.ps.iter().map(f).collect();
    PointSet { ps }
  }

  fn rotate(&self, r: usize) -> PointSet {
    match r {
      0 => self.clone(),
      1 => self.map(&|p| (-p.0, -p.1, -p.2)),
      _ => todo!()
    }
  }
  
  fn contains(&self, p: &Point) -> bool {
    self.ps.contains(p)
  }
}

fn distance(p1: &Point, p2: &Point) -> u64 {
  let sq = |x| (x * x) as u64;
  sq(p1.0 - p2.0) + sq(p1.1 - p2.1) + sq(p1.2 - p2.2)
}

fn diff(p1: &Point, p2: &Point) -> Vector {
  ((p2.0 - p1.0), (p2.1 - p1.1), (p2.2 - p1.2))
}

fn translate(ps: &PointSet, d: &Vector) -> PointSet {
  let ps = ps.ps.iter().map(|(x, y, z)| (x + d.0, y + d.1, z + d.2)).collect();
  PointSet::new(&ps)
}

fn has_at_least_12_matching_points(ps1: &PointSet, ps2: &PointSet) -> Option<Vector> {
  println!("ps1={:?}", ps1);
  println!("ps2={:?}", ps2);
  for p1 in &ps1.ps {
    let d = diff(p1, &ps2.first());
    println!("Translate by {:?}", d);
    let ps1 = translate(ps1, &d);
    let mut count = 0;
    for p1 in ps1.ps {
      if ps2.contains(&p1) {
        count += 1;
        println!("Match on {:?}, {}", p1, count);
      }
      if count == 12 {
        return Some(d)
      }
    }
  }
  None
}

fn parse(input: &Vec<&str>) -> Vec<PointSet> {
  fn str_to_point(s: &str) -> Point {
    let coords = s.split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    (coords[0], coords[1], coords[2])
  }
  input.split(|s| s.starts_with("---"))
    .map(|ps| PointSet::new(&ps.iter().filter(|s| !s.is_empty()).map(|x| str_to_point(x)).collect::<Vec<_>>()))
    .skip(1)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn finds_translation_for_12_matching_points() {
    let point_sets = sample();
    println!("X {}", point_sets.len());
    let ps0 = &point_sets[0];
    let ps1 = &point_sets[1].rotate(1);
    let diff = has_at_least_12_matching_points(ps0, ps1);
    assert_eq!(Some((-68, 398, 43)), diff);
  }

  fn sample() -> Vec<PointSet> {
    parse(&vec_of_str(include_str!("../test_inputs/day19.txt")))
  }
}