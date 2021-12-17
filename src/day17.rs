pub fn run(part: u8) -> u64 {
  let input_line = include_str!("../inputs/day17.txt");
  match part {
    1 => part1(input_line),
    2 => part2(input_line),
    _ => panic!("no such part")
  }
}

pub fn part1(input: &str) -> u64 {
  let mut max = isize::MIN;
  let mut on_hit = |probe: &Probe, _: &(isize, isize)| {
    if probe.y_max > max {
      max = probe.y_max;
    }
  };
  solve(input, &mut on_hit);
  max as u64
}

pub fn part2(input: &str) -> u64 {
  let mut vel_values = vec![];
  let mut on_hit = |_: &Probe, vel: &(isize, isize)| {
    vel_values.push(vel.clone());
  };
  solve(input, &mut on_hit);
  vel_values.len() as u64
}

fn solve(input: &str, on_hit: &mut impl FnMut(&Probe, &(isize, isize))) {
  let target_area = parse(input);
  for vx in 0..=target_area.x_range.1 {
    for vy in -target_area.x_range.1..=target_area.x_range.1 {
      let vel = (vx, vy);
      let mut probe = Probe::new(vel);
      if step_until_end(&mut probe, &target_area) {
        on_hit(&probe, &vel);
      }
    }
  }
}

fn step_until_end(probe: &mut Probe, target_area: &TargetArea) -> bool {
  loop {
    if probe.in_target(target_area) {
      return true;
    }
    if probe.missed_target(target_area) {
      return false;
    }
    probe.step();
  }
}

struct TargetArea {
  x_range: (isize, isize),
  y_range: (isize, isize),
}

struct Probe {
  pos: (isize, isize),
  vel: (isize, isize),
  y_max: isize,
}

impl Probe {
  fn new(vel: (isize, isize)) -> Probe {
    Probe { pos: (0,0), vel, y_max: 0 }
  }

  fn step(&mut self) {
    self.pos.0 += self.vel.0;
    self.pos.1 += self.vel.1;
    self.vel.0 -= self.vel.0.signum();
    self.vel.1 -= 1;

    if self.pos.1 > self.y_max {
      self.y_max = self.pos.1;
    }
  }

  fn in_target(&self, target_area: &TargetArea) -> bool {
    target_area.x_range.0 <= self.pos.0 && self.pos.0 <= target_area.x_range.1 &&
    target_area.y_range.0 <= self.pos.1 && self.pos.1 <= target_area.y_range.1
  }

  fn missed_target(&self, target_area: &TargetArea) -> bool {
    self.pos.1 < target_area.y_range.0
  }
}

fn parse(input: &str) -> TargetArea {
  let input = &input[15..];
  let (x_range, y_range) = input.split_once(", y=").unwrap();
  fn parse_range(r: &str) -> (isize, isize) {
    let (lo, hi) = r.split_once("..").unwrap();
    (lo.parse().unwrap(), hi.parse().unwrap())
  }
  let x_range = parse_range(x_range);
  let y_range = parse_range(y_range);
  TargetArea { x_range, y_range }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_finds_max_y_pos() {
    assert_eq!(45, part1("target area: x=20..30, y=-10..-5"));
  }

  #[test]
  fn part2_finds_distinct_vel_values() {
    assert_eq!(112, part2("target area: x=20..30, y=-10..-5"));
  }
}
