pub fn day5_1_result(path: &str) -> String {
    let (stack, instructions) = read_file(path);
    let mut stacks = create_stacks(stack);

    instructions
        .iter()
        .map(|i| (i[0], i[1], i[2]))
        .for_each(|(count, from, to)| {
            (0..count).for_each(|_| {
                if let Some(a) = stacks[from as usize - 1].pop() {
                    stacks[to as usize - 1].push(a);
                }
            });
        });

    stacks
        .iter()
        .map(|stack| *stack.last().unwrap_or(&32u8) as char)
        .collect()
}

pub fn day5_2_result(path: &str) -> String {
    let (stack, instructions) = read_file(path);

    let mut stacks = create_stacks(stack);

    instructions
        .iter()
        .map(|i| (i[0], i[1], i[2]))
        .for_each(|(count, from, to)| {
            let index = stacks[from as usize - 1].len() - count as usize;
            let mut movement = stacks[from as usize - 1].drain(index..).collect();
            stacks[to as usize - 1].append(&mut movement);
        });

    stacks
        .iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect()
}

fn create_stacks(data: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let num_of_stacks = 1 + data.first().unwrap().len() / 4;
    let mut transposed: Vec<Vec<u8>> = vec![vec![]; num_of_stacks];
    data.iter()
        .rev()
        .skip(1) // the line with index numbers
        .flat_map(|bytes| bytes.chunks(4).map(|quad| quad[1]).collect::<Vec<u8>>())
        .enumerate()
        .for_each(|(index, char)| {
            if char > 64u8 && char < 91u8 {
                transposed[index % num_of_stacks].push(char);
            }
        });

    transposed
}

fn read_file(path: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let raw_data = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));

    let state = raw_data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let instructions = raw_data
        .lines()
        .skip_while(|line| !line.starts_with("move "))
        .map(|line| line.split(' '))
        .map(|words| words.filter_map(|word| word.parse::<u8>().ok()).collect())
        .collect::<Vec<Vec<u8>>>();

    (state, instructions)
}

#[test]
fn read_file_test() {
    let (stack, instructions) = read_file("test_input");
    // assert_eq!(stack.len(), 3);
}

#[test]
fn day5_1_result_1_test() {
    assert_eq!(day5_1_result("test_input"), "CMZ");

    assert_eq!(day5_1_result("input"), "MQTPGLLDN");
}

#[test]
fn day5_2_result_1_test() {
    assert_eq!(day5_2_result("test_input"), "MCD");

    assert_eq!(day5_2_result("input"), "LVZPSTTCZ");
}
