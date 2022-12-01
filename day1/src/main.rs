use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day1_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day1_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

fn day1_1_result(path: &str) -> u64 {
    let cals = read_file(path);
    *cals.iter().max().unwrap()
}

fn day1_2_result(path: &str) -> u64 { 
    let mut cals = read_file(path);
    cals.sort();
    cals[cals.len()-3..].iter().sum()
}

// this is not nice...
fn read_file(path: &str) -> Vec<u64> {
    let contents = &std::fs::read_to_string(path).unwrap();

    let result: Vec<u64> = contents.lines().fold(vec![0], |mut cals, line| {
        match line.parse::<u64>() {
            Ok(x) => {
                let c = cals.pop().unwrap() + x;
                cals.push(c);
            }
            Err(_) => cals.push(0),
        };
        cals
    });
    result
}


#[test]
fn read_file_test() {
    let cals = read_file("test_input");
    assert_eq!(cals.len(), 5);
}

#[test]
fn day1_1_result_1_test() {
    assert_eq!(day1_1_result("test_input"), 24000);
    assert_eq!(day1_1_result("input"), 67622);
}

#[test]
fn day1_2_result_1_test() {
    assert_eq!(day1_2_result("test_input"), 45000);
    assert_eq!(day1_2_result("input"), 201491); //66377 + 67492 + 67622);
}
