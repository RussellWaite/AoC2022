use std::time::Instant;
use day2_solve::{day2_2_result, day2_1_result};

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day2_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day2_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

