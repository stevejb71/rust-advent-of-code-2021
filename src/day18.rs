use crate::common::*;
use std::rc::Rc;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day18.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(inputs: &Vec<&str>) -> u64 {
  let total = inputs.iter()
    .map(|&s| parse(s))
    .reduce(|total, n| add(total, n))
    .unwrap();
  total.magnitude()
}

pub fn part2(inputs: &Vec<&str>) -> u64 {
  let snail_nums = inputs.iter()
    .map(|&s| parse(s))
    .collect::<Vec<_>>();
  use core::ptr::eq;
  let mut max = 0;
  for lhs in &snail_nums {
    for rhs in &snail_nums {
      if !eq(lhs, rhs) {
        max = max.max(add(lhs.clone(), rhs.clone()).magnitude());
      }
    }
  }
  max
}

#[derive(Clone, Debug, PartialEq)]
enum SnailNum {
  Pair(Rc<SnailNum>, Rc<SnailNum>),
  Single(u64)
}

impl SnailNum {
  fn pair(lhs: SnailNum, rhs: SnailNum) -> SnailNum {
    SnailNum::Pair(Rc::new(lhs), Rc::new(rhs))
  }

  fn value(&self) -> u64 {
    match self {
      SnailNum::Single(n) => *n,
      _ => panic!("cannot get value of pair: {:?}", self),
    }
  }

  fn magnitude(&self) -> u64 {
    match self {
      SnailNum::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
      SnailNum::Single(n) => *n,
    }
  }

  #[allow(dead_code)]
  fn to_string(&self) -> String {
    match self {
      SnailNum::Pair(lhs, rhs) => format!("[{},{}]", lhs.to_string(), rhs.to_string()),
      SnailNum::Single(n) => n.to_string()
    }
  }
}

#[derive(Debug)]
struct Update {
  add_l: Option<u64>,
  new_num: SnailNum,
  add_r: Option<u64>
}

impl Update {
  fn some_new(add_l: Option<u64>, new_num: SnailNum, add_r: Option<u64>) -> Option<Update> {
    Some(Update {add_l, new_num, add_r})
  }
}

fn explode(num: &SnailNum) -> Option<SnailNum> {
  fn dig(num: &SnailNum, depth: usize) -> Option<Update> {
    match (num, depth) {
      (SnailNum::Pair(lhs, rhs), d) if d < 4 => {
        if let Some(Update {add_l, new_num, add_r}) = dig(lhs, d + 1) {
          let rhs = add_r.map(|x| Rc::new(add_to_right(x, rhs))).unwrap_or(rhs.clone());
          let num = SnailNum::Pair(Rc::new(new_num), rhs);
          Update::some_new(add_l, num, None)
        } else if let Some(Update {add_l, new_num, add_r}) = dig(rhs, d + 1) {
          let lhs = add_l.map(|x| Rc::new(add_to_left(lhs, x))).unwrap_or(lhs.clone());
          let num = SnailNum::Pair(lhs, Rc::new(new_num));
          Update::some_new(None, num, add_r)
        } else {
          None
        }
      },
      (SnailNum::Pair(lhs, rhs), 4) => 
        Update::some_new(Some(lhs.value()), SnailNum::Single(0), Some(rhs.value())),
      _ => None,
    }
  }

  dig(&num, 0).map(|u| u.new_num)
}

fn add(lhs: SnailNum, rhs: SnailNum) -> SnailNum {
  let mut n = SnailNum::pair(lhs, rhs);
  while let Some(e) = reduce_once(&n) {
    n = e;
  }
  n
}

fn reduce_once(n: &SnailNum) -> Option<SnailNum> {
  let mut reduced = false;
  let mut n = n.clone();
  while let Some(e) = explode(&n) {
    reduced = true;
    n = e;
  }
  if let Some(e) = split(&n) {
    Some(e)
  } else if reduced { 
    Some(n) 
  } else { 
    None 
  }
}

fn split(num: &SnailNum) -> Option<SnailNum> {
  match num {
    SnailNum::Single(n) if *n >= 10 => {
      if n % 2 == 0 {
        let n = n / 2;
        Some(SnailNum::pair(SnailNum::Single(n), SnailNum::Single(n)))
      } else {
        let down = n / 2;
        let up = n - down;
        Some(SnailNum::pair(SnailNum::Single(down), SnailNum::Single(up)))
      }
    }
    SnailNum::Pair(lhs, rhs) => {
      if let Some(lhs) = split(lhs) {
        Some(SnailNum::Pair(Rc::new(lhs), (*rhs).clone()))
      } else if let Some(rhs) = split(rhs) {
        Some(SnailNum::Pair((*lhs).clone(), Rc::new(rhs)))
      } else {
        None
      }
    }
    _ => None,
  }
}

