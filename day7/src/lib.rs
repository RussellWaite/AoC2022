use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    children: Vec<Node>,
    files: Vec<u64>,
}

pub enum FsLine<'a> {
    Pushd(&'a str),
    Popd,
    Dir(&'a str),
    File(u64),
    Ls,
    Error,
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
    
    fn parse_line(line: &[u8]) -> FsLine {
        match line {
            [b'$', b' ', b'c', b'd', b' ', rest @..] => {
                match rest {
                    b".." => FsLine::Popd,
                    name => FsLine::Pushd(std::str::from_utf8(&name).unwrap()),
                }
            },
            [b'$', b' ', b'l', b's'] => FsLine::Ls,
            [b'd',b'i', b'r', b' ', rest @ ..] => {
                let mut split_index = 0usize;
                for idx in 0..rest.len() {
                    if rest[idx] == b' ' {
                        split_index = idx;
                        break;
                    }
                }
            
                return FsLine::Dir(std::str::from_utf8(rest.split_at(split_index).1).unwrap());
                // let mut split = line.rsplit(|c| *c == b' ');
                // let char_array = split.next().unwrap();
                // return FsLine::Dir(std::str::from_utf8(char_array).unwrap());
            },
            _ => {
                let mut split_index = 0usize;
                for idx in 0..line.len() {
                    if line[idx] == b' ' {
                        split_index = idx;
                        break;
                    }
                }
                
                let number = std::str::from_utf8(line.split_at(split_index).0).unwrap();
                FsLine::File(number.parse().unwrap())
                // let mut split = line.split(|c| *c == b' ');
                // let char_array = split.next().unwrap();
                // let number = std::str::from_utf8(char_array).unwrap();
                // FsLine::File(number.parse().unwrap())
            }
        }
    }

    fn parse<'a>(node: &'a mut Node, data: &'a [&'a str], mut index: usize) -> (&'a Node, usize) {
        'recurse: loop {
            if index >= data.len() { break 'recurse };
            match Node::parse_line(data[index].as_bytes()) {
            // match data[index].as_bytes() {
                FsLine::Ls => (),
                FsLine::Pushd(name) => {
                    if name != "/" {
                        let mut sub_dir = Node::new();
                        sub_dir.name = name.to_string();
                        let (sub_dir, jump_index) = Self::parse(&mut sub_dir, data, index + 1);
                        node.children.push(sub_dir.clone());
                        index = jump_index -1;
                    }
                },
                FsLine::Popd => {
                    index += 1; 
                    break 'recurse;
                },
                FsLine::Dir(_) => (),
                FsLine::File(size) => node.files.push(size),
                FsLine::Error => panic!("How have you get here, you've probably messed up the test input"),
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
pub fn day7_parse(data: &Vec<&str>) -> Node {

    // let start = Instant::now();

    let mut root = Node::new();
    let result = Node::parse(&mut root, &data, 0).0;

    // let duration = Instant::now() - start;
    // println!("PARSING TAKES {} μs", duration.as_micros());
    
    result.to_owned() 
}
pub fn day7_1_result(fs: &Node) -> u64 {

    let mut results: Vec<(String, u64)> = vec![];
    let (_,_) = fs.directory_sizes(&mut results);
    results.iter().filter(|(_, size)| *size <=100_000).map(|(_, size)| *size).sum()
}

pub fn day7_2_result(fs: &Node) -> u64 {
    let total_space =    70_000_000;
    let free_space_req = 30_000_000; // 6090134

    let mut results: Vec<(String, u64)> = vec![];
    fs.directory_sizes(&mut results);
    
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

const INPUT: &str = include_str!("../input");
pub fn day7_all() {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let fs = day7_parse(&lines);
    day7_1_result(&fs);
    day7_2_result(&fs);
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
        let lines = test_data.lines().collect::<Vec<&str>>();
        let fs = day7_parse(&lines);

        assert_eq!(day7_1_result(&fs), 95437);
    }

    #[test]
    fn day7_1_result_live_test() {
        let lines = INPUT.lines().collect::<Vec<&str>>();
        let fs = day7_parse(&lines);
        assert_eq!(day7_1_result(&fs), 1491614);
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
        let lines = test_data.lines().collect::<Vec<&str>>();
        let fs = day7_parse(&lines);

        assert_eq!(day7_2_result(&fs), 24933642);
    }

    #[test]
    fn day7_2_result_live_test() {
        let lines = INPUT.lines().collect::<Vec<&str>>();
        let fs = day7_parse(&lines);
        assert_eq!(day7_2_result(&fs), 6400111);
    }
}

