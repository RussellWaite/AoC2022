use day17_solve::{day17_1_result, day17_2_result};
use std::time::Instant;

const INPUT: &[u8] = include_bytes!("../input");

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day17_1_result(INPUT)); // is AA always correct?
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day17_2_result(INPUT));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    // const TEST_INPUT: &[u8] = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".as_bytes();
    // println!("TEMP EXPERIMENT: {}:", day17_2_result(TEST_INPUT));
}
