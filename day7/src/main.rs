use std::time::Instant;
use day7_solve::{day7_1_result, day7_2_result};

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day7_1_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day7_2_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
