pub fn run(part: u8) -> u64 {
  let input_line = include_str!("../inputs/day16.txt");
  match part {
    1 => part1(input_line),
    2 => part2(input_line),
    _ => panic!("no such part")
  }
}

pub fn part1(_line: &str) -> u64 {
  0
}

pub fn part2(_line: &str) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_gets_version_sum() {
    let sample = "8A004A801A8002F478";
    assert_eq!(16, part1(&sample));
  }
}
