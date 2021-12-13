use std::collections::HashSet;
use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day13.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(lines: &Vec<&str>) -> u64 {
  let (sheet, folds) = parse(lines);
  let sheet = apply_fold(&sheet, &folds[0]);
  sheet.len() as u64
}

pub fn part2(lines: &Vec<&str>) -> u64 {
  let sheet = part2_noprint(lines);
  print(&sheet);
  0
}

pub fn part2_noprint(lines: &Vec<&str>) -> Sheet {
  let (mut sheet, folds) = parse(lines);
  for fold in &folds {
    sheet = apply_fold(&sheet, fold);
  }
  sheet
}

#[derive(Debug, PartialEq)]
enum FoldType {
  Up, Left
}

struct Fold {
  fold_type: FoldType,
  pos: u64,
}

pub type Pos = (u64, u64);
pub type Sheet = HashSet<Pos>;

fn print(sheet: &Sheet) {
  let x_max = sheet.iter().max_by_key(|(x, _)| x).unwrap().0;
  let y_max = sheet.iter().max_by_key(|(_, y)| y).unwrap().1;
  for y in 0..=y_max {
    let mut row = String::with_capacity(x_max as usize + 1);
    for x in 0..=x_max {
      let ch = if sheet.iter().any(|(xd, yd)| *xd == x && *yd == y) {
        '#'
      } else {
        '.'
      };
      row.push(ch);
    }
    println!("{}", row);
  }
}

fn apply_fold(sheet: &Sheet, fold: &Fold) -> Sheet {
  let folded_dots = if fold.fold_type == FoldType::Up {
    sheet.iter()
      .map(|&(xd, yd)| {
        if yd > fold.pos {
          (xd, (fold.pos - (yd - fold.pos)))
        } else {
          (xd, yd)
        }
      }).collect::<HashSet<_>>()
  } else {
    sheet.iter()
      .map(|&(xd, yd)| {
        if xd > fold.pos {
          (fold.pos - (xd - fold.pos), yd)
        } else {
          (xd, yd)
        }
      }).collect::<HashSet<_>>()
  };
  folded_dots
}

fn parse(lines: &Vec<&str>) -> (Sheet, Vec<Fold>) {
  fn split_comma_sep_pair(s: &str) -> (u64, u64) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
  }
  fn parse_fold(s: &str) -> Fold {
    let info = &s[11..];
    let fold_type = if info.chars().nth(0) == Some('x') {
      FoldType::Left
    } else {
      FoldType::Up
    };
    let pos = info[2..].parse().unwrap();
    Fold { fold_type, pos }
  }
  let mut split = lines.split(|x| x.is_empty());
  let dots = split.next().unwrap().iter()
    .map(|l| split_comma_sep_pair(l))
    .collect::<HashSet<_>>();
  let folds = split.next().unwrap().iter()
    .map(|&l| parse_fold(l))
    .collect();
  (dots, folds)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_counts_dots() {
    let sample = vec_of_str(include_str!("../test_inputs/day13.txt"));
    let count = part1(&sample);
    assert_eq!(17, count);
  }
}