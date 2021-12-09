pub fn run_part<'a, A>(part: usize, inputs: &'a str, parse: fn(&'a str) -> A, part1: fn(&A) -> usize, part2: fn(&A) -> usize) -> usize {
  let parsed = parse(inputs);
  match part {
    1 => part1(&parsed),
    2 => part2(&parsed),
    _ => panic!("no such part")
  }
}

pub fn vec_of_usize(str: &str) -> Vec<usize> {
  str.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

pub fn vec_of_str<'a>(str: &'a str) -> Vec<&'a str> {
  str.lines().collect()
}

pub fn vec_of_rows_of_digits<'a>(str: &'a str) -> Vec<Vec<u8>> {
  str.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>()).collect()
}