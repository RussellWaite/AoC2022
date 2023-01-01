/// IDEA GRAVEYARD
/// 1 - just mapping the entire cave is wasteful and we could maybe do something like
///     map the height of each position - so 7 integers rather than thousands of positions
///     ==> would fail when one slipped down a side and then moved under the column's
///     top most value
///
/// 2 - store column as a byte and bit shift the comparisons
///     Each object could also be mapped as aseries of bytes... THIS MIGHT WORK
pub fn day17_1_result(data: &'static [u8]) -> usize {
    let cave = solve(data, Box::new(|cave| cave.rock_count == 2022));
    cave.highest_rock
}

/// IDEA GRAVEYARD PT 2
/// 1 - just run simulate - REAL rough esitmates put it at 2ms for the part 1 so ignoring setup
///     code, 2ms for 2000 put it about 22 days for 1000000000000. we'll leae that as a backup
///     option.
/// 2 - some mathematical formula to just calculate it - it's probably going to take me more than
///     22 days to work out that formula
/// 3 - the numers form a repeating pattern and we can use that to get close
///     My worry is we start with a 7 wide flat surface, but we could mimic that with a
///     horizontal and tops of a square vertical... so it's possible.
///     We might need to find the repeat and add the start to it
///     i.e. cave floor to start of repeating pattern +
///          (repeating pattern * factor) +
///          last few to hit magic height
///     now... how to find a repeating pattern as I don't want to become a specialist in elephant
///     rock math
pub fn day17_2_result(data: &'static [u8]) -> usize {
    //solve(data, 1000000000000) HAHA - not a chance
    let elephants_choice: usize = 1000000000000;
    // I could just print it out and look, but it's not advent of me noticing patterns of characters
    //
    // what is the magic number for finding a repetition..?
    // could I cheese it and use a pre built compression algorithm to help me detect the right
    // magic number?
    let cave = solve(data, Box::new(|cave| cave.rock_count >= 6000)); 

    let mut largest_repetition: Repetition = Repetition { start: 0, len: 0 };
    let mut i: usize = 0;
    let mut j: usize = 1;

    #[allow(unused_labels)]
    'i: loop {
        if i >= cave.space.len() {
            break;
        }
        let mut i_increment = 1;
        'j: loop {
            if j >= cave.space.len() {
                break 'j;
            }

            if cave.space[i] != cave.space[j] {
                j += 1;
                continue;
            }
            // we have at least a 1 char repeat...
            let mut repeat = Repetition { start: i, len: 1 };

            'repeat: loop {
                if j+repeat.len >= cave.space.len() || // out of bounds
                    cave.space[i + repeat.len] != cave.space[j + repeat.len] || // no equality
                    i + repeat.len >= j
                {
                    // overlapped - so good we check for it twice
                    break 'repeat;
                }
                repeat.len += 1;
            }
            if largest_repetition.len < repeat.len && 
                (cave.space.len() < j + 2 * repeat.len || cave.space[i] == cave.space[j + 2 * repeat.len]) {
                largest_repetition = repeat;
            }
            // have we found our sequential pattern
            if repeat.len > 1 && i + repeat.len == j {
                break 'j; // <- THIS... I typoed this as i and it stumped me for AGES as it didnt
                          // affect the sample input, just the real one - pfft
            }

            i_increment = std::cmp::max(i_increment, repeat.len);
            j += repeat.len;
            // j += 1;
        }
        i += i_increment;
        j = i + 1;
    }
    // println!("{largest_repetition:?}");

    let start_rocks = solve(data, 
        Box::new(|c| c.highest_rock > largest_repetition.start))
        .rock_count as usize - 1;
    
    let start_plus_one_repeat_rocks = solve(data, 
        Box::new(|c| c.highest_rock > largest_repetition.start + largest_repetition.len))
        .rock_count as usize - 1;
    
    let rocks_per_repeat = start_plus_one_repeat_rocks - start_rocks;
    let end_rocks = (elephants_choice - start_rocks) % rocks_per_repeat;
    let end_height = solve(data, 
        Box::new(|c| c.rock_count as usize > start_rocks + end_rocks))
        .highest_rock - largest_repetition.start - 1;
    
    let repeat_count = (elephants_choice - start_rocks - end_rocks) / rocks_per_repeat;
    // println!("Start height: {}, Start Rocks: {start_rocks}, Start plus cycle: {start_plus_one_repeat_rocks}, repeat height: {}, repeat rocks: {rocks_per_repeat}, end rocks: {end_rocks}, end height: {end_height}, repeat count: {repeat_count}", largest_repetition.start, largest_repetition.len);
    // println!("({repeat_count} * {}) + {} + {end_height}", largest_repetition.len, largest_repetition.start);
    (repeat_count * largest_repetition.len) + largest_repetition.start + end_height
}

