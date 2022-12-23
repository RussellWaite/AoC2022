use day16_solve::{day16_1_result, day16_2_result};
use std::time::Instant;

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day16_1_result(INPUT, "AA")); // is AA always correct?
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day16_2_result(INPUT, "AA"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}
