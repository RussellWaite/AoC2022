use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day25_solve::day25_1_result;

const INPUT: &str = include_str!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day25part1", |b| b.iter(|| day25_1_result(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1);
criterion_main!(benches);
