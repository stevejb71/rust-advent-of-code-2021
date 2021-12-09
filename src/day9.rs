use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day9.txt");
  run_part(part, input_lines, vec_of_rows_of_digits, part1, part2)
}

pub fn part1(map: &Vec<Vec<u8>>) -> u64 { 
  let map = HeightMap::new(map);
  map.low_points().iter().map(|(_, _, value)| value + 1).sum()
}

pub fn part2(map: &Vec<Vec<u8>>) -> u64 {
  let map = HeightMap::new(map);
  let mut basin_sizes = map.low_points()
    .iter()
    .map(|(x, y, _)| map.find_basin_size(*x, *y))
    .collect::<Vec<_>>();
  (basin_sizes.len()-3..=basin_sizes.len()-1)
    .map(|i| *basin_sizes.select_nth_unstable(i).1 as u64)
    .product()
}

struct HeightMap<'a> {
  m: &'a Vec<Vec<u8>>,
  x_max: usize,
  y_max: usize,
}

impl <'a> HeightMap<'a> {
  fn new(m: &'a Vec<Vec<u8>>) -> HeightMap {
    let x_max = m[0].len();
    let y_max = m.len();
    HeightMap { m, x_max, y_max }
  }

  fn at(&self, p: (usize, usize)) -> u8 {
    self.m[p.1][p.0]
  }

  fn low_points(&self) -> Vec<(usize, usize, u64)> {
    (0..self.y_max).flat_map(|y| {
      (0..self.x_max).filter_map(move |x| {
        let value = self.at((x, y)) as u64;
        let neighbours = self.neighbours(x, y);
        let min_neighbours = neighbours.iter()
          .map(|&p| self.at(p))
          .min()
          .unwrap();
          if value < min_neighbours.into() {
            Some((x, y, value))
          } else {
            None
          }
      })
    }).collect()
  }

  fn find_basin_size(&self, x: usize, y: usize) -> usize {
    use std::collections::*;
    let mut filled = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((x, y));
    while let Some((x, y)) = q.pop_front() {
      if filled.insert((x, y)) {
        let neighbours = self.neighbours(x, y);
        let basin = neighbours
          .iter()
          .filter(|&&p| self.at(p) != 9);
        q.extend(basin);  
      }
    }
    filled.len()
  }
    
  fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    const OFFSETS: &[(isize, isize); 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    OFFSETS.iter().filter_map(|(dx, dy)| {
      let new_x = (x as isize) + dx;
      let new_y = (y as isize) + dy;
      if new_x >= 0 && new_x < self.x_max as isize && 
         new_y >= 0 && new_y < self.y_max as isize {
        Some((new_x as usize, new_y as usize))
      } else {
        None
      }
    }).collect()
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

