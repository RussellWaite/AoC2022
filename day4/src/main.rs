use std::time::Instant;
use day4_solve::{day4_1_result, day4_2_result};

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day4_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day4_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}
