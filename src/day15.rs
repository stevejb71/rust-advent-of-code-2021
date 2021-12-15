use pathfinding::directed::dijkstra;
use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day15.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(lines: &Vec<&str>) -> u64 {
  let cavern = parse(lines);
  find_path_cost(
    |x, y| { cavern[y][x] }, 
    cavern[0].len() - 1, 
    cavern.len() - 1
  )
}

pub fn part2(lines: &Vec<&str>) -> u64 {
  let cavern = parse(lines);
  let width = cavern[0].len();
  let height = cavern.len();
  let risk_at = |x, y: usize| {
    let x_rep = x / width; 
    let y_rep = y / height;
    let value = cavern[y % height][x % width] as usize + x_rep + y_rep;
    if value > 9 {
      (value - 9) as u8
    } else {
      value as u8
    }
  };
  find_path_cost(risk_at, 5 * width - 1, 5 * height - 1)
}

fn find_path_cost(
  risk_at: impl Fn(usize, usize) -> u8,
  x_max: usize,
  y_max: usize
) -> u64 {
  let (path, _) = dijkstra::dijkstra(
    &(0, 0), 
    |&p| successors(&risk_at, x_max, y_max, p),
    |&(x, y)| x == x_max && y == y_max
  ).unwrap();
  let risk = path.iter()
      .map(|&(x, y)| risk_at(x, y) as u64)
      .sum::<u64>();
  risk - risk_at(0 ,0) as u64
}

type Pos = (usize, usize);

fn parse(lines: &Vec<&str>) -> Vec<Vec<u8>> {
  lines.iter().map(|l| {
    l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>()
  }).collect()
}

fn successors(risk_at: impl Fn(usize, usize) -> u8, x_max: usize, y_max: usize, (x, y): Pos) -> Vec<(Pos, u64)> {
  const OFFSETS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
  let x = x as isize;
  let y = y as isize;
  let x_max = x_max as isize;
  let y_max = y_max as isize;
  let mut successors = vec![];
  for (dx, dy) in &OFFSETS {
    if x + dx >= 0 && x + dx <= x_max && y + dy >= 0 && y + dy <= y_max {
      let next_x = (x + dx) as usize;
      let next_y = (y + dy) as usize;
      successors.push(((next_x, next_y), risk_at(next_x, next_y) as u64));
    }
  }
  successors
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_finds_cheapest_path() {
    let sample = vec_of_str(include_str!("../test_inputs/day15.txt"));
    let count = part1(&sample);
    assert_eq!(40, count);
  }

  #[test]
  fn part2_finds_cheapest_path_in_extended_map() {
    let sample = vec_of_str(include_str!("../test_inputs/day15.txt"));
    let count = part2(&sample);
    assert_eq!(315, count);
  }
}