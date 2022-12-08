use std::time::Instant;
use day8_solve::{day8_1_result, day8_2_result};

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day8_1_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day8_2_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
