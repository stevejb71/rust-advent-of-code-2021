use advent_of_code_2021::*;
use advent_of_code_2021::common::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day1(c: &mut Criterion) {
  let input = include_str!("../inputs/day1.txt");
  let values = vec_of_u64(input);
  c.bench_function("day 1 part 1", |b| b.iter(|| day1::part1(&values)));
  c.bench_function("day 1 part 2", |b| b.iter(|| day1::part2(&values)));
}

fn bench_day2(c: &mut Criterion) {
  let input = include_str!("../inputs/day2.txt");
  let values = vec_of_str(input);
  c.bench_function("day 2 part 1", |b| b.iter(|| day2::part1(&values)));
  c.bench_function("day 2 part 2", |b| b.iter(|| day2::part2(&values)));
}

fn bench_day3(c: &mut Criterion) {
  let input = include_str!("../inputs/day3.txt");
  let values = vec_of_str(input);
  c.bench_function("day 3 part 1", |b| b.iter(|| day3::part1(&values)));
  c.bench_function("day 3 part 2", |b| b.iter(|| day3::part2(&values)));
}

fn bench_day4(c: &mut Criterion) {
  let input = include_str!("../inputs/day4.txt");
  let values = vec_of_str(input);
  c.bench_function("day 4 part 1", |b| b.iter(|| day4::part1(&values)));
  c.bench_function("day 4 part 2", |b| b.iter(|| day4::part2(&values)));
}

fn bench_day5(c: &mut Criterion) {
  let input = include_str!("../inputs/day5.txt");
  let values = vec_of_str(input);
  c.bench_function("day 5 part 1", |b| b.iter(|| day5::part1(&values)));
  c.bench_function("day 5 part 2", |b| b.iter(|| day5::part2(&values)));
}

fn bench_day6(c: &mut Criterion) {
  let input = include_str!("../inputs/day6.txt");
  let values = vec_of_u64_one_line(input);
  c.bench_function("day 6 part 1", |b| b.iter(|| day6::part1(&values)));
  c.bench_function("day 6 part 2", |b| b.iter(|| day6::part2(&values)));
}

fn bench_day7(c: &mut Criterion) {
  let input = include_str!("../inputs/day7.txt");
  let values = vec_of_u64_one_line(input);
  c.bench_function("day 7 part 1", |b| b.iter(|| day7::part1(&values)));
  c.bench_function("day 7 part 2", |b| b.iter(|| day7::part2(&values)));
}

fn bench_day8(c: &mut Criterion) {
  let input = include_str!("../inputs/day8.txt");
  let values = vec_of_str(input);
  c.bench_function("day 8 part 1", |b| b.iter(|| day8::part1(&values)));
  c.bench_function("day 8 part 2", |b| b.iter(|| day8::part2(&values)));
}

fn bench_day9(c: &mut Criterion) {
  let input = include_str!("../inputs/day9.txt");
  let values = vec_of_rows_of_digits(input);
  c.bench_function("day 9 part 1", |b| b.iter(|| day9::part1(&values)));
  c.bench_function("day 9 part 2", |b| b.iter(|| day9::part2(&values)));
}

fn bench_day10(c: &mut Criterion) {
  let input = include_str!("../inputs/day10.txt");
  let values = vec_of_str(input);
  c.bench_function("day 10 part 1", |b| b.iter(|| day10::part1(&values)));
  c.bench_function("day 10 part 2", |b| b.iter(|| day10::part2(&values)));
}

fn bench_day11(c: &mut Criterion) {
  let input = include_str!("../inputs/day11.txt");
  let values = vec_of_u8_lines_from_digits(input);
  c.bench_function("day 11 part 1", |b| b.iter(|| day11::part1(&values)));
  c.bench_function("day 11 part 2", |b| b.iter(|| day11::part2(&values)));
}

fn bench_day12(c: &mut Criterion) {
  let input = include_str!("../inputs/day12.txt");
  let values = vec_of_str(input);
  c.bench_function("day 12", |b| b.iter(|| day12::part1(&values)));
  let mut group = c.benchmark_group("day 12 part 2");
  group.sample_size(20);
  group.bench_function("part 2", |b| b.iter(|| day12::part2(&values)));
}

fn bench_day13(c: &mut Criterion) {
  let input = include_str!("../inputs/day13.txt");
  let values = vec_of_str(input);
  c.bench_function("day 13 part 1", |b| b.iter(|| day13::part1(&values)));
  c.bench_function("day 13 part 2", |b| b.iter(|| day13::part2(&values)));
}

fn bench_day14(c: &mut Criterion) {
  let input = include_str!("../inputs/day14.txt");
  let values = vec_of_str(input);
  c.bench_function("day 14 part 1", |b| b.iter(|| day14::part1(&values)));
  c.bench_function("day 14 part 2", |b| b.iter(|| day14::part2(&values)));
}

fn bench_day15(c: &mut Criterion) {
  let input = include_str!("../inputs/day15.txt");
  let values = vec_of_str(input);
  c.bench_function("day 15 part 1", |b| b.iter(|| day15::part1(&values)));
  c.bench_function("day 15 part 2", |b| b.iter(|| day15::part2(&values)));
}

fn bench_day16(c: &mut Criterion) {
  let input = include_str!("../inputs/day16.txt");
  c.bench_function("day 16 part 1", |b| b.iter(|| day16::part1(input)));
  c.bench_function("day 16 part 2", |b| b.iter(|| day16::part2(input)));
}

fn bench_day17(c: &mut Criterion) {
  let input = include_str!("../inputs/day17.txt");
  c.bench_function("day 17 part 1", |b| b.iter(|| day17::part1(input)));
  c.bench_function("day 17 part 2", |b| b.iter(|| day17::part2(input)));
}

fn run_all(c: &mut Criterion) {
  let day13_input = vec_of_str(include_str!("../inputs/day13.txt"));
  let mut group = c.benchmark_group("ALL");
  group.sample_size(10);
  group.bench_function("all", |b| b.iter(|| {
    day1::run(1);
    day1::run(2);
    day2::run(1);
    day2::run(2);
    day3::run(1);
    day3::run(2);
    day4::run(1);
    day4::run(2);
    day5::run(1);
    day5::run(2);
    day6::run(1);
    day6::run(2);
    day7::run(1);
    day7::run(2);
    day8::run(1);
    day8::run(2);
    day9::run(1);
    day9::run(2);
    day10::run(1);
    day10::run(2);
    day11::run(1);
    day11::run(2);
    day12::run(1);
    day12::run(2);
    day13::part1(&day13_input);
    day13::part2_noprint(&day13_input);
    day14::run(1);
    day14::run(2);
    day15::run(1);
    day15::run(2);
    day16::run(1);
    day16::run(2);
    day17::run(1);
    day17::run(2);
  }));
}

criterion_group!(benches, 
  bench_day1,  bench_day2,  bench_day3,  bench_day4,  bench_day5,  bench_day6,  bench_day7, 
  bench_day8,  bench_day9,  bench_day10, bench_day11, bench_day12, bench_day13, bench_day14,
  bench_day15, bench_day16, bench_day17, 
  run_all
);
criterion_main!(benches);