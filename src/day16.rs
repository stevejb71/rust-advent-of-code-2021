pub fn run(part: u8) -> u64 {
  let input_line = include_str!("../inputs/day16.txt");
  match part {
    1 => part1(input_line),
    2 => part2(input_line),
    _ => panic!("no such part")
  }
}

#[derive(Debug, PartialEq)]
struct Packet {
  version: u8,
  value: Value,
}

#[derive(Debug, PartialEq)]
enum Value {
  Literal(u64),
  Operator(u8, Vec<Packet>)
}

impl Packet {
  fn total_version(&self) -> u64 {
    self.version as u64 + if let Value::Operator(_, subs) = &self.value {
      subs.iter().map(|p| p.total_version()).sum::<u64>()
    } else {
      0
    }
  }

  fn value(&self) -> u64 {
    match &self.value {
      Value::Literal(n) => *n,
      Value::Operator(op, subs) => {
        let mut sub_values = subs.iter().map(|s| s.value());
        match op {
          0 => sub_values.sum(),
          1 => sub_values.product(),
          2 => sub_values.min().unwrap(),
          3 => sub_values.max().unwrap(),
          _ => {
            let mut choose = |f: fn(u64, u64) -> bool| {
              if f(sub_values.next().unwrap(), sub_values.next().unwrap()) { 1 } else { 0 }
            };
            match op {
              5 => choose(|x, y| x > y),
              6 => choose(|x, y| x < y),
              7 => choose(|x, y| x == y),
              _ => panic!()
            }
          },
        }
      }
    }
  }
}

struct Bits {
  bits: Vec<u8>,
  index: usize,
}

impl Bits {
  fn new(bits: Vec<u8>) -> Bits {
    Bits { bits, index: 0 }
  }

  fn read_u8(&mut self, num: usize) -> u8 {
    self.read_u16(num) as u8
  }

  fn read_u16(&mut self, num: usize) -> u16 {
    let value = &self.bits[self.index..self.index+num];
    self.index += num;
    binary_to_u64(value) as u16
  }
}

pub fn part1(line: &str) -> u64 {
  let mut bits = Bits::new(hex_to_binary(line));
  let packet = read_packet(&mut bits);
  packet.total_version()
}

pub fn part2(line: &str) -> u64 {
  let mut bits = Bits::new(hex_to_binary(line));
  let packet = read_packet(&mut bits);
  packet.value()
}

fn read_packet(bits: &mut Bits) -> Packet {
  let version = bits.read_u8(3);
  let typ = bits.read_u8(3);
  if typ == 4 {
    let mut total = 0;
    loop {
      let digit = bits.read_u8(5) as u64;
      total *= 16;
      total += digit & 15;
      if digit & 16 == 0 {
        break;
      }
    }
    Packet { version, value: Value::Literal(total) }
  } else {
    let length_type = bits.read_u8(1);
    let length = bits.read_u16(if length_type == 0 { 15 } else { 11 });
    let mut subs = vec![];
    if length_type == 0 {
      let start = bits.index;
      while bits.index != start + (length as usize) {
        let sub = read_packet(bits);
        subs.push(sub);
      }
    } else {
      for _ in 0..length {
        let sub = read_packet(bits);
        subs.push(sub);
      }
    }
    Packet { version, value: Value::Operator(typ, subs) }
  }
}

fn binary_to_u64(n: &[u8]) -> u64 {
  n.iter().fold(0, |total, &n| 2 * total + (n as u64))
}

fn hex_to_binary(n: &str) -> Vec<u8> {
  let mut binary = vec![0; n.len() * 4];
  let mut i = 0;
  n.chars().for_each(|c| {
    let c = c.to_digit(16).unwrap() as u8;
    if c & 1 != 0 {
      binary[i + 3] = 1;
    }
    if c & 2 != 0 {
      binary[i + 2] = 1;
    }
    if c & 4 != 0 {
      binary[i + 1] = 1;
    }
    if c & 8 != 0 {
      binary[i] = 1;
    }
    i += 4;
  });
  binary
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hex_to_binary_works() {
    assert_eq!(vec![1,1,0,1,0,0,1,0,1,1,1,1,1,1,1,0,0,0,1,0,1,0,0,0], hex_to_binary("D2FE28"));
  }

  #[test]
  fn binary_to_u64_works() {
    assert_eq!(13, binary_to_u64(&[1,1,0,1]));
  }

  #[test]
  fn parses_literal_packet() {
    let mut bits = Bits::new(hex_to_binary("D2FE28"));
    let packet = read_packet(&mut bits);

    assert_eq!(6, packet.version);
    assert_eq!(Value::Literal(2021), packet.value);
  }

  #[test]
  fn parses_operator_packet_with_length_type_id_0() {
    let mut bits = Bits::new(hex_to_binary("38006F45291200"));
    let packet = read_packet(&mut bits);

    assert_eq!(1, packet.version);
    let subs = vec![
      Packet { 
        version: 6, 
        value: Value::Literal(10),
      },
      Packet { 
        version: 2, 
        value: Value::Literal(20),
      },
    ];
    assert_eq!(Value::Operator(6, subs), packet.value);
  }

  #[test]
  fn parses_operator_packet_with_length_type_id_1() {
    let mut bits = Bits::new(hex_to_binary("EE00D40C823060"));
    let packet = read_packet(&mut bits);

    let subs = vec![
      Packet { 
        version: 2, 
        value: Value::Literal(1),
      },
      Packet { 
        version: 4, 
        value: Value::Literal(2),
      },
      Packet { 
        version: 1, 
        value: Value::Literal(3),
      },
    ];
    assert_eq!(Value::Operator(3, subs), packet.value);
  }

  #[test]
  fn part1_gets_version_sum() {
    let sample = "8A004A801A8002F478";
    assert_eq!(16, part1(&sample));
  }

  #[test]
  fn part2_gets_sum_value() {
    let sample = "C200B40A82";
    assert_eq!(3, part2(&sample));
  }
}
