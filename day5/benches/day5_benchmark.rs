use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5_solve::{day5_1_result, day5_2_result};

pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day5part1", |b| b.iter(|| day5_1_result(black_box("input"))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day5part2", |b| b.iter(|| day5_2_result(black_box("input"))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
