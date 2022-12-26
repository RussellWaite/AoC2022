pub fn day25_1_result(data: &str) -> (i64, String) {
    //let snafu_lookup: &[char; 5] = &['=','-','0','1','2'];
    let snafu_lookup: &str = "=-012";
    let base10: i64 = data.lines()
        .map(|line| { 
            let length = line.len() as i64;
            line.chars()
                .fold((0i64, length - 1), |(mut total, position), c| {
                    let adjustment = 5i64.pow(position as u32);
                    // index is giving us a number 2 out - subtract to get actual snafu value
                    total += (snafu_lookup.find(c).unwrap() as i64 - 2) * adjustment;
                    (total, position - 1)
                }).0
        })
        .sum();
    let snafu = to_snafu(base10);
    (base10, snafu)
}

fn to_snafu(input: i64) -> String {
    let five = 5u64;
    let mut remaining = input;
    let significant_digits = (input as f32 * 2.0).log(5.0).ceil() as u32;
    let mut characters: Vec<char> = vec![];
    for digit in (0..=significant_digits).rev() {
        
        let this_column = five.pow(digit) as i64;

        match remaining {
            0 => {
                characters.push('0');
                continue;
            },
            x if x > 0 => {    // remainder is positive.
                if remaining.abs() < (this_column + 1)/2 {
                    if !characters.is_empty() {
                        characters.push('0');
                    }
                    continue;
                }
                remaining -= this_column;
                if remaining < (this_column + 1) / 2 { 
                    characters.push('1');
                    continue;
                }
                remaining -= this_column;
                characters.push('2');
            },
            x if x < 0 => {     // remainder is negative.
                if remaining.abs() < (this_column + 1) / 2 {
                    if !characters.is_empty() {
                        characters.push('0');
                    }
                    continue;
                }
                remaining += this_column;
                if remaining.abs() < (this_column +1 ) / 2 { 
                    characters.push('-');
                    continue;
                }
                remaining += this_column;
                characters.push('=');
            },
            _ => panic!("Rust being rust, must have all matches included, like those integers that aren't greater than zero, nor zero, nor less than zero...")
        }
    }
    characters.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");
   
    #[test]
    fn day25_1_result_single_digit_2() {
        assert_eq!(day25_1_result("2").0, 2);
    }

    #[test]
    fn day25_1_result_single_digit_equals() {
        assert_eq!(day25_1_result("=").0, -2);
    }

    #[test]
    fn day25_1_result_single_line() {
        assert_eq!(day25_1_result("12111").0, 906);
    }
    #[test]
    fn day25_1_to_snafu_1_simple() {
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(15), "1=0");
    }

    #[test]
    fn day25_1_to_snafu_2_extra() {
        assert_eq!(to_snafu(12345),"1-0---0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn day25_1_result_1_example() {
        let (human, snafu) = day25_1_result(TEST_INPUT);
        assert_eq!(human, 4890);
        assert_eq!(snafu, "2=-1=0");
    }


    #[test]
    fn day25_1_result_live_test() {
        let (human, snafu) = day25_1_result(INPUT);
        assert_eq!(human, 35023647158862);
        assert_eq!(snafu, "2-10==12-122-=1-1-22");
    }

    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}