/*  i
 *  0 8 7 4 9 6 7 2 7 4 9 7 4 9 6 7 2 7 4 9 7
 *    n n n n n
 *    j
 *-------------------------------------------
 *    i
 *  0 8 7 4 9 6 7 2 7 4 9 7 4 9 6 7 2 7 4 9 7
 *      n n n n
 *      j
 * ------------------------------------------
 *      i . . . . . . . . !
 *  0 8 7 4 9 6 7 2 7 4 9 7 4 9 6 7 2 7 4 9 7
 *        n n n Y n Y Y Y n
 *                        Y Y Y Y Y Y Y Y Y !
 *        j j j 1 j - - 3 j - - - - - - - 9 j
 *-------------------------------------------
 * we found 3 repeating patterns, 1 (pfft), 3 and
 * then 9. We did have to stop though as i's index
 * overlapped j's start point!
 *
 */

/* slightly more complicated...
 *
 * ----------------------------------------------
 *      i . . . . . . . . !
 *  0 8 7 6 5 4 3 2 1 7 6 7 6 5 4 3 2 1 7 9 5 7 0
 *        n n n n n Y Y Y n
 *                        Y Y Y Y Y Y Y Y Y !
 *        j j j j j j 1 j j - - - - - - - 9 j
 *-------------------------------------------
 * 2 repeats, 1 (pfft) and 8.
 * can we increment i by repeat length?
 */
#[derive(Clone, Copy, Debug, PartialEq)]
struct Repetition {
    start: usize,
    len: usize,
}

