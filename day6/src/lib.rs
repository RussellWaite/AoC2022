pub fn day6_1_result(data: &str) -> usize {
   let unique_counter = 4;
   common_solver(data, unique_counter)
}

pub fn day6_2_result(data: &str) -> usize {
    let unique_count = 14;
    common_solver(data, unique_count)
}

fn common_solver(data:&str, unique_count: usize) -> usize {
    let (_, start_of_packet, _, _) =  data
        .chars()
        .fold((vec![' ';unique_count], 0usize, unique_count as i32 - 3i32, false), 
            |(mut window, index, mut last_dupe_seen, mut found), next_char| {
            
                if found { return (window, index, 0, found); }
                
                if last_dupe_seen == -1 { found = true; }

                window.remove(0);

                if window.contains(&next_char) {
                    last_dupe_seen = std::cmp::max(
                        last_dupe_seen,
                        window
                            .iter()
                            .enumerate()
                            .filter(|(_i,c)| **c == next_char)
                            .map(|(i,_c)| i as i32)
                            .max()
                            .unwrap()
                        );
                    found = false;
                }
                // println!(" STEP: {:?} {} | {}({})|{}", prev_fourteen, next_char, index, dup_count_down, found);
                window.push(next_char);
                
                if last_dupe_seen > -1 { last_dupe_seen-=1; }

                if index < unique_count { return (window, index+1, last_dupe_seen, false); }

                (window, index+1, last_dupe_seen, found)
        }); 
    
    start_of_packet 
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day6_1_result_1_test() {
        assert_eq!(day6_1_result("bvwbjplbgvbhsrlpgdmjqwftvncz"),  5);
    }
    
    #[test]
    fn day6_1_result_2_test() {
        assert_eq!(day6_1_result("nppdvjthqldpwncqszvftbrmjlhg"),  6);
    }
    
    #[test]
    fn day6_1_result_3_test() {
        assert_eq!(day6_1_result("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }
    
    #[test]
    fn day6_1_result_4_test() {
        assert_eq!(day6_1_result("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn day6_1_result_5_test() {
        assert_eq!(day6_1_result(INPUT), 1804);
    }

    #[test]
    fn day6_2_result_1_test() {
        assert_eq!(day6_2_result("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn day6_2_result_2_test() {
        assert_eq!(day6_2_result("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
    
    #[test]
    fn day6_2_result_3_test() {
        assert_eq!(day6_2_result("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn day6_2_result_4_test() {
        assert_eq!(day6_2_result("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),29);
    }

    #[test]
    fn day6_2_result_5_test() {
        assert_eq!(day6_2_result("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn day6_2_result_6_test() {
        assert_eq!(day6_2_result(INPUT), 2508);
    }
}
