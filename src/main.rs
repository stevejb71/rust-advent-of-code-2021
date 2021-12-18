fn main() {
  use std::env;  
  use advent_of_code_2021::*;

  let args: Vec<String> = env::args().collect();
  let day = args[1].parse::<u32>().unwrap();
  let part = args[2].parse::<u8>().unwrap();
  let run = match day {
    1 => day1::run,
    2 => day2::run,
    3 => day3::run,
    4 => day4::run,
    5 => day5::run,
    6 => day6::run,
    7 => day7::run,
    8 => day8::run,
    9 => day9::run,
    10 => day10::run,
    11 => day11::run,
    12 => day12::run,
    13 => day13::run,
    14 => day14::run,
    15 => day15::run,
    16 => day16::run,
    17 => day17::run,
    18 => day18::run,
    _ => panic!("Unimplemented")
  };
  let result = run(part);
  println!("Result for day {}, part {} is: {}", day, part, result);
}
