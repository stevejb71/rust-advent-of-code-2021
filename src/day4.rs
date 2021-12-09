use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day4.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(inputs: &Vec<&str>) -> u64 {
  let get_score = |cards: &Vec<Card>, n| {
    cards.iter().find(|c| c.is_winner())
      .map(|w| n * w.sum_unmarked())
  };
  solve(&inputs, get_score)
}

pub fn part2(inputs: &Vec<&str>) -> u64 {
  let mut last_board = 0;
  let get_score = |cards: &Vec<Card>, n| {
    let non_winners = cards.iter()
      .enumerate()
      .filter(|(_, c)| !c.is_winner())
      .map(|(i, _)| i)
      .collect::<Vec<_>>();
    match non_winners.len() {
      0 => Some(cards[last_board].sum_unmarked() * n),
      1 => {
        last_board = non_winners[0];
        None
      },
      _ => None,
    }
  };
  solve(&inputs, get_score)
}

fn solve<'a>(inputs: &'a Vec<&str>, mut get_score: impl FnMut(&Vec<Card>, u64) -> Option<u64>) -> u64 {
  let (numbers, mut cards) = parse(inputs);
  for n in numbers {
    for card in &mut cards {
      card.mark(n);
    }
    let score = get_score(&cards, n);
    if score.is_some() {
      return score.unwrap();
    }
  }
  panic!("didn't find winning card");
}

#[derive(Debug)]
struct Card {
  numbers: Vec<Vec<u64>>,
  marks: Vec<u8>,
}

impl Card {
  fn new(numbers: Vec<Vec<u64>>) -> Card {
    Card {numbers, marks: vec![0; 5]}
  }

  fn mark(&mut self, called_num: u64) {
    for (mark, row) in self.marks.iter_mut().zip(&self.numbers) {
      for (col, &num) in row.iter().enumerate() {
        if num == called_num {
          *mark |= 1 << col;
        }
      }
    }
  }

  fn is_winner(&self) -> bool {
    self.marks.iter().any(|&row| row == 31) || self.has_winning_col()
  }

  fn has_winning_col(&self) -> bool {
    (0..5).any(|col| self.marks.iter().all(|mark_row| mark_row & 1 << col != 0))
  }

  fn sum_unmarked(&self) -> u64 {
    let mut sum = 0;
    for (mark, nums_row) in self.marks.iter().zip(&self.numbers) {
      for (col, &num) in nums_row.iter().enumerate() {
        if mark & (1 << col) == 0 {
          sum += num;
        }
      }
    }
    sum
  }
}

fn parse(inputs: &Vec<&str>) -> (Vec<u64>, Vec<Card>) {
  let numbers = inputs[0].split(",").map(|x| x.parse::<u64>().unwrap()).collect();
  let inputs = inputs.iter().skip(2)
      .filter(|line| !line.is_empty())
      .map(|line| line.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>())
      .collect::<Vec<_>>()
      .chunks(5)
      .map(|nums| Card::new(nums.to_vec()))
      .collect::<Vec<_>>();
  (numbers, inputs)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mark_card() {
    let sample = sample();
    let card = &mut parse(&sample).1[0];
    card.mark(11);
    assert_eq!(8, card.marks[0]);
    card.mark(1);
    assert_eq!(1, card.marks[4]);
  }

  #[test]
  fn is_winner() {
    let sample = sample();
    let card = &mut parse(&sample).1[0];
    assert!(!card.is_winner());
    card.mark(13);
    card.mark(2);
    card.mark(9);
    card.mark(10);
    assert!(!card.is_winner());
    card.mark(12);
    assert!(card.is_winner());
  }

  #[test]
  fn sum_unmarked() {
    let sample = sample();
    let card = &mut parse(&sample).1[0];
    assert_eq!(300, card.sum_unmarked());
    card.mark(1);
    assert_eq!(299, card.sum_unmarked());
  }

  #[test]
  fn part1_finds_first_winning_board_score() {
    let numbers = sample();
    assert_eq!(4512, part1(&numbers));
  }  

  #[test]
  fn part2_finds_last_winning_board_score() {
    let numbers = sample();
    assert_eq!(1924, part2(&numbers));
  }  

  fn sample() -> Vec<&'static str> {
    include_str!("../test_inputs/day4.txt").lines().collect()
  }
}
