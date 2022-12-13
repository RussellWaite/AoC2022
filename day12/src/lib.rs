// struct TerrainMap {
  // plan is to mae this a singleton backed factory to prevent having to parse data twice...
// }

struct CostMap {
    map: Vec<u64>,
    start: usize,
    end: usize,
    width: usize,
}

impl CostMap {
    pub fn new(size: usize) -> Self {
        Self { map: vec![u64::MAX; size], start: 0, end: 0, width: usize::MAX }
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn calculate_costs(&mut self, terrain_map: &[u8], map_width: usize) {
        self.width = map_width;

        self.map[self.end] = 0;

        let mut journeys: Vec<usize> = vec![self.end];
        let mut min_height = b'z';
        while let Some(position) = journeys.pop() {
            if terrain_map[position] < min_height {
                min_height = terrain_map[position];
            }
            let positions = Self::get_valid_moves(position, terrain_map, map_width);
            let cost = self.map[position] + 1;
            positions.iter().for_each(|position| {
                if self.map[*position] > cost {
                    // println!("Visiting {}, was {} now lower: {}", *position, self.map[*position], cost);
                    self.map[*position] = cost;
                    journeys.push(*position);
                }
            });
        }
    }

    pub fn cheapest_part1(&self) -> u64 {
        self.map[self.start]
    }
    
    pub fn cheapest_part2(&self, terrain_map: &[u8]) -> u64 {
        let all_a_positions: Vec<usize> = terrain_map.iter()
            .enumerate()
            .filter(|(_, b)| **b == b'a')
            .map(|(i, _)| i)
            .collect();
            
        let result = all_a_positions.iter()
            .map(|a| self.map[*a])
            .min();

        result.unwrap()
    } 
    
    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    #[inline]
    fn get_valid_moves(from: usize, map: &[u8], map_width: usize) -> Vec<usize> {
        let normalise = |x| -> u8 {
            match x {
                b'S' => b'a',
                b'E' => b'z',
                x => x,
            }
        };
        let current_elevation = normalise(map[from]); 

        let mut results: Vec<usize> = vec![];
        //up
        if from >= map_width {
            // add up coords
            let potential = from - map_width;
            let map_potential = normalise(map[potential]);
            if map_potential > current_elevation ||
                current_elevation <= map_potential + 1 {
                    results.push(potential);
                }
        } 
        //down
        if from < (map.len() - map_width) {
            // TODO: repeated code - hmmm - might be better to pull logic out and use option"
            let potential = from + map_width;
            let map_potential = normalise(map[potential]);
            if map_potential > current_elevation ||  
                current_elevation <= map_potential + 1 {
                    results.push(potential);
                }
        }
        let x_pos = from%map_width;
        //left
        if x_pos > 0 {
            let potential = from - 1;
            let map_potential = normalise(map[potential]);
            if map_potential > current_elevation ||
                current_elevation <= map_potential + 1 {
                    results.push(potential);
                }
        }
        //right
        if x_pos != map_width-1 {
            let potential = from + 1;
            let map_potential = normalise(map[potential]);
            if map_potential > current_elevation || 
                current_elevation <= map_potential + 1 {
                    results.push(potential);
                }
        }
        results
    }
}

impl std::fmt::Display for CostMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let visits = self.map.iter()
            .map(|x| if *x == u64::MAX {b'X'} else {b'.'})
            .collect::<Vec<u8>>();

        let result: String = visits.chunks(self.width)
            .map(|chunk| format!("{}\n",std::str::from_utf8(chunk).unwrap()))
            .collect();

        write!(f, "{}", result)
    }
}

fn clean_map_data(data: &[u8]) -> (Vec<u8>, usize) {
    let map_width = data.splitn(2, |x| *x == b'\n')
        .next()
        .expect("data has newlines present")
        .len();

    let result: Vec<u8> = data.iter()
        .filter(|b| **b != b'\n')
        .copied()
        .collect();

    (result.to_vec(), map_width)
}

fn normalise_map(data: &[u8]) -> Vec<u8> {
    let result: Vec<u8> = data.iter()
        .map(|b| match *b {
            b'S' => &b'a',
            b'E' => &b'z',
            _ => b
        })
    .copied()
        .collect();

    result.to_vec()
}

pub fn day12_1_result(data: &[u8]) -> u64 {
    let (map, map_width) = clean_map_data(data);
    let map = map.as_slice();

    let (start, end) = find_points_of_interest(map);
    
    let map = normalise_map(map);
    let map = map.as_slice();

    let start = start.expect("start of map was found");
    let end = end.expect("end of map was found");

    // we want to calculate the min score for each tile and then just refuse to visit it score <
    // current score
    
    let mut cost_map = CostMap::new(map.len());
    cost_map.set_start(start);
    cost_map.set_end(end);
    cost_map.calculate_costs(map, map_width);
    cost_map.cheapest_part1()
}

pub fn day12_2_result(data: &[u8]) -> u64 {
    // TODO: REMOVE DUPLICATE CODE------------
    let (map, map_width) = clean_map_data(data);
    let map = map.as_slice();

    let (start, end) = find_points_of_interest(map);
    
    let map = normalise_map(map);
    let map = map.as_slice();

    let start = start.expect("start of map was found");
    let end = end.expect("end of map was found");

    // we want to calculate the min score for each tile and then just refuse to visit it score <
    // current score
    
    let mut cost_map = CostMap::new(map.len());
    cost_map.set_start(start);
    cost_map.set_end(end);
    cost_map.calculate_costs(map, map_width);
 
    // UNIQUE CODE STARTS HERE----------------
    let all_a_positions: Vec<usize> = map.iter()
        .enumerate()
        .filter(|(_, b)| **b == b'a')
        .map(|(i, _)| i)
        .collect();
    let result = all_a_positions.iter()
        .map(|a| cost_map.map[*a])
        .min();

    result.unwrap()
}

// TODO: should try puling in a djikstra crate to see how fast it is (am betting ~10x faster..).

fn find_points_of_interest(map: &[u8]) -> (Option<usize>, Option<usize>) {
    let result = map.iter()
        .enumerate()
        .fold((None, None), |mut acc: (Option<usize>, Option<usize>), (i,&x)| {
            if x == b'S' { acc.0 = Some(i) };
            if x == b'E' { acc.1 = Some(i) };
            acc
        });
    result
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = include_bytes!("../input");

    #[test]
    fn day12_1_result_1_test() {
        assert_eq!(day12_1_result(TEST_INPUT.as_bytes()), 31);
    }

    #[test]
    fn day12_1_result_live_test() {
        assert_eq!(day12_1_result(INPUT), 380);
    }

    #[test]
    fn day12_2_result_1_test() {
        assert_eq!(day12_2_result(TEST_INPUT.as_bytes()), 29);
    }

    #[test]
    fn day12_2_result_live_test() {
        assert_eq!(day12_2_result(INPUT), 375);
    }

const TEST_INPUT: &str = r###"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"###;
}
