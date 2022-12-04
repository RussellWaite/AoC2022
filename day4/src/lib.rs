pub fn day4_1_result(path: &str) -> usize {
    let result = read_file(path);
    
    result.iter()
        .filter(|((a_start,a_end),(b_start,b_end))| 
                a_start >=b_start && a_end<=b_end || 
                b_start >= a_start && b_end<= a_end)
        .count()
}

pub fn day4_2_result(path: &str) -> usize {
    let result = read_file(path);
    
    result.iter()
        .filter(|((a_start,a_end),(b_start,b_end))| 
             a_start >=b_start && a_end<=b_end || 
             b_start >= a_start && b_end<= a_end || 
             a_start <= b_start && a_end>= b_start || 
             a_start >= b_start && a_start <= b_end
        )
        .count()
}

fn read_file(path: &str) -> Vec<((u64,u64),(u64,u64))> {
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));

    let result: Vec<((u64,u64),(u64,u64))> = data.lines()
        .map(|line| {
            let rooms: Vec<&str> = line.split(['-',',']).collect();
            
            match rooms.as_slice() {
                [a,b,c,d] => {
                    (
                        (a.parse::<u64>().unwrap(),b.parse::<u64>().unwrap()),
                        (c.parse::<u64>().unwrap(),d.parse::<u64>().unwrap())
                    )
                },
                _ => panic!("wasn't 4 numbers on the lines - GAME OVER"),
            }
        }).collect();
    result
}

#[test]
fn read_file_test() {
    let result = read_file("test_input");
    assert_eq!(result.len(), 6);
}

#[test]
fn day4_1_result_1_test() {
    assert_eq!(day4_1_result("test_input"), 2);

    assert_eq!(day4_1_result("input"), 475);
}

#[test]
fn day4_2_result_1_test() {
     assert_eq!(day4_2_result("test_input"), 4);

    assert_eq!(day4_2_result("input"), 825);
}
