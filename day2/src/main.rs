use std::time::Instant;

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

fn day2_1_result(path: &str) -> u64 {
    let cals = read_file(path);
    cals.iter().map(|(them, us)| calculate_score(them, us)).sum()
}

fn day2_2_result(path: &str) -> u64 { 
    let cals = read_file(path);
    cals.iter().map(|(them, us)| calculate_score_pt2(them, us)).sum()
}

// this is quite verbose...
fn calculate_score(opponent: &str, choice: &str) -> u64 {
// 1 for X, 2 for Y, 3 for Z
// 0 for lose, 3 draw, 6 win
    match opponent {
        "A" => {
            match choice {
                "X" => 4, // 1 + draw 3 
                "Y" => 8, // 2 + win 6
                "Z" => 3, // 3 + loss 0
                _ => panic!("invaid choice")
            }
        },
        "B" => {
            match choice {
                "X" => 1, // 1 + loss 0
                "Y" => 5, // 2 + draw 3
                "Z" => 9, // 3 + win 6
                _ => panic!("invaid choice")
            }
        },
        "C" => { 
            match choice {
                "X" => 7, // 1 + win 6
                "Y" => 2, // 2 + loss 0
                "Z" => 6, // 3 + draw 3
                _ => panic!("invaid choice")
            }
        },
        _ => panic!("that's not a valid opponent value")    
    }
}

// copying this is nasty as it's so much boilerplate
fn calculate_score_pt2(opponent: &str, outcome: &str) -> u64 {
// x lose, y draw, z win
    match opponent {
        "A" => {
            match outcome {
                "X" => 3, // 0 loss as we pick scissors 3 
                "Y" => 4, // 3 draw as we pick rock 1
                "Z" => 8, // 6 win as we pick paper 2
                _ => panic!("invaid outcome")
            }
        },
        "B" => {
            match outcome {
                "X" => 1, // 0 loss, rock 1
                "Y" => 5, // 3 draw,paper 2 
                "Z" => 9, // 6 win, scissors 3
                _ => panic!("invaid outcome")
            }
        },
        "C" => { 
            match outcome {
                "X" => 2, // 0 loss, paper 2
                "Y" => 6, // 3 draw, scissors 3
                "Z" => 7, // 6 win, rock 1
                _ => panic!("invaid outcome")
            }
        },
        _ => panic!("that's not a valid opponent value")    
    }
}

fn read_file(path: &str) -> Vec<(String, String)> {
    let contents = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));

    let result: Vec<(String, String)> = contents.lines()
        .map(|line| 
                {
                    // let line = line.unwrap();
                    let temp:Vec<&str> = line.split(' ').collect();
                    ((temp[0]).to_owned(), (temp[1]).to_owned())
                })
        .collect()
    ;
    result
}

#[test]
fn read_file_test() {
    let cals = read_file("test_input");
    assert_eq!(cals.len(), 3);
}

#[test]
fn day2_1_result_1_test() {
    assert_eq!(day2_1_result("test_input"), 15);
    assert_eq!(day2_1_result("input"), 9651);
}

#[test]
fn day2_2_result_1_test() {
    assert_eq!(day2_2_result("test_input"), 12);
    assert_eq!(day2_2_result("input"), 10560); 
}
