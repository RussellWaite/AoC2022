use std::collections::VecDeque;

struct SoonToBeDeadMonkey {
    items: VecDeque<u64>,
    inspected_count: u64,
    divisor: u64,
    common_modulous: Option<u64>,
    process_worry: Box<dyn Fn(u64, Option<u64>) -> u64>,
    throw_to: Box<dyn Fn(u64) -> usize>,
}

impl SoonToBeDeadMonkey {
    fn process_items(&mut self) -> Option<(usize, u64)> {
        match self.items.pop_front() {
            Some(item) => {
                self.inspected_count += 1;
                let worry_item = (self.process_worry)(item, self.common_modulous);
                
                Some(((self.throw_to)(worry_item), worry_item))
            },
            None => None
        }
    }

    fn recieve_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

struct KillList {
    list: Vec<SoonToBeDeadMonkey>,
}

impl KillList {
    fn process_monkeys(&mut self) {
        let local_list = &mut self.list;
        for i in 0..local_list.len() {
            while let Some((index, item)) = 
                local_list[i].process_items(){
                local_list[index].recieve_item(item);
            }
        }
    }
    #[allow(dead_code)]
    fn monkey_state(&self) {
        let local_list = &self.list;
        for i in 0..local_list.len() {
            println!("Monkey: {}, inspected: {} and has {:?}", i, local_list[i].inspected_count, local_list[i].items);
        }
    }
}

fn parse_kill_list(data: &str) -> Vec<SoonToBeDeadMonkey> {
    data.split("Monkey ").skip(1)
        .map(|deets| {
            let mut data_iter = deets.lines().skip(1);
            let items:VecDeque<u64> = data_iter.next()
                .unwrap()
                .trim()
                .replace("Starting items: ", "")
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            let operation = data_iter.next()
                .unwrap()
                .trim()
                .replace("Operation: new = old ","")
                ;
                // THIS IS WEIRD....
            let operation: (&str, &str) = operation
                .split_once(" ")
                .unwrap();
            
            let divisor = data_iter.next()
                .unwrap()
                .trim()
                .replace("Test: divisible by ", "")
                .parse::<u64>()
                .unwrap();

            let true_monkey = data_iter.next()
                .unwrap()
                .trim()
                .replace("If true: throw to monkey ", "")
                .parse::<usize>()
                .unwrap();

            let false_monkey = data_iter.next()
                .unwrap()
                .trim()
                .replace("If false: throw to monkey ", "")
                .parse::<usize>()
                .unwrap();

            SoonToBeDeadMonkey { 
                items, 
                inspected_count: 0, 
                divisor,
                common_modulous: None,
                process_worry: match operation {
                   ("*", "old") => {
                        Box::new(worry_squared)
                    },
                    ("*", value) => {
                        let factor = value.parse::<u64>().unwrap();
                        Box::new(move |worry, cmt| worry_multiplied(worry, factor, cmt))
                    },
                    ("+", value) => {
                        let factor = value.parse::<u64>().unwrap();
                        Box::new(move |worry, cmt| worry_added(worry, factor, cmt))
                    },
                    _ => panic!("Couldn't read operation for the monkey..."),
                }, 
                throw_to: Box::new(move |worry| monkey_action(worry, divisor, true_monkey, false_monkey)), 
            }
        }).collect::<Vec<SoonToBeDeadMonkey>>()

}
fn adjust_for_part_2(monkeys: &mut Vec<SoonToBeDeadMonkey>) {
    let chinese_remainder_theorem = monkeys.iter().map(|m| m.divisor).product();
    
    for i in 0..monkeys.len() {
        monkeys[i].common_modulous = Some(chinese_remainder_theorem);
    }
}

fn monkey_action(worry: u64, post_inspection_factor: u64, true_recipient: usize, false_recipient: usize) -> usize {
    if worry % post_inspection_factor == 0 {
        true_recipient
    }
    else {
        false_recipient
    }
}

fn worry_squared(worry: u64, pif: Option<u64>) -> u64 {
    match pif {
        Some(modulous) => (worry * worry)%modulous,
        None => calm_down(worry * worry)
    }
}
fn worry_multiplied(worry: u64, factor: u64, pif: Option<u64>) -> u64 {
    match pif {
        Some(modulous) => (worry * factor) % modulous,
        None => calm_down(worry * factor)
    }
}
fn worry_added(worry: u64, factor: u64, pif: Option<u64>) -> u64 {
    match pif {
        Some(modulous) => (worry + factor)%modulous,
        None => calm_down(worry + factor)
    }
}

#[inline]
fn calm_down(worry: u64) -> u64 {
    worry / 3
}

pub fn day11_1_result(data: &str) -> u64 {

    let mut kill_list = KillList{ list: parse_kill_list(data) };

    for _round in 0..20 {
        // kill_list.monkey_state();
        kill_list.process_monkeys();
    }

    let mut temp = kill_list.list.iter()
        .map(|monkey| monkey.inspected_count)
        .collect::<Vec<u64>>();
    temp.sort();
    temp.reverse();
    match temp.windows(2).next().unwrap() {
        [a,b] => a*b,
        _ => panic!("it's all gone wrong, right at the end too - oh noes") 
    }
}

pub fn day11_2_result(data: &str) -> u64 {
    let mut kill_list = KillList{ list: parse_kill_list(data) };
    adjust_for_part_2(&mut kill_list.list);
    
    for _round in 0..10000 {
        kill_list.process_monkeys();
    }

    let mut temp = kill_list.list.iter()
        .map(|monkey| monkey.inspected_count)
        .collect::<Vec<u64>>();
    temp.sort();
    temp.reverse();
    
    match temp.windows(2).next().unwrap() {
        [a,b] => a*b,
        _ => panic!("it's all gone wrong, right at the end too - oh noes") 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day11_1_result_1_test() {
        assert_eq!(day11_1_result(TEST_INPUT), 10605);
    }

    #[test]
    fn day11_1_result_live_test() {
        assert_eq!(day11_1_result(INPUT), 58794);
    }

    #[test]
    fn day11_2_result_1_test() {
        assert_eq!(day11_2_result(TEST_INPUT), 2713310158);
    }

    #[test]
    fn day11_2_result_live_test() {
        assert_eq!(day11_2_result(INPUT), 20151213744);
    }

const TEST_INPUT: &str = r###"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"###;
}
