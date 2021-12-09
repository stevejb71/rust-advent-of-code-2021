use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day2.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

#[derive(Default, Debug)]
struct Position {
  depth: u32,
  x: u32,
  aim: u32,
}

pub fn part1(inputs: &Vec<&str>) -> u64 {
  fn next_position(p: Position, cmd: &str) -> Position {
    apply_command(
      cmd, 
      |n| Position { x: p.x + n, ..p },
      |n| Position { depth: p.depth - n, ..p },
      |n| Position { depth: p.depth + n, ..p },
    )
  }
  solve(inputs, next_position)
}

pub fn part2(inputs: &Vec<&str>) -> u64 {
  fn next_position(p: Position, cmd: &str) -> Position {
    apply_command(
      cmd, 
      |n| Position { x: p.x + n, depth: p.depth + p.aim * n, ..p },
      |n| Position { aim: p.aim - n, ..p },
      |n| Position { aim: p.aim + n, ..p },
    )
  }
  solve(inputs, next_position)
}

fn solve(inputs: &Vec<&str>, next_position: fn(Position, &str) -> Position) -> u64 {
  let final_pos = inputs.iter().fold(Position::default(), |p, &s| next_position(p, s));
  (final_pos.x * final_pos.depth).into()
}

fn apply_command(
    command: &str, 
    forward: impl Fn(u32) -> Position,
    up: impl Fn(u32) -> Position,
    down: impl Fn(u32) -> Position,
  ) -> Position {
  let words = command.split(' ').collect::<Vec<_>>();
  let number = words[1].parse::<u32>().unwrap();
  match words[0] {
    "forward" => forward(number),
    "up" => up(number),
    "down" => down(number),
    _ => todo!("unhandled command"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_calculates_position() {
    let commands = commands();
    assert_eq!(150, part1(&commands));
  }

  #[test]
  fn part2_calculates_position() {
    let commands = commands();
    assert_eq!(900, part2(&commands));
  }

  fn commands() -> Vec<&'static str> {
    vec![
      "forward 5",
      "down 5",
      "forward 8",
      "up 3",
      "down 8",
      "forward 2",
    ]
  }
}
