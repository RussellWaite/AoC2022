use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day7_solve::{day7_parse, day7_1_result, day7_2_result, day7_all};

const INPUT: &str = include_str!("../input");
pub fn criterion_benchmark_part1(c: &mut Criterion) {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let fs = day7_parse(&lines);
    c.bench_function("day7part1", |b| b.iter(|| day7_1_result(black_box(&fs))));
}

pub fn criterion_benchmark_part2(c: &mut Criterion) {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let fs = day7_parse(&lines);
    c.bench_function("day7part2", |b| b.iter(|| day7_2_result(black_box(&fs))));
}

pub fn criterion_benchmark_all(c: &mut Criterion) {
    c.bench_function("day7all", |b| b.iter(|| day7_all()));
}
criterion_group!(benches, criterion_benchmark_part1, criterion_benchmark_part2, criterion_benchmark_all);
criterion_main!(benches);
