pub fn day5_1_result(path: &str) -> String {
    day5_1(path, false)
}

pub fn day5_2_result(path: &str) -> String {
    day5_2(path, false)
}

// fudge for testing as didn't code around dynamic sized input
pub fn day5_1(path: &str, test: bool) -> String {
    let (stack, instructions)  = if test {read_file(path, 3)} else {read_file(path, 8)};
    
    let mut stacks = if test {create_stacks_test(stack)} else {create_stacks(stack)};
    
    instructions.iter()
        .map(|i| (i[0],i[1], i[2]))
        .for_each(|(count, from, to)| {
            (0..count).for_each(|_| {
                if let Some(a) = stacks[from as usize - 1].pop() {
                    stacks[to as usize - 1].push(a);
                }
            });
        });

    stacks.iter().map(|stack| *stack.last().unwrap() as char).collect() 
}

// fudge for testing as didn't code around dynamic sized input
fn day5_2(path: &str, test: bool) -> String {
    let (stack, instructions)  = if test {read_file(path, 3)} else {read_file(path, 8)};
    
    let mut stacks = if test {create_stacks_test(stack)} else {create_stacks(stack)};
    
    instructions.iter()
        .map(|i| (i[0],i[1], i[2]))
        .for_each(|(count, from, to)| {
                let index = stacks[from as usize - 1].len() - count as usize;
                let mut movement = stacks[from as usize - 1].drain(index..).collect();
                stacks[to as usize - 1].append(&mut movement);
        });

    stacks.iter().map(|stack| *stack.last().unwrap() as char).collect()
}

fn create_stacks(data: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = vec![vec![];9];
    data.iter()
        .rev()
        .flat_map(|bytes| 
                    [
                        bytes[1],bytes[5],bytes[9],
                        bytes[13],bytes[17],bytes[21],
                        bytes[25],bytes[29],bytes[33]
                    ].to_vec())
        .enumerate()
        .for_each(|(index,char)| {
            if char > 64u8 && char < 91u8 {
                transposed[index % 9].push(char);
            }
        });

    transposed
}

// this is sickeningly dirty
fn create_stacks_test(data: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = vec![vec![];3];
    data.iter()
        .rev()
        .flat_map(|bytes| [bytes[1],bytes[5],bytes[9]].to_vec())
        .enumerate()
        .for_each(|(index,char)| {
            if char > 64u8 && char < 91u8 {
                transposed[index % 3].push(char);
            }
        });
    transposed
}

fn read_file(path: &str, max_start_stack_size: usize) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    
    let raw_data = std::fs::read_to_string(path)
        .unwrap_or_else(
            |_| panic!("couldn't open input file: {}", path));    
    
    let state = raw_data.lines()
            .take(max_start_stack_size)
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

    let instructions = raw_data.lines()
            .skip(max_start_stack_size+2)
            .map(|line| line.split(' '))
            .map(|words| words.filter_map(
                    |word| word.parse::<u8>().ok()
                    ).collect()
                )
            .collect::<Vec<Vec<u8>>>();

    (state, instructions)
}

#[test]
fn read_file_test() {
    let (stack, instructions) = read_file("test_input", 3);
    // assert_eq!(stack.len(), 3);
}

#[test]
fn day5_1_result_1_test() {
    assert_eq!(day5_1("test_input", true), "CMZ");

    assert_eq!(day5_1_result("input"), "MQTPGLLDN");
}

#[test]
fn day5_2_result_1_test() {
    assert_eq!(day5_2("test_input", true), "MCD");
    
    assert_eq!(day5_2_result("input"), "LVZPSTTCZ");
}
