use std::collections::*;
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
    .map(|result| {
      if let ParseResult::SyntaxError(ch) = result {
        score(ch)
      } else {
        0
      }
    })
    .sum()
}

pub fn part2(lines: &Vec<&str>) -> u64 {
  let mut scores = lines.iter()
    .filter_map(|&line| find_first_syntax_error(line))
    .filter_map(|result| {
      if let ParseResult::Incomplete(stack) = result {
        Some(autocomplete(&stack))
      } else {
        None
      }
    })
    .map(|ac| score_autocompletion(&ac))
    .collect::<Vec<_>>();
  let middle_index = (scores.len() - 1) / 2;
  *scores.select_nth_unstable(middle_index).1
}

fn score_autocompletion(line: &Vec<char>) -> u64 {
  fn score_char(ch: char) -> u64 {
    match ch {
      ')' => 1,
      ']' => 2,
      '}' => 3,
      '>' => 4,
      _ => panic!("unhandled")
    }  
  }
  line.iter().fold(0u64, |score, &ch| {
    score * 5 + score_char(ch)
  })
}

fn autocomplete(stack: &VecDeque<char>) -> Vec<char> {
  stack.iter().map(|ch| {
    match ch {
      '(' => ')',
      '[' => ']',
      '{' => '}',
      '<' => '>',
      _ => panic!("unhandled")
    }
  }).collect()
}

#[derive(Debug, PartialEq)]
enum ParseResult {
  SyntaxError(char),
  Incomplete(VecDeque<char>),
}

fn find_first_syntax_error(line: &str) -> Option<ParseResult> {
  let mut stack = VecDeque::new();
  fn check(stack: &mut VecDeque<char>, expected: char, error: char) -> Option<ParseResult> {
    let actual = stack.pop_front();
    if actual == Some(expected) {
      None
    } else if actual == None {
      Some(ParseResult::Incomplete(stack.clone()))
    } else {
      Some(ParseResult::SyntaxError(error))
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
  if stack.is_empty() {
    None
  } else {
    Some(ParseResult::Incomplete(stack))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_first_syntax_error_on_corrupt_line_returns_syntax_error() {
    let error = find_first_syntax_error("{([(<{}[<>[]}>{[]{[(<()>");
    assert_eq!(Some(ParseResult::SyntaxError('}')), error);
  }

  #[test]
  fn find_first_syntax_error_on_incomplete_line_returns_stack_which_can_be_autocompleted() {
    let error = find_first_syntax_error("[({(<(())[]>[[{[]{<()<>>");
    if let Some(ParseResult::Incomplete(stack)) = error {
      let completion = autocomplete(&stack);
      assert_eq!(completion, vec!['}', '}', ']', ']', ')', '}', ')', ']']);
    } else {
      assert!(false, "{:?}", error);
    }
  }

  #[test]
  fn part1_find_the_syntax_error_score() {
    let lines = sample();
    assert_eq!(26397, part1(&lines));
  }

  #[test]
  fn part2_scoring_works() {
    let score = score_autocompletion(&")}>]})".chars().collect());
    assert_eq!(5566, score);
  }

  #[test]
  fn part2_find_the_autocomplete_score() {
    let lines = sample();
    assert_eq!(288957, part2(&lines));
  }

  fn sample() -> Vec<&'static str> {
    let lines = include_str!("../test_inputs/day10.txt");
    vec_of_str(&lines)    
  }
}
