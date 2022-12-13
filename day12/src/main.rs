use std::time::Instant;
use day12_solve::{day12_1_result, day12_2_result};

const INPUT: &[u8] = include_bytes!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day12_1_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day12_2_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
