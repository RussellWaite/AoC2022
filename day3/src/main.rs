use std::time::Instant;
use day3_solve::{day3_1_result, day3_2_result};

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day3_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day3_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}
