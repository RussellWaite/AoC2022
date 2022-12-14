mod any_row;
mod range;

use any_row::AnyRow;

use regex::Regex;

pub fn day15_1_result(data: &str, row: usize) -> usize {
    let (readings, min_x, max_x) = parse_input(data);
    let mut special: SpecialRow = SpecialRow::new(min_x, max_x);
    // special.print_stats();

    readings
        .iter()
        .filter(|r| (row as i64 - r.sensor.y).abs() <= r.distance)
        .for_each(|s| {
            let moves_remaining = s.distance - (row as i64 - s.sensor.y).abs();
            special.add_coverage(s.sensor.x, moves_remaining);
        });

    readings
        .iter()
        .filter(|r| r.beacon.y == row as i64)
        .for_each(|r| special.remove_beacon(r.beacon.x));

    special.count_filled_columns()
}

pub fn day15_2_result(data: &str, min_xy: usize, max_xy: usize) -> usize {
    let (readings, min_x, max_x) = parse_input(data);
    const TUNING_FREQUENCY: usize = 4_000_000;

    for row in min_xy..=max_xy {
        let mut current = AnyRow::new(min_xy, max_xy);
        readings
            .iter()
            .filter(|r| (row as i64 - r.sensor.y).abs() <= r.distance)
            .for_each(|s| {
                let moves_remaining = s.distance - (row as i64 - s.sensor.y).abs();
                current.add_range(s.sensor.x, moves_remaining.abs());
            });

        if current.is_there_a_gap() {
            return TUNING_FREQUENCY * current.what_is_that_gap() + row;
        }
    }

    0
}

fn parse_input(data: &str) -> (Vec<Reading>, i64, i64) {
    let mut readings: Vec<Reading> = vec![];
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;

    let re =
        Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)")
            .unwrap();
    for capture in re.captures_iter(data) {
        let sensor_x = capture[1].parse::<i64>().unwrap();
        let sensor_y = capture[2].parse::<i64>().unwrap();
        let beacon_x = capture[3].parse::<i64>().unwrap();
        let beacon_y = capture[4].parse::<i64>().unwrap();
        readings.push(Reading {
            sensor: Coord {
                x: sensor_x,
                y: sensor_y,
            },
            beacon: Coord {
                x: beacon_x,
                y: beacon_y,
            },
            distance: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
        });

        if sensor_x < min_x {
            min_x = sensor_x;
        }
        if beacon_x < min_x {
            min_x = beacon_x;
        }
        if sensor_x > max_x {
            max_x = sensor_x;
        }
        if beacon_x > max_x {
            max_x = beacon_x;
        }
    }
    (readings, min_x, max_x)
}
#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Reading {
    sensor: Coord,
    beacon: Coord,
    distance: i64,
}

struct SpecialRow {
    contents: Vec<bool>,
    min: i64,
    max: i64,
    offset: i64,
}

impl SpecialRow {
    pub fn new(min_x: i64, max_x: i64) -> Self {
        Self {
            // hack by adding 2* for size - I think I'd constraining
            // the ranges too much for live data
            contents: vec![false; 2 * (max_x - min_x) as usize + 1],
            min: min_x,
            max: max_x,
            offset: 0 - min_x,
        }
    }

    fn print_stats(&self) {
        println!(
            "SpecialRow as {} columns, with min: {}, max: {}, and offset: {}",
            self.contents.len(),
            self.min,
            self.max,
            self.offset
        );
    }

    fn count_filled_columns(&self) -> usize {
        self.contents.iter().filter(|c| **c).count()
    }

    fn add_coverage(&mut self, position: i64, traversal: i64) {
        let start = position - traversal;
        let end = position + traversal;
        for x in start..=end {
            self.contents[(x + self.offset) as usize] = true;
        }
    }

    fn remove_beacon(&mut self, x: i64) {
        self.contents[(x + self.offset) as usize] = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::range::Range;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day15_1_result_1_test() {
        assert_eq!(day15_1_result(TEST_INPUT, 10), 26);
    }

    #[test]
    fn day15_1_result_live_test() {
        assert_eq!(day15_1_result(INPUT, 2_000_000), 5_144_286);
    }

    #[test]
    fn day15_2_result_1_test() {
        assert_eq!(day15_2_result(TEST_INPUT, 0, 20), 56_000_011);
    }

    #[test]
    fn day15_2_result_live_test() {
        // assert_eq!(day15_2_result(INPUT, 0, 4_000_000), 10_229_191_267_339);
    }
    const TEST_INPUT: &str = r###"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"###;
}
