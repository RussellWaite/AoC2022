enum Cycle {
    Noop,
    AddxStart,
    AddxEnd(i64)
}

enum Instruction {
    Noop,
    Addx(i64),
}

struct ElftechCrtCPU {
    register: i64,
    instructions: Vec<Cycle>,
    instruction_pointer: usize,
    cycle_states: Vec<i64>,
}

impl ElftechCrtCPU {

    pub fn new() -> Self {
        Self {
            register: 1,
            instructions: vec![],
            instruction_pointer: 0usize,
            cycle_states: vec![1i64],
        }
    }

    fn add_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.instructions.push(Cycle::Noop);
            },
            Instruction::Addx(x) => {
                self.instructions.push(Cycle::AddxStart);
                self.instructions.push(Cycle::AddxEnd(*x));
            },
        }
    }
        
    fn show_raw_display(&mut self, row_size: i64) -> String {
        let bytes: Vec<u8> = self.cycle_states
            .iter()
            .enumerate()
            .map(|(i, x)| {
               if (x-1..=x+1).contains(&((i as i64)%row_size)) { b'#' } else { b'.' }  
            })
            .collect();

        std::str::from_utf8(&bytes).unwrap().to_string()
    }
    
    /// This will maybe run over by 1 if part way through an instruction - i.e. we process
    /// instructions to get cycles.
    fn run_to_cycle(&mut self, cycle: usize) {
        self.register = *self.cycle_states.last().unwrap();

        while self.cycle_states.len() - 1 < cycle {
            

            match self.instructions[self.instruction_pointer] {
                Cycle::Noop | Cycle::AddxStart => (),
                Cycle::AddxEnd(x) => {
                    self.register += x;
                },
            }
            self.cycle_states.push(self.register);
            self.instruction_pointer += 1;
        }
    }

    fn register_x_value_at_cycle(&mut self, cycle_count: usize) -> i64 {
        if self.cycle_states.len() - 1 < cycle_count {
            self.run_to_cycle(cycle_count);
        }
        self.cycle_states[cycle_count-1]
    }
}


fn parse_instructions(data: &str) -> Vec<Instruction> {
    data.lines()
        .map(|line| 
            match line.split_once(' ') {
                None => Instruction::Noop,
                Some((_addx, value)) => Instruction::Addx(value.parse::<i64>().unwrap()),
            }
        ).collect()
}

pub fn day10_1_result(data: &str) -> i64 {
    let instructions = parse_instructions(data);

    // hold signal stength of interest slice
    let signal_strength_of_interest: &[usize] = &[20,60,100,140,180,220];
    
    let mut cpu = ElftechCrtCPU::new();

    instructions.iter()
        .for_each(|instruction| cpu.add_instruction(instruction));

    signal_strength_of_interest.iter()
        .map(|cycle| cpu.register_x_value_at_cycle(*cycle) * *cycle as i64)
        .sum()
}


pub fn day10_2_result(data: &str) -> String {
    const ROW_SIZE:i64 = 40;
    let instructions = parse_instructions(data);
    let mut cpu = ElftechCrtCPU::new();

    instructions.iter()
        .for_each(|instruction| cpu.add_instruction(instruction));

    cpu.register_x_value_at_cycle(239);
    cpu.show_raw_display(ROW_SIZE)

/*
 *
 * (###) is the sprite
 * for each clock cycle, 1, 2, 3, 4, etc. will stamp part of sprtie onto screen if any part of it
 * is over the cycle index
 * so ctr starts a 0 to 39, CRT: ..........
 * register is 1, so sprite is centered at 1, i.e. 0-2.
 * sprite is visible at clock index so # is printed to screen, CRT: #........
 * fist cycle is 1t half of addx so cycle 2 begins with register unchanged so sprite hasn't moved
 * as cycle is now 1, and sprite is 0-2, print #, so CRT: ##.........
 *
 */
    
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day10_1_result_1_test() {
        assert_eq!(day10_1_result(TEST_INPUT), 13140);
    }

    #[test]
    fn day10_1_result_live_test() {
        assert_eq!(day10_1_result(INPUT), 15880);
    }

    #[test]
    fn day10_2_result_1_test() {
        assert_eq!(day10_2_result(TEST_INPUT).as_str(), PART2_TEST_RESULT);
    }

    #[test]
    fn day10_2_result_live_test() {
        assert_eq!(day10_2_result(INPUT), PART2_ACTUAL_RESULT);
    }

const PART2_TEST_RESULT: &str = r###"##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######....."###;

const PART2_ACTUAL_RESULT: &str = r###"###..#.....##..####.#..#..##..####..##..#..#.#....#..#.#....#.#..#..#....#.#..#.#..#.#....#....###..##...#..#...#..#....###..#....#.##.#....#.#..####..#...#.##.#....#....#..#.#....#.#..#..#.#....#..#.#....####..###.#....#..#.#..#.####..###."###;

const TEST_INPUT: &str = r###"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"###;
}