fn add_to_left(num: &SnailNum, x: u64) -> SnailNum {
  match num {
    SnailNum::Single(y) => SnailNum::Single(x + y),
    SnailNum::Pair(lhs, rhs) => SnailNum::Pair(lhs.clone(), Rc::new(add_to_left(rhs, x))),
  }
}

fn add_to_right(x: u64, num: &SnailNum) -> SnailNum {
  match num {
    SnailNum::Single(y) => SnailNum::Single(x + y),
    SnailNum::Pair(lhs, rhs) => SnailNum::Pair(Rc::new(add_to_right(x, lhs)), rhs.clone()),
  }
}

fn parse(line: &str) -> SnailNum {
  fn parse_snail_num(line: &[char]) -> (&[char], SnailNum) {
    let line = skip(line, '[');
    let (line, lhs) = parse_elt(line);
    let line = skip(line, ',');
    let (line, rhs) = parse_elt(line);
    let line = skip(line, ']');
    (line, SnailNum::pair(lhs, rhs))
  }
  fn skip(line: &[char], expected: char) -> &[char] {
    assert_eq!(expected, line[0], "Expecting {} but got {}", expected, line[0]);
    &line[1..]
  }
  fn parse_elt(line: &[char]) -> (&[char], SnailNum) {
    if line[0] == '[' {
      parse_snail_num(line)
    } else {
      let digits = line.iter().take_while(|&&c| c.is_digit(10)).collect::<String>();
      let num = SnailNum::Single(digits.parse().unwrap());
      (&line[digits.len()..], num)
    }
  }
  let line = line.chars().collect::<Vec<_>>();
  let (_, snail_num) = parse_snail_num(&line);
  snail_num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_single_number() {
    assert_eq!(SnailNum::pair(SnailNum::Single(1), SnailNum::Single(2)), parse("[1,2]"));
  }

  #[test]
  fn parses_nested_pair() {
    assert_eq!(SnailNum::pair(
      SnailNum::pair(SnailNum::Single(1), SnailNum::Single(9)),
      SnailNum::pair(SnailNum::Single(8), SnailNum::Single(5)),
    ), parse("[[1,9],[8,5]]"));
  }

  #[test]
  fn explode_on_num_with_no_lhs() {
    test_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
  }

  #[test]
  fn explode_on_num_with_no_rhs() {
    test_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
  }

  #[test]
  fn explode_on_num_with_rhs_higher_up_the_tree() {
    test_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
  }

  #[test]
  fn explode_one_at_a_time() {
    test_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
  }

  #[test]
  fn explode_again() {
    test_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
  }

  #[test]
  fn split_does_nothing_on_9() {
    let n = SnailNum::Single(9);
    let n = split(&n);
    assert_eq!(None, n);
  }

  #[test]
  fn split_10() {
    let n = SnailNum::Single(10);
    let n = split(&n);
    assert_eq!(Some(SnailNum::pair(SnailNum::Single(5), SnailNum::Single(5))), n);
  }

  #[test]
  fn split_11() {
    let n = SnailNum::Single(11);
    let n = split(&n);
    assert_eq!(Some(SnailNum::pair(SnailNum::Single(5), SnailNum::Single(6))), n);
  }

  #[test]
  fn split_pair_with_no_num_over_10_returns_none() {
    let n = SnailNum::pair(
      SnailNum::pair(SnailNum::Single(5), SnailNum::Single(4)),
      SnailNum::pair(SnailNum::Single(5), SnailNum::Single(4)),
    );
    let n = split(&n);
    assert_eq!(None, n);
  }

  #[test]
  fn split_pair_with_num_over_10_splits_one_on_lhs() {
    let n = SnailNum::pair(
      SnailNum::pair(SnailNum::Single(12), SnailNum::Single(4)),
      SnailNum::Single(13),
    );
    let expected = SnailNum::pair(
      SnailNum::pair(
        SnailNum::pair(SnailNum::Single(6), SnailNum::Single(6)
      ), SnailNum::Single(4)),
      SnailNum::Single(13),
    );
    let n = split(&n);
    assert_eq!(Some(expected), n);
  }

  #[test]
  fn magnitude() {
    let n = parse("[[1,2],[[3,4],5]]");
    assert_eq!(143, n.magnitude());
  }

  #[test]
  fn adding() {
    let lhs = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let rhs = parse("[1,1]");
    let added = add(lhs, rhs);
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", added.to_string());
  }

  #[test]
  fn part1_works() {
    let input = sample();
    assert_eq!(4140, part1(&input));
  }

  #[test]
  fn part2_works() {
    let input = sample();
    assert_eq!(3993, part2(&input));
  }

  fn test_explode(s: &str, expected: &str) {
    let sample = parse(s);
    let sample = explode(&sample);
    assert_eq!(expected, sample.unwrap().to_string());
  }

  fn sample() -> Vec<&'static str> {
    vec_of_str(include_str!("../test_inputs/day18.txt"))
  }
}