fn solve<'a>(data: &'static [u8], predicate: Box<dyn Fn(&Cave)->bool + 'a>) -> Cave {
    let mut rock_fall = rock_iterator();
    let mut gusts = gusts_iterator(data);
    let mut cave = Cave::new(Box::new(move || gusts.next()));

    loop {
        let death_from_above = rock_fall.next().unwrap();
        cave.add_rock(death_from_above);
        if (predicate)(&cave) {
            break;
        }
    }
    cave
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum RockShape {
    HoriontalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

fn rock_iterator() -> impl Iterator<Item = RockShape> {
    let order: &[RockShape; 5] = &[
        RockShape::HoriontalLine,
        RockShape::Plus,
        RockShape::ReverseL,
        RockShape::VerticalLine,
        RockShape::Square,
    ];
    order.iter().cycle().copied()
}

#[derive(Debug, Clone)]
struct Rock {
    height: usize,
    top: usize,
    bitmasks: Box<Vec<Vec<u8>>>,
}

impl Rock {
    pub fn new(shape: RockShape, spawn_height: usize) -> Self {
        let (height, bitmasks): (usize, Vec<Vec<u8>>) = match shape {
            RockShape::HoriontalLine => (1, Self::rock_horizontal_positions()),
            RockShape::Plus => (3, Self::rock_plus_positions()),
            RockShape::ReverseL => (3, Self::rock_reversel_positions()),
            RockShape::VerticalLine => (4, Self::rock_vertical_positions()),
            RockShape::Square => (2, Self::rock_square_positions()),
        };
        Self {
            height,
            top: spawn_height + height,
            bitmasks: Box::new(bitmasks),
        }
    }

    fn rock_horizontal_positions() -> Vec<Vec<u8>> {
        vec![
            vec![0b1111000],
            vec![0b0111100],
            vec![0b0011110],
            vec![0b0001111],
        ]
    }

    fn rock_plus_positions() -> Vec<Vec<u8>> {
        vec![
            vec![0b0100000, 0b1110000, 0b0100000],
            vec![0b0010000, 0b0111000, 0b0010000],
            vec![0b0001000, 0b0011100, 0b0001000],
            vec![0b0000100, 0b0001110, 0b0000100],
            vec![0b0000010, 0b0000111, 0b0000010],
        ]
    }
    fn rock_reversel_positions() -> Vec<Vec<u8>> {
        vec![
            vec![0b0010000, 0b0010000, 0b1110000],
            vec![0b0001000, 0b0001000, 0b0111000],
            vec![0b0000100, 0b0000100, 0b0011100],
            vec![0b0000010, 0b0000010, 0b0001110],
            vec![0b0000001, 0b0000001, 0b0000111],
        ]
    }
    fn rock_vertical_positions() -> Vec<Vec<u8>> {
        vec![
            vec![0b1000000, 0b1000000, 0b1000000, 0b1000000],
            vec![0b0100000, 0b0100000, 0b0100000, 0b0100000],
            vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
            vec![0b0001000, 0b0001000, 0b0001000, 0b0001000],
            vec![0b0000100, 0b0000100, 0b0000100, 0b0000100],
            vec![0b0000010, 0b0000010, 0b0000010, 0b0000010],
            vec![0b0000001, 0b0000001, 0b0000001, 0b0000001],
        ]
    }
    fn rock_square_positions() -> Vec<Vec<u8>> {
        vec![
            vec![0b1100000, 0b1100000],
            vec![0b0110000, 0b0110000],
            vec![0b0011000, 0b0011000],
            vec![0b0001100, 0b0001100],
            vec![0b0000110, 0b0000110],
            vec![0b0000011, 0b0000011],
        ]
    }
}

/// Maybe modeling cave is better than rocks..
struct Cave {
    space: Vec<u8>,
    rock_count: u64,
    highest_rock: usize,
    gusts: Box<dyn FnMut() -> Option<Direction>>,
}

impl Cave {
    pub fn new(gusts: Box<dyn FnMut() -> Option<Direction>>) -> Self {
        let mut space: Vec<u8> = Vec::with_capacity(2048);
        // b01111111 (could have used that but keep in bit-shifting mindset)
        space.push(((1u64 << 7) - 1u64) as u8); // add cave floor
        space.push(0); // |
        space.push(0); // |- add 3 spaces above it
        space.push(0); // |
        Self {
            space,
            rock_count: 0,
            highest_rock: 0,
            gusts: Box::new(gusts),
        }
    }

    #[allow(dead_code)]
    fn print_cave(&self) {
        self.space
            .iter()
            .rev()
            .for_each(|level| println!("|{level:07b}|"));
        println!();
    }
    #[allow(dead_code)]
    fn print_cave_0_top(&self) {
        self.space
            .iter()
            .for_each(|level| println!("|{level:07b}|"));
        println!();
    }
    #[allow(dead_code)]
    fn print_cave_0_top_raw(&self) {
        self.space.iter().for_each(|level| println!("{level}"));
        println!();
    }
    #[allow(dead_code)]
    fn print_repetition(&self, start: usize, end: usize) {
        self.space.iter().enumerate().for_each(|(i, level)| {
            if (start..=end).contains(&i) {
                println!("{level}")
            }
        });
        println!();
    }
    pub fn add_rock(&mut self, rock_shape: RockShape) {
        self.rock_count += 1;
        let mut rock: Rock = Rock::new(rock_shape, self.highest_rock + 3);
        const GRAVITY: usize = 1;
        let mut rock_indexer = 2usize;

        while rock.top >= self.space.len() {
            self.space.push(0b0);
        }

        loop {
            let Some(gust) = (self.gusts)() else {
                panic!("You done broke the gust iterator, none returned for a supposed infinite iterator"); 
            };

            // 1 - MOVE LEFT/RIGHT
            let mut can_move = true;
            if !(gust == Direction::Left && rock_indexer == 0
                || gust == Direction::Right && rock_indexer == rock.bitmasks.len() - 1)
            {
                let potential = match gust {
                    Direction::Left => rock_indexer - 1,
                    Direction::Right => rock_indexer + 1,
                };

                for y_adjust in 0..rock.height {
                    if rock.bitmasks[potential][y_adjust] & self.space[rock.top - y_adjust] > 0 {
                        can_move = false;
                        break;
                    }
                }

                if can_move {
                    rock_indexer = potential;
                }
            }

            // 2 - DROP DOWN THANKS TO GRAVITY
            let mut can_drop = true;

            for y_adjust in 0..rock.height {
                if rock.bitmasks[rock_indexer][y_adjust] & self.space[rock.top - y_adjust - GRAVITY]
                    > 0
                {
                    can_drop = false;
                    break;
                }
            }

            if can_drop {
                rock.top -= GRAVITY;
            } else {
                // add rock to cave
                for y_adjust in 0..rock.height {
                    self.space[rock.top - y_adjust] |= rock.bitmasks[rock_indexer][y_adjust]
                }
                break;
            }
        }
        self.highest_rock = std::cmp::max(self.highest_rock, rock.top);
    }
}

fn gusts_iterator(data: &[u8]) -> impl Iterator<Item = Direction> + '_ {
    data.iter()
        .filter(|b| **b == 60 || **b == 62)
        .map(|b| match b {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            x => panic!("somehow read in a character other than < or > ...'{x:?}'..."),
        })
        .cycle()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = include_bytes!("../input");

    #[test]
    fn day17_1_result_1_test() {
        assert_eq!(day17_1_result(TEST_INPUT), 3068);
    }

    #[test]
    fn day17_1_result_live_test() {
        assert_eq!(day17_1_result(INPUT), 3065);
    }

    #[test]
    fn day17_2_result_1_test() {
        assert_eq!(day17_2_result(TEST_INPUT), 1_514_285_714_288);
    }

    #[test] 
    fn day17_2_result_live_test() {
        assert_eq!(day17_2_result(INPUT), 1_562_536_022_966);
        // 1_999_999_999_959 too high aswell
        // 2_499_999_999_917 too high
        // 2_666_666_666_653 too high
    }
    const TEST_INPUT: &[u8] = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".as_bytes();
}
