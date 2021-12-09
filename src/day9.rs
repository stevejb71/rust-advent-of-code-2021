use crate::common::*;

pub fn run(part: usize) -> usize {
  let input_lines = include_str!("../inputs/day9.txt");
  run_part(part, input_lines, vec_of_rows_of_digits, part1, part2)
}

struct Matrix {
  m: Vec<Vec<u8>>,
  x_max: usize,
  y_max: usize,
}

impl Matrix {
  fn new(m: Vec<Vec<u8>>) -> Matrix {
    let x_max = m[0].len();
    let y_max = m.len();
    Matrix { m, x_max, y_max }
  }

  fn at(&self, x: usize, y: usize) -> u8 {
    self.m[y][x]
  }

  fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    const OFFSETS: &[(isize, isize); 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in OFFSETS {
      let new_x = (x as isize) + dx;
      let new_y = (y as isize) + dy;
      if new_x >= 0 && new_x < self.x_max as isize && new_y >= 0 && new_y < self.y_max as isize {
        let pt = (new_x as usize, new_y as usize);
        neighbours.push(pt);
      } 
    }
    neighbours
  }
}

pub fn part1(map: &Vec<Vec<u8>>) -> usize { 
  let map = Matrix::new(map.clone());
  let mut sum = 0;
  for_low_points(&map, &mut |_, _, value| {
    sum += 1 + value;
  });
  sum
}

pub fn part2(map: &Vec<Vec<u8>>) -> usize {
  let map = Matrix::new(map.clone());
  let mut basin_sizes = vec![];
  for_low_points(&map, &mut |x, y, _| {
    let basin = find_basin_size(&map, x, y);
    basin_sizes.push(basin);
  });
  basin_sizes.sort();
  basin_sizes.reverse();
  basin_sizes.iter()
    .take(3)
    .product()
}

fn find_basin_size(map: &Matrix, x: usize, y: usize) -> usize {
  use std::collections::*;
  let mut filled = HashSet::new();
  let mut q = VecDeque::new();
  q.push_back((x, y));
  while let Some((x, y)) = q.pop_front() {
    if filled.insert((x, y)) {
      let neighbours = map.neighbours(x, y);
      let basin = neighbours
        .iter()
        .filter(|p| map.at(p.0, p.1) != 9);
      q.extend(basin);  
    }
  }
  filled.len()
}

fn for_low_points(map: &Matrix, f: &mut impl FnMut(usize, usize, usize)) {
  for y in 0..map.y_max {
    for x in 0..map.x_max {
      let value = map.at(x, y) as usize;
      let neighbours = map.neighbours(x, y);
      let min_neighbours = neighbours.iter()
        .map(|&(x, y)| map.at(x, y))
        .min()
        .unwrap();
      if value < min_neighbours.into() {
        f(x, y, value)
      }
    }         
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_finds_lowest_point() {
    let entries = sample();
    assert_eq!(15, part1(&entries));
  }

  #[test]
  fn part2_finds_basins() {
    let entries = sample();
    assert_eq!(1134, part2(&entries));
  }

  fn sample() -> Vec<Vec<u8>> {
    let map = include_str!("../test_inputs/day9.txt");
    vec_of_rows_of_digits(&map)    
  }
}

