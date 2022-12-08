fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {

    let col_count:usize = data.find('\n').unwrap();
    let row_count:usize = data.len() / col_count;
    let data = data.replace("\n", "");
    let map = data.as_str().as_bytes();
    // print_data(data.as_str(), col_count);
    let mut rows: Vec<Vec<u8>> = vec![vec![]; row_count];
    let mut cols: Vec<Vec<u8>> = vec![vec![]; col_count];

    for (idx, tree) in map.iter().enumerate() {
        rows[idx/col_count].push(*tree);
        cols[idx%col_count].push(*tree);
    }

    // print_rows(&rows, col_count);
    // print_cols(&cols, col_count);
    (rows,cols)
}

fn scenic_scores(data: &Vec<u8>, tree_index: usize) -> (u64,u64) {
        let forwards = scenic_score(data, tree_index);
        let mut rev = data.clone();
        rev.reverse(); // i'm passed caring about allocations now
        let backwards = scenic_score(&rev, data.len() -1  - tree_index);
        (forwards, backwards)
}

fn scenic_score(data: &Vec<u8>, tree_index: usize) -> u64 {
    if tree_index == data.len()-1 { return 0; }

    let tree_height = data[tree_index];
    let mut clear_path = 0u64;
    for i in (tree_index+1)..data.len() {
        clear_path += 1;
        if data[i] >= tree_height {
            break;
        }
    }
    clear_path
}


fn visible_from_edges(data: &Vec<u8>, tree_index: usize) -> (u8,u8) {
    let forwards = visible_from_edge(data, tree_index);
    let mut rev = data.clone();
    rev.reverse(); // i'm passed caring about allocations now
    let backwards = visible_from_edge(&rev, data.len() -1  - tree_index);
    // println!("  tree: {} fwds: {}, back:{}", tree_index, forwards, backwards);
    (forwards, backwards)
}

fn visible_from_edge(data: &Vec<u8>, tree_index: usize) -> u8 {
    let tree_height = data[tree_index];
    let mut clear_path = 1u8;
    for (i, tree) in data.iter().enumerate() {
        if i >= tree_index { break; }
        if *tree >= tree_height {
            clear_path = 0u8;
            break;
        }
    }

    clear_path
}

#[allow(dead_code)] 
fn print_data(data: &str, enries_per_row: usize) {
println!("RAW DATA --------------");
    data.chars().enumerate().for_each(|(i, c)| {
        if i%enries_per_row == 0 { 
            println!();
        }
        print!("{}", c);
        });
    
            println!();
}

#[allow(dead_code)] 
fn print_counter<T>(data: &Vec<T>, enries_per_row: usize) where T: std::fmt::Display {
println!("COUNTER --------------");
    data.iter().enumerate().for_each(|(i, c)| {
        if i%enries_per_row == 0 { 
            println!();
        }
        print!("{},", c);
        });
    
            println!();
}

#[allow(dead_code)] 
fn print_rows(data: &Vec<Vec<u8>>, entries_per_row: usize) {
println!("ROWS----------");
    data.iter().enumerate().for_each(|(i, x)|{
       x.iter().for_each(|c| print!("{}",c - 48u8)); 
        println!();
    });
    println!();
}

#[allow(dead_code)] 
fn print_cols(data: &Vec<Vec<u8>>, entries_per_row: usize) {
println!("COLS-----------");
    for i in 0..data[0].len() {
        for j in 0..data.len() {
            print!("{}",  data[j][i]-48u8);
        }
        println!();
    }
    println!();
}

pub fn day8_1_result(data: &str) -> usize {

    let (rows, cols) = parse(data);
    
    let data = data.replace('\n', "");
    
    let mut counter: Vec<u8> = vec![0u8;data.len()];
    let col_count = cols.len();

    for index in 0..counter.len() {
        let (down,up) =  visible_from_edges(&rows[(index/col_count) as usize], (index%col_count) as usize) ;
        let (right, left) =  visible_from_edges(&cols[(index%col_count) as usize], (index/col_count) as usize);
        counter[index] = (down + up + left + right).clamp(0, 1);
    }
    // print_counter(&counter, col_count);
    counter.iter().filter(|x| **x > 0u8).count()
}

pub fn day8_2_result(data: &str) -> u64 {
    let (rows, cols) = parse(data);
    
    let data = data.replace("\n", "");
    // println!("DATA LENGTH = {}", data.len());
    
    let mut counter: Vec<u64> = vec![0u64;data.len()];
    let col_count = cols.len();

    for index in 0..counter.len() {
        let (down,up) =  scenic_scores(&rows[(index/col_count) as usize], (index%col_count) as usize) ;
        let (right, left) =  scenic_scores(&cols[(index%col_count) as usize], (index/col_count) as usize);
        counter[index] = down * up * left * right;
    }
    // print_counter(&counter, col_count);
    *counter.iter().max().unwrap() as u64

}

/*
 * WHAT I SHOULD HAVE DONE:
 *
 * find pos of newline
 * stri newlines
 * convert to slice of bytes
 * no more allocation of maps, etc (pt2 maybe has 1 new DS).
 *
 * have functions that work on slice
 * pt 1
 * check if 1 is returned, if so short circuit all other checks.
 *
 * pt 2 
 *
 * maybe build a slice of (tree size, (left, right, up, down))
 * we can then use those values as we scan,
 *
 * e.g. (3, (0,0,0,0)), (4, (1, 1, 0, 0)), (5, (2,1,0,0)), (6, {smart calc}
 *  use prev cell to work out how far you can skip, i.e. 6 > 5, so 5's counts are inclusive
 *  next height check... i.e. minimise work where possible
 *
 */

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day8_1_result_1_test() {
        let test_data = r###"30373
25512
65332
33549
35390"###;
        assert_eq!(day8_1_result(test_data), 21);
    }

    #[test]
    fn day8_1_result_live_test() {
        assert_eq!(day8_1_result(INPUT), 1870);
    }

    #[test]
    fn day8_2_result_1_test() {
        let test_data = r###"30373
25512
65332
33549
35390"###;

        assert_eq!(day8_2_result(test_data), 8);
    }

    #[test]
    fn day8_2_result_live_test() {
        assert_eq!(day8_2_result(INPUT), 517440);
    }
}
