#[inline]
pub fn day2_1_result(path: &str) -> u64 {
    calculate_score_based_on_strategy(path, true)
}

#[inline]
pub fn day2_2_result(path: &str) -> u64 { 
    calculate_score_based_on_strategy(path, false)
}

fn calculate_score_based_on_strategy(path: &str, x_is_choice: bool) -> u64 {
    let data = read_file(path);
    
    let result = data.lines()
        .map(|line| {
            match line {
                "A X" => (4,3),
                "A Y" => (8,4),
                "A Z" => (3,8),
                "B X" => (1,1),
                "B Y" => (5,5),
                "B Z" => (9,9),
                "C X" => (7,2),
                "C Y" => (2,6),
                "C Z" => (6,7),
                _ => panic!("whatever you've read in - it's BAD...")
            }
        })
        .map(|(choice, strategy)| if x_is_choice { choice } else { strategy });

    result.sum()
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path))
}

#[test]
fn read_file_test() {
    let rawdata = read_file("test_input");
    let mut lines = rawdata.lines();
    assert_eq!(lines.next(), Some("A Y"));
    assert_eq!(lines.next(), Some("B X"));
    assert_eq!(lines.next(), Some("C Z"));
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
