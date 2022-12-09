use std::collections::HashSet;

const NEWLINE: u8 = 10u8;

pub fn day9_1_result(data: &str) -> usize {
    // this would be faster to run, slower to write: data.as_bytes().split(|b| b == NEWLINE).
    //
    let mut tail_visisted: HashSet<(i32,i32)> = HashSet::new();
    tail_visisted.insert((0i32,0i32));

    let result = data.lines()
        .map(|line| line.split_whitespace())
        .map(|mut split| (split.next().unwrap(), split.next().unwrap().parse::<u8>().unwrap()))
        .map(|(direction, size)| {
            match direction {
                "U" => ((1,0), size),
                "D" => ((-1,0), size),
                "L" => ((0,-1), size),
                "R" => ((0,1), size),
                _ => panic!("invalid direction parsed out of input data"),
            }
        })
    .fold(
        (tail_visisted, ((0,0),(0,0))),
        |acc: (HashSet<(i32,i32)>, ((i32, i32),(i32, i32))), movement: ((i32,i32), u8)| {
            // and now for the magic that is the Head and Tail dance
            let (mut tail_visited, ((mut hx, mut hy),(mut tx, mut ty))) = acc;
            let ((dx, dy), steps) = movement;

            let mut last_hx = hx;
            let mut last_hy = hy;

            (0..steps).for_each(|_| {
                
                hx += dx;
                hy += dy;
                if (tx-hx).abs()>1 || (ty-hy).abs()>1 {
                    tx = last_hx;
                    ty = last_hy;
                    tail_visited.insert((tx,ty));
                }
                last_hx = hx;
                last_hy = hy;
            });
            (tail_visited, ((hx,hy), (tx,ty))) 
    });
    

    result.0.len()
}

#[allow(dead_code)]
fn print_positions(data: &Vec<(i32,i32)>) {
    let (nx, px, ny, py) = data.iter().fold((0i32,0i32,0i32,0i32), |acc, (x,y)| {
        (std::cmp::min(acc.0, *x), std::cmp::max(acc.1, *x), std::cmp::min(acc.2, *y), std::cmp::max(acc.3,*y))
    });
    let mut output: Vec<Vec<u8>> = vec![vec![b'.';(px - nx + 1) as usize]; (py - ny + 1) as usize];
   
    data.iter().enumerate().for_each(|(i, (x,y))| {
        if output[(y + ny.abs()) as usize][(x + nx.abs()) as usize] == b'.' {
            output[(y + ny.abs()) as usize][(x + nx.abs()) as usize] = if i == 0 { b'H' } else if i == data.len()-1 { b'T' } else { i as u8 +48 };
        }
    });
    output.iter().rev().for_each(|y| {
        y.iter().for_each(|c| {
            print!("{}", *c as char);
        });
        println!();
    });
    println!("----------")
}


pub fn day9_2_result(data: &str) -> usize {
    let mut tail_visited: HashSet<(i32,i32)> = HashSet::new();
    tail_visited.insert((0i32,0i32));

    let (result, _) = data.lines()
        .map(|line| line.split_whitespace())
        .map(|mut split| (split.next().unwrap(), split.next().unwrap().parse::<u8>().unwrap()))
        .map(|(direction, size)| {
            match direction {
                "U" => ((0,1), size),
                "D" => ((0,-1), size),
                "L" => ((-1,0), size),
                "R" => ((1,0), size),
                _ => panic!("invalid direction parsed out of input data"),
            }
        })
    .fold(
        (tail_visited, vec![(0i32,0i32);10]),
        |acc: (HashSet<(i32, i32)>, Vec<(i32, i32)>), movement: ((i32,i32), u8)| {
       
            let (mut tail_visited, mut positions) = acc;
            let ((dx, dy), steps) = movement;

            (0..steps).for_each(|_step| {
                (0..positions.len()-1).for_each(|i|
                {
                    let next = i + 1;
                    if i == 0 {
                        positions[i].0 += dx;
                        positions[i].1 += dy;
                    }
                    let happened = i;
                    let relative_move = (
                        positions[happened].0 - positions[next].0, 
                        positions[happened].1 - positions[next].1);

                    if (positions[happened].0 - positions[next].0).abs() > 1 || 
                        (positions[happened].1 - positions[next].1).abs() > 1 {

                        match relative_move {
                            (0,2)|(0,-2) => {
                                positions[next].1 += relative_move.1.clamp(-1, 1);
                            }, 
                            (2,0)|(-2,0) => {
                                positions[next].0 += relative_move.0.clamp(-1, 1);
                            }, 
                            (_,2)|(_,-2)|
                            (2,_)|(-2,_) => {
                                positions[next].0 += relative_move.0.clamp(-1, 1);
                                positions[next].1 += relative_move.1.clamp(-1, 1);
                            },
                            (_,_) => (),
                        }
                    }  
                });
                tail_visited.insert(*positions.last().unwrap());
            });
            (tail_visited, positions) 
    });
    
    result.len()

}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day9_1_result_1_test() {
        let test_data = r###"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"###;
        assert_eq!(day9_1_result(test_data), 13);
    }

    #[test]
    fn day9_1_result_live_test() {
        assert_eq!(day9_1_result(INPUT), 6745);
    }

    #[test]
    fn day9_2_result_1_test() {
    let test_data = r###"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"###;

        assert_eq!(day9_2_result(test_data), 1);
    }

    #[test]
    fn day9_2_result_2_test() {
    let test_data = r###"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"###;

        assert_eq!(day9_2_result(test_data), 36);
    }

    #[test]
    fn day9_2_result_live_test() {
        assert_eq!(day9_2_result(INPUT), 2793);
    }
}
