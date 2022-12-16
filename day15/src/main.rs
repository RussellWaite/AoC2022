use std::time::Instant;
use day15_solve::{day15_1_result, day15_2_result};

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day15_1_result(INPUT, 2_000_000));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day15_2_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
