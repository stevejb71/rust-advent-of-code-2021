use std::collections::HashSet;
use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day11.txt");
  run_part(part, input_lines, vec_of_u8_lines_from_digits, part1, part2)
}

pub fn part1(lines: &Vec<Vec<u8>>) -> u64 {
  let mut lines = lines.clone();
  let mut flashes = 0;
  for _ in 0..100 {
    let step_flashes = run_step(&mut lines);
    flashes += step_flashes;
  }
  flashes
}

pub fn part2(lines: &Vec<Vec<u8>>) -> u64 {
  let mut lines = lines.clone();
  let num_octopuses = (lines.len() * lines.len()) as u64;
  for i in 1..u64::MAX {
    let step_flashes = run_step(&mut lines);
    if step_flashes == num_octopuses {
      return i;
    }
  }
  unreachable!();
}

fn run_step(lines: &mut Vec<Vec<u8>>) -> u64 {
  let size = lines.len();
  for y in 0..size {
    for x in 0..size {
      lines[y][x] += 1;
    }
  }
  let mut flashes = 0;
  let mut one_has_flashed = true;
  let mut flashed_already = HashSet::new();
  while one_has_flashed {
    one_has_flashed = false;
    for y in 0..size {
      for x in 0..size {
        if lines[y][x] > 9 {
          let mut flashes_this_time = 0;
          flash(lines, size, x, y, &mut flashed_already, &mut flashes_this_time);
          flashes += flashes_this_time;
          one_has_flashed = flashes_this_time > 0;
        }
      }
    }
  }
  for (x, y) in flashed_already {
    lines[y][x] = 0;
  }
  flashes
}

fn flash(lines: &mut Vec<Vec<u8>>, size: usize, x: usize, y: usize, flashed_already: &mut HashSet<(usize, usize)>, flashes: &mut u64) {
  if !flashed_already.contains(&(x, y)) {
    *flashes += 1;
    flashed_already.insert((x, y));
    let neighbours = neighbours(size, x, y);
    for (nx, ny) in neighbours {
      lines[ny][nx] += 1;
      if lines[ny][nx] > 9 {
        flash(lines, size, nx, ny, flashed_already, flashes);
      }
    }
  }
}

fn neighbours(size: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
  let mut neighbours = vec![];
  const OFFSETS: &[(isize, isize)] = &[
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1),
  ];
  for &(dx, dy) in OFFSETS {
    let size = size as isize;
    let x_next = x as isize + dx;
    let y_next = y as isize + dy;
    if x_next >= 0 && y_next >= 0 && x_next < size && y_next < size {
      neighbours.push((x_next as usize, y_next as usize));
    }
  }
  neighbours
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_step_from_day1_to_day2() {
    let mut day1 = vec_of_u8_lines_from_digits(include_str!("../test_inputs/day11_step1.txt"));
    let day2 = vec_of_u8_lines_from_digits(include_str!("../test_inputs/day11_step2.txt"));

    let flashes = run_step(&mut day1);

    assert_eq!(day2, day1);
    assert_eq!(35, flashes);
  }

  #[test]
  fn part1_calculates_total_flashes() {
    let mut day1 = vec_of_u8_lines_from_digits(include_str!("../test_inputs/day11_step0.txt"));

    let flashes = part1(&mut day1);

    assert_eq!(1656, flashes);
  }

  #[test]
  fn part2_calculates_first_sync_flash() {
    let mut day1 = vec_of_u8_lines_from_digits(include_str!("../test_inputs/day11_step0.txt"));

    let sync_flash_time = part2(&mut day1);

    assert_eq!(195, sync_flash_time);
  }
}