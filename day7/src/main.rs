use std::time::Instant;
use day7_solve::{day7_parse, day7_1_result, day7_2_result};

const INPUT: &str = include_str!("../input");

fn main() {
    let start = Instant::now();
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let fs = day7_parse(&lines);
    println!("The first answer is {}", day7_1_result(&fs));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day7_2_result(&fs));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

}
