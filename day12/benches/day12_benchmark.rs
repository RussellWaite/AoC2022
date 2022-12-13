use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day12_solve::{day12_1_result, day12_2_result};

const INPUT: &[u8] = include_bytes!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    c.bench_function("day12part1", |b| b.iter(|| day12_1_result(black_box(INPUT))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    c.bench_function("day12part2", |b| b.iter(|| day12_2_result(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2);
criterion_main!(benches);
