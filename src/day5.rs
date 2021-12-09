use crate::common::*;

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day5.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

struct Line {
  start: (usize, usize),
  end: (usize, usize),
}

pub fn part1(inputs: &Vec<&str>) -> usize {
  solve(inputs, false)
}

pub fn part2(inputs: &Vec<&str>) -> usize {
  solve(inputs, true)
}

pub fn solve(inputs: &Vec<&str>, allow_diagonals: bool) -> usize {
  let mut sea_bed = vec![vec![0usize; 1000]; 1000];
  let lines = parse(&inputs);
  for line in lines {
    if line.start.0 == line.end.0 {
      let (start, end) = minmax(line.start.1, line.end.1);
      for y in start..=end {
        sea_bed[y][line.start.0] += 1;
      }
    } else if line.start.1 == line.end.1 {
      let (start, end) = minmax(line.start.0, line.end.0);
      for x in start..=end {
        sea_bed[line.start.1][x] += 1;
      }
    } else if allow_diagonals && line.start.0 < line.end.0 {
      let (y_start, y_end) = (line.start.1, line.end.1);
      let dy = if y_start > y_end { -1 } else { 1 };
      let mut y = y_start as isize;
      for x in line.start.0..=line.end.0 {
        sea_bed[y as usize][x] += 1;
        y += dy;
      }
    } else if allow_diagonals {
      let (y_start, y_end) = (line.end.1, line.start.1);
      let dy = if y_start > y_end { -1 } else { 1 };
      let mut y = y_start as isize;
      for x in line.end.0..=line.start.0 {
        sea_bed[y as usize][x] += 1;
        y += dy;
      }
    }
  }
  sea_bed.iter().map(|row| {
    row.iter().filter(|&&n| n >= 2).count()
  }).sum()
}

fn parse(inputs: &Vec<&str>) -> Vec<Line> {
  fn parse_line(s: &str) -> Line {
    let coords = s.split(" -> ")
      .map(|x| {
        let cs = x.split(",").collect::<Vec<_>>();
        (cs[0].parse::<usize>().unwrap(), cs[1].parse::<usize>().unwrap())
      })
      .collect::<Vec<_>>();
    Line { start: coords[0], end: coords[1] }
  }
  inputs.iter().map(|&x| parse_line(x)).collect()
}

fn minmax(a: usize, b: usize) -> (usize, usize) {
  if a < b {
    (a, b)
  } else {
    (b, a)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_scores_correctly() {
    let lines = sample();
    assert_eq!(5, part1(&lines));
  }

  #[test]
  fn part2_scores_correctly() {
    let lines = sample();
    assert_eq!(12, part2(&lines));
  }

  fn sample() -> Vec<&'static str> {
    include_str!("../test_inputs/day5.txt").lines().collect()
  }
}