pub fn day4_1_result(path: &str) -> usize {
    let result = read_file(path);
    
    result.iter()
        .filter(|(a_start,a_end,b_start,b_end)| 
                a_start >= b_start && a_end <= b_end || 
                b_start >= a_start && b_end <= a_end)
        .count()
}

pub fn day4_2_result(path: &str) -> usize {
    let result = read_file(path);
    
    result.iter()
        .filter(|(a_start,a_end,b_start,b_end)| 
             a_start >= b_start && a_end <= b_end || 
             b_start >= a_start && b_end <= a_end || 
             a_start <= b_start && a_end >= b_start || 
             a_start >= b_start && a_start <= b_end
        )
        .count()
}

fn read_file(path: &str) -> Vec<(u8,u8,u8,u8)> {
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));

    let result: Vec<(u8,u8,u8,u8)> = data.lines()
        .map(|line| {
            let mut rooms = line.split(['-',',']);

            (rooms.next(),rooms.next(), rooms.next(), rooms.next())
        })
        .map(|(a,b,c,d)| 
             (
                 a.unwrap().parse::<u8>().unwrap(),
                 b.unwrap().parse::<u8>().unwrap(),
                 c.unwrap().parse::<u8>().unwrap(),
                 d.unwrap().parse::<u8>().unwrap()
             ))
        .collect();
    result
}

// ---------------------------------------------------------------------
// github.com/codyphobe's gists I learned about on twitch.tv/realvoidboy
//
// added some wrapper code to make it easier to benchmark it like mine
// ---------------------------------------------------------------------
pub fn cody1(path: &str) -> usize {
    
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));
    
    part1(&data)    
}

pub fn cody2(path: &str) -> usize {
    
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("couldn't open input file: {}", path));

    part2(&data)
}

pub fn part1(input: &str) -> usize {
    count_overlap_by(input, |(l, r)|
        (l.0 <= r.0 && l.1 >= r.1) ||
        (r.0 <= l.0 && r.1 >= l.1)
    )
}

pub fn part2(input: &str) -> usize {
    count_overlap_by(input, |(l, r)|
        !(l.1 < r.0 || r.1 < l.0)
    )
}

fn count_overlap_by<F>(input: &str, filter: F) -> usize
where
    F: Fn(&((u32, u32), (u32, u32))) -> bool
{
    input.lines()
        .flat_map(|p| p.split_once(',').and_then(|(l, r)| Some((
            l.split_once('-').and_then(|(a, b)| Some((
                a.parse().ok()?,
                b.parse().ok()?,
            )))?,
            r.split_once('-').and_then(|(a, b)| Some((
                a.parse().ok()?,
                b.parse().ok()?,
            )))?,
        ))))
        .filter(filter)
        .count()
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
