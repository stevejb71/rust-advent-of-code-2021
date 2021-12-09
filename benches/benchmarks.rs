use advent_of_code_2021::*;
use advent_of_code_2021::common::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day1(c: &mut Criterion) {
  let input = include_str!("../inputs/day1.txt");
  let values = vec_of_usize(input);
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
  let values = vec_of_usize_one_line(input);
  c.bench_function("day 6 part 1", |b| b.iter(|| day6::part1(&values)));
  c.bench_function("day 6 part 2", |b| b.iter(|| day6::part2(&values)));
}

fn bench_day7(c: &mut Criterion) {
  let input = include_str!("../inputs/day7.txt");
  let values = vec_of_usize_one_line(input);
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

criterion_group!(benches, 
  bench_day1, bench_day2, bench_day3, bench_day4, bench_day5, bench_day6, bench_day7, 
  bench_day8, bench_day9
);
criterion_main!(benches);