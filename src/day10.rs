use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day10.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(lines: &Vec<&str>) -> u64 {
  fn score(error: char) -> u64 {
    match error {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
      _ => panic!("unhandled")
    }
  }
  lines.iter()
    .filter_map(|&line| find_first_syntax_error(line))
    .map(|error| score(error))
    .sum()
}

fn find_first_syntax_error(line: &str) -> Option<char> {
  use std::collections::*;
  let mut stack: VecDeque<char> = VecDeque::new();
  fn check(stack: &mut VecDeque<char>, expected: char, error: char) -> Option<char> {
    let actual = stack.pop_front();
    if actual != Some(expected) {
      Some(error)
    } else {
      None
    }
  }
  for ch in line.chars() {
    let error = match ch {
      '(' | '[' | '{' | '<' => {
        stack.push_front(ch);
        None
      },
      ')' => check(&mut stack, '(', ch),
      ']' => check(&mut stack, '[', ch),
      '}' => check(&mut stack, '{', ch),
      '>' => check(&mut stack, '<', ch),
      _ => panic!("unhandled"),
    };
    if error.is_some() {
      return error;
    }
  }
  None
}

pub fn part2(_lines: &Vec<&str>) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_first_syntax_error_on_first_line() {
    let error = find_first_syntax_error("{([(<{}[<>[]}>{[]{[(<()>");
    assert_eq!(Some('}'), error);
  }

  #[test]
  fn part1_find_the_syntax_error_score() {
    let lines = sample();
    assert_eq!(26397, part1(&lines));
  }

  fn sample() -> Vec<&'static str> {
    let lines = include_str!("../test_inputs/day10.txt");
    vec_of_str(&lines)    
  }
}
