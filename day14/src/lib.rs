use std::{usize, collections::VecDeque};

pub fn day14_1_result(data: &str) -> u64 {
    let mut cave = Cave::new(data);
    cave.create_map();
    cave.show_cave();
    cave.pour_sand()
}

pub fn day14_2_result(data: &str) -> u64 {
   let data2 = format!("{}{}", data, "\n250,163 -> 700,163"); 
    let mut cave = Cave::new(data2.as_str());
    cave.create_map();
    cave.show_cave();
    cave.pour_sand()
// y = 163, x = ( - )
}

#[derive(Debug, Default)]
struct Cave {
    rocks: Vec<Rock>,
    contents: Vec<u8>,
    bounds: Vector,
}

#[derive(Debug, Clone, Copy, Default)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct Vector {
    from: Coord,
    to: Coord,
}


#[derive(Debug, Default)]
struct Rock {
    edges: Vec<Vector>,
}

impl Cave {
    const SAND_ORIGIN: Coord = Coord { x: 500, y: 0};
    pub fn new(data: &str) -> Cave {
        let stage1: Vec<Vec<Vector>> = data
            .lines()
            .map(|rock| rock.split(" -> ").collect::<Vec<&str>>())
            .map(Self::make_rock_edges)
            .collect();

        let stage2 = stage1.iter()
            .map(|x| Rock { edges: x.to_vec() })
            .collect();

        let bounds = Self::find_grid_bounds(&stage2);

        Cave { 
            rocks: stage2, 
            contents: vec![b'.';Self::slice_size_from_bounds(bounds)], 
            bounds }
    }

    fn find_grid_bounds(rocks: &Vec<Rock>) -> Vector {
        let top_left = Coord{ x: usize::MAX, y: usize::MAX };
        let bottom_right = Coord{ x: usize::MIN, y: usize::MIN };
        let mut bounds: Vector = Vector { from: top_left, to: bottom_right };

        rocks.iter()
            .flat_map(|r| r.edges.iter())
            .flat_map(|x| [[x.from.x, x.from.y], [x.to.x, x.to.y]])
            .for_each(|i|  Self::update_bounds(&mut bounds, Coord {x: i[0], y: i[1]})
            );
        Self::update_bounds(&mut bounds, Self::SAND_ORIGIN);
        bounds
    }

    fn update_bounds(bounds: &mut Vector, coord: Coord){
        bounds.from.x = std::cmp::min(bounds.from.x, coord.x);
        bounds.from.y = std::cmp::min(bounds.from.y, coord.y);
        bounds.to.x = std::cmp::max(bounds.to.x, coord.x);
        bounds.to.y = std::cmp::max(bounds.to.y, coord.y);
    }


    fn make_rock_edges(data: Vec<&str>) -> Vec<Vector> {
        data.iter()
            .flat_map(|pair| pair.split_once(','))
            .map(|(x, y)| Coord {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            })
            .fold((vec![], None), |(mut acc, last_coord), to| -> (Vec<Vector>, Option<Coord>) {
                if let Some(from) = last_coord {
                    acc.push(Vector { from, to});
                }
                (acc, Some(to))
            }).0
    }
    
    fn slice_size_from_bounds(bounds: Vector) -> usize {
        ((1 + bounds.to.x - bounds.from.x) * (1 + bounds.to.y - bounds.from.y)) as usize
    } 

    fn adjusted_bounds(&self) -> Vector {
        Vector { 
            from: self.translate_coord(&self.bounds.from),
            to: self.translate_coord(&self.bounds.to)
        }
    }

    #[inline]
    fn translate_coord(&self, xy: &Coord) -> Coord {
        Coord { 
            x: xy.x - self.bounds.from.x, 
            y: xy.y - self.bounds.from.y }
    }
    
    #[inline]
    fn coord_to_index(&self, xy: &Coord) -> usize {

       (self.adjusted_bounds().to.x + 1) * xy.y + xy.x
    }

    #[inline]
    fn edge_to_index(&self, rock_coord: &Coord) -> usize {
        let translated = self.translate_coord(rock_coord);
        
        self.coord_to_index(&translated)
    }

