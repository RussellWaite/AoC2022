use std::time::Instant;
use day10_solve::{day10_1_result, day10_2_result};

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day10_1_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day10_2_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
