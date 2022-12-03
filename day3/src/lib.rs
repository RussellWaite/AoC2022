pub fn day3_1_result(path: &str) -> u64 {
    let result = read_file(path);
    result.iter()
        .map(|chars| {
            let compartments = chars.split_at(chars.len() / 2);
            compartments.0.iter()
                .find_map(|x| if compartments.1.contains(x) { Some(*x as u64) } else { None }).unwrap()
        })
        .sum()
}

pub fn day3_2_result(path: &str) -> u64 {
    let result = read_file(path);
    result.chunks(3)
        .map(|chunks| {
            let mut result = [0u8;53];
            
            // set bit for each elf's backback when an item is present
            chunks[0].iter().for_each(|x| result[*x as usize] |= 0b001);
            chunks[1].iter().for_each(|x| result[*x as usize] |= 0b010);
            chunks[2].iter().for_each(|x| result[*x as usize] |= 0b100);
            
            // find a number that has all 3 bits set
            result.iter()
                .enumerate()
                .find_map(|(i, &x)| if x == 0b111 { Some(i as u64) } else { None })
                .unwrap() // always with the unwrap... pfft
        }).sum()
}

#[inline]
fn read_file(path: &str) -> Vec<Vec<u8>> {
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));

    let ascii_mapper = |&x| if x > 90u8 { x - 96u8 } else { x - 38u8 };

    let result = data
        .lines()
        .map(|line| {
            let chars = line.as_bytes();
            chars.iter().map(ascii_mapper).collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    result
}

#[test]
fn read_file_test() {
    let result = read_file("test_input");
    assert_eq!(result.len(),6 );
}

#[test]
fn day3_1_result_1_test() {
    assert_eq!(day3_1_result("test_input"), 157);

    assert_eq!(day3_1_result("input"), 7980);
}

#[test]
fn day3_2_result_1_test() {
    assert_eq!(day3_2_result("test_input"), 70);

    assert_eq!(day3_2_result("input"), 2881);
}