    #[inline]
    fn list_rock_edge(&self, edge: &Vector) -> Vec<usize>{
        let mut results: Vec<usize> = vec![];

        if edge.from.x != edge.to.x {
            for x in std::cmp::min(edge.from.x, edge.to.x)..=std::cmp::max(edge.from.x, edge.to.x) {
                results.push(self.edge_to_index(&Coord { x, y: edge.to.y } ));
            }
        }
        else if edge.from.y != edge.to.y {
            for y in std::cmp::min(edge.from.y, edge.to.y)..=std::cmp::max(edge.from.y, edge.to.y) {
                results.push(self.edge_to_index(&Coord { x: edge.to.x, y } ));
            }
        }
        else {
            results.push(self.edge_to_index(&edge.to));
        }
        results
    }

    pub fn create_map(&mut self) {
        let mut rock_positions : Vec<usize> = vec![];
        let rocks = &self.rocks;
        let edges: Vec<Vector> = rocks.iter().flat_map(|rock| rock.edges.iter().map(|edge| *edge)).collect();
        for edge in edges.iter() {
            rock_positions.append(&mut self.list_rock_edge(edge));
        }
        for rock_pos in rock_positions {
            self.contents[rock_pos] = b'#';
        }
    }

    pub fn show_cave(&self) {
        let width = self.cave_width();
        for i in 0..self.contents.len() {
            let x = self.contents[i] as char;
            if i%width == width -1  {
                println!("{x}");
            }
            else if i == 6 {
                print!("+");
            }
            else {
                print!("{x}");
            }
        }
        println!("BOUNDS: {:?}", self.bounds);
        println!("ADJUSTED BOUNDS: {:?}", self.adjusted_bounds());
        println!("SAND IDX: {:?}", self.edge_to_index(&Self::SAND_ORIGIN));
    }
    fn cave_width(&self) -> usize {
         1 + self.bounds.to.x - self.bounds.from.x
    }
    pub fn pour_sand(&mut self) -> u64 {
        println!("HERE COMES THE SAND: ");
        let sand_index = self.edge_to_index(&Self::SAND_ORIGIN);
        
        let mut sand_path: VecDeque<usize> = VecDeque::new();
        sand_path.push_back(sand_index);
        let mut particles:u64 = 0;
        while let Some(next) = self.next_sand_position(&mut sand_path) {
            particles += 1;
            if next == sand_index { break; }
            self.contents[next] = b'o';
        }
        self.show_cave(); 

        particles
    }

    fn next_sand_position(&self, sand_path: &mut VecDeque<usize>) -> Option<usize> {
        let last_start = sand_path.get(0);
        let potential_position = last_start.unwrap() + self.cave_width();
        if let Some(cell) = self.contents.get(potential_position) {
            let stop = match cell {
                
                b'o' | b'#' => last_start,
                b'.' => {
                    sand_path.push_front(potential_position);
                    return self.next_sand_position(sand_path)
                },
                _ => panic!("something unknown is down in that cave... be scared"),
            };
            if potential_position%self.cave_width() > 0 { 
                if let Some(left) = self.contents.get(potential_position-1) {
                    match left {
                        b'.' => { 
                            sand_path.push_front(potential_position - 1);
                            return self.next_sand_position(sand_path);
                        },
                        b'o' | b'#' => {
                            ();
                        },
                        _ => panic!("crashed on left, invalid contents"),
                    }
                }
            }
            else { 
                return None; 
            }
            if potential_position%self.cave_width() < self.cave_width()-1 { 
                if let Some(right) = self.contents.get(potential_position+1)  {
                    match right {
                        b'.' => {
                            sand_path.push_front(potential_position + 1);
                            return self.next_sand_position(sand_path);
                        },
                        b'o' | b'#'  => { 
                            ();
                        },
                        _ => panic!("crashed on right, invalid contents"),
                    }
                }
            }
            else {
                return None;
            }
        }
        else {
            return None;
        }
        sand_path.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day14_1_result_1_test() {
        assert_eq!(day14_1_result(TEST_INPUT), 24);
    }

    // #[test]
    fn day14_1_result_live_test() {
        assert_eq!(day14_1_result(INPUT), 4809);
    }

    // #[test]
    fn day14_2_result_1_test() {
        // assert_eq!(day14_2_result(TEST_INPUT.as_bytes()), 29);
    }

    // #[test]
    fn day14_2_result_live_test() {
        // assert_eq!(day14_2_result(INPUT), 375);
    }
    const TEST_INPUT: &str = r###"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"###;
}
