use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day6_solve::{day6_1_result, day6_2_result};

const INPUT: &str = include_str!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day6part1", |b| b.iter(|| day6_1_result(black_box(INPUT))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day6part2", |b| b.iter(|| day6_2_result(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
