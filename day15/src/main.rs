use day15_solve::{day15_1_result, day15_2_result};
use std::time::Instant;

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day15_1_result(INPUT, 2_000_000));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    const MIN_XY: usize = 0;
    const MAX_XY: usize = 4_000_000;
    let start = Instant::now();
    println!(
        "The second answer is {}",
        day15_2_result(INPUT, MIN_XY, MAX_XY)
    );
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}
