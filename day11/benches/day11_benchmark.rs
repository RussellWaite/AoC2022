use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day11_solve::{day11_1_result, day11_2_result};

const INPUT: &str = include_str!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day11part1", |b| b.iter(|| day11_1_result(black_box(INPUT))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day11part2", |b| b.iter(|| day11_2_result(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
