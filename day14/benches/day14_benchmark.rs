use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day14_solve::{day14_1_result, day14_2_result};

const INPUT: &[u8] = include_bytes!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day14part1", |b| b.iter(|| day14_1_result(black_box(INPUT))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day14part2", |b| b.iter(|| day14_2_result(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
