use crate::common::*;

pub fn run(part: u8) -> u64 {
  let input_lines = include_str!("../inputs/day3.txt");
  run_part(part, input_lines, vec_of_str, part1, part2)
}

pub fn part1(inputs: &Vec<&str>) -> u64 {
  let num_bits = inputs[0].len();
  let mut ones_count = vec![0u16; num_bits];
  for number in inputs {
    number.chars().enumerate()
      .filter(|(_, bit)| *bit == '1')
      .for_each(|(i, _)| ones_count[i] += 1);
  }
  let mut gamma = 0;
  let mut epsilon = 0;
  let mut power = 1 << (num_bits - 1);
  for count in ones_count {
    if count as usize > inputs.len() / 2 {
      gamma += power;
    } else {
      epsilon += power;
    }
    power >>= 1;
  }
  epsilon * gamma
}

pub fn part2(inputs: &Vec<&str>) -> u64 {
  let inputs = inputs.iter().map(|x| x.chars().collect()).collect();
  let oxygen_rating = get_rating(&inputs, |x, y| x > y);
  let co2_rating = get_rating(&inputs, |x, y| x <= y);
  (oxygen_rating * co2_rating).into()
}

fn get_rating(inputs: &Vec<Vec<char>>, choose_category: fn(usize, usize) -> bool) -> u32 {
  let mut index = 0;
  let mut numbers = inputs.iter().map(|x| x).collect::<Vec<_>>();
  while numbers.len() > 1 {
    let mut zeroes = vec![];
    let mut ones = vec![];
    for number in numbers {
      let bit_category = if number[index] == '1' {
        &mut ones
      } else {
        &mut zeroes
      };
      bit_category.push(number);
    }
    index += 1;
    numbers = if choose_category(zeroes.len(), ones.len()) {
      zeroes
    } else {
      ones
    };
  }
  from_binary(numbers[0])
}

fn from_binary(bits: &Vec<char>) -> u32 {
  let mut power = 1 << (bits.len() - 1);
  let mut result = 0;
  for &bit in bits {
    if bit == '1' {
      result += power;
    }
    power >>= 1;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_calculates_power_consumption() {
    let numbers = sample();
    assert_eq!(198, part1(&numbers));
  }
  
  #[test]
  fn part2_calculates_life_support_rating() {
    let numbers = sample();
    assert_eq!(230, part2(&numbers));
  }

  fn sample() -> Vec<&'static str> {
    vec![
      "00100",
      "11110",
      "10110",
      "10111",
      "10101",
      "01111",
      "00111",
      "11100",
      "10000",
      "11001",
      "00010",
      "01010",
    ]
  }
}
