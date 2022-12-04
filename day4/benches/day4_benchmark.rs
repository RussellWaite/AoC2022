use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4_solve::{day4_1_result, day4_2_result, cody1, cody2};

pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day4part1", |b| b.iter(|| day4_1_result(black_box("input"))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day4part2", |b| b.iter(|| day4_2_result(black_box("input"))));
}

pub fn criterion_benchmark_cody1(c: &mut Criterion) {
    c.bench_function("day4cody1", |b| b.iter(|| cody1(black_box("input"))));
}

pub fn criterion_benchmark_cody2(c: &mut Criterion) {
    c.bench_function("day4cody2", |b| b.iter(|| cody2(black_box("input"))));
}
criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2, criterion_benchmark_cody1, criterion_benchmark_cody2);
criterion_main!(benches);
