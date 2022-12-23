use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day16_solve::{day16_1_result, day16_2_result};

const INPUT: &str = include_str!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day16part1", |b| b.iter(|| day16_1_result(black_box(INPUT))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day16part2", |b| b.iter(|| day16_2_result(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
