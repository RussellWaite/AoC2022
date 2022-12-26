use day25_solve::day25_1_result;
use std::time::Instant;

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    let (human, snafu) = day25_1_result(INPUT);
    println!("The first answer is {} ... {}", human, snafu); // is AA always correct?
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
