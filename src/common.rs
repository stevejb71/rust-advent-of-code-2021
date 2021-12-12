pub fn run_part<'a, A>(part: u8, inputs: &'a str, parse: fn(&'a str) -> A, part1: fn(&A) -> u64, part2: fn(&A) -> u64) -> u64 {
  let parsed = parse(inputs);
  match part {
    1 => part1(&parsed),
    2 => part2(&parsed),
    _ => panic!("no such part")
  }
}

pub fn vec_of_u64_one_line(str: &str) -> Vec<u64> {
  str.split(",").map(|x| x.parse::<u64>().unwrap()).collect()
}

pub fn vec_of_u64(str: &str) -> Vec<u64> {
  str.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

pub fn vec_of_str<'a>(str: &'a str) -> Vec<&'a str> {
  str.lines().collect()
}

pub fn vec_of_rows_of_digits<'a>(str: &'a str) -> Vec<Vec<u8>> {
  str.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>()).collect()
}

pub fn vec_of_u8_lines_from_digits(str: &str) -> Vec<Vec<u8>> {
  str.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
    .collect()
}