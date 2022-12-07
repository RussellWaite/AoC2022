#[derive(Clone, Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
    files: Vec<u64>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            name: "/".to_string(),
            children: vec!(),
            files: vec![],
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn parse<'a>(node: &'a mut Node, data: &'a [&'a str], mut index: usize) -> (&'a Node, usize) {
        'recurse: loop {
            if index >= data.len() { break 'recurse };
            match data[index].split_whitespace().collect::<Vec<&str>>()[..] {
                ["$","cd", ".."] => {
                    index +=1;
                    break 'recurse;
                },
                ["$","cd", name] => {
                    let mut sub_dir = Node::new();
                    sub_dir.name = name.to_string();
                    let (sub_dir, jump_index) = Self::parse(&mut sub_dir, data, index + 1);
                    node.children.push(sub_dir.clone());
                    index = jump_index -1;
                },
                ["$", "ls"] => (),
                ["dir", _name] => (),
                [size, _name] => {
                    node.files.push(size.parse::<u64>().unwrap());
                },
                _ => (),
            }
            index+=1;
        }
        (node, index)
    }

    fn directory_sizes(&self, state: &mut Vec<(String, u64)>) -> (String, u64) {//, &mut Vec<(String, u64)>) {
        let mut running_total = 0u64;
        if !self.is_leaf() { // get childrens' size, then your own 
            running_total += self
                .children
                .iter()
                .map(|node| node.directory_sizes(state).1)
                .sum::<u64>();
        }
        
        running_total += self.files.iter().sum::<u64>();
        state.push((self.name.clone(), running_total));

        (self.name.clone(), running_total)
    }
}

pub fn day7_1_result(data: &str) -> u64 {
    let lines = data.lines().collect::<Vec<&str>>();
    let mut root = Node::new();
    let temp = Node::parse(&mut root, &lines, 0).0;
    let mut results: Vec<(String, u64)> = vec![];
    let (_,_) = temp.directory_sizes(&mut results);
    results.iter().filter(|(_, size)| *size <=100_000).map(|(_, size)| *size).sum()
}

pub fn day7_2_result(data: &str) -> u64 {
    let total_space =    70_000_000;
    let free_space_req = 30_000_000; // 6090134

    let lines = data.lines().collect::<Vec<&str>>();
    let mut root = Node::new();
    let temp = Node::parse(&mut root, &lines, 0).0;
    let mut results: Vec<(String, u64)> = vec![];
    let (_,_) = temp.directory_sizes(&mut results);
    results.sort_by(|a, b| a.1.cmp(&b.1));
    // dbg!(&results);
    
    let space_taken = results.iter()
    .map(|(_, size)| *size)
    .max()
    .unwrap();

    let space_needed = free_space_req - (total_space - space_taken);

    results.iter()
    .filter(|(_, size)| *size >= space_needed)
    .map(|(_, size)| *size)
    .min()
    .unwrap()

}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day7_1_result_1_test() {
        let test_data = r###"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"###;
        assert_eq!(day7_1_result(test_data), 95437);
    }

    #[test]
    fn day7_1_result_live_test() {
        assert_eq!(day7_1_result(INPUT), 1491614);
    }

    #[test]
    fn day7_2_result_1_test() {
        let test_data = r###"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"###;

        assert_eq!(day7_2_result(test_data), 24933642);
    }

    #[test]
    fn day7_2_result_live_test() {
        assert_eq!(day7_2_result(INPUT), 6400111);
    }
}

