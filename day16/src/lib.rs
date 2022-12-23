use std::collections::{HashMap, VecDeque};
use regex::Regex;

pub fn day16_1_result(data: &str, start_at: &str) -> i64 {
    let owned_data = parse_input_as_owned_data(data);
    let scans = parse_input_as_hashmap(&owned_data);
    const START_TIME: i64 = 30;
    let useful_valves: Vec<&str> = 
        scans.iter()
            .filter(|(_valve, details)| details.flow > 0)
            .map(|(key,  _val)| *key)
            .collect();
    
    let useful_valves_count = useful_valves.len();

    let _cost_map: HashMap<String, Valve> = HashMap::with_capacity(useful_valves_count);
    let distance_map = create_distance_graph(&scans, &useful_valves, start_at);
    let valve_index = create_state_index(start_at, &useful_valves);
    let cache: HashMap<TimedValveState, i64> = HashMap::new();
    
    dfs(
        cache, 
        TimedValveState { state: 0, time: START_TIME, valve: start_at },
        &distance_map,
        &valve_index,
        &scans
    ).0
}
// AA FF GG II all have a zero value in test set.
pub fn day16_2_result(data: &str, start_at: &str) -> i64 {

    let owned_data = parse_input_as_owned_data(data);
    let scans = parse_input_as_hashmap(&owned_data);
    const START_TIME: i64 = 26;
    let useful_valves: Vec<&str> = 
        scans.iter()
            .filter(|(_valve, details)| details.flow > 0)
            .map(|(key,  _val)| *key)
            .collect();
    
    let useful_valves_count = useful_valves.len();

    let _cost_map: HashMap<String, Valve> = HashMap::with_capacity(useful_valves_count);
    let distance_map = create_distance_graph(&scans, &useful_valves, start_at);
    let valve_index = create_state_index(start_at, &useful_valves);
    let cache: HashMap<TimedValveState, i64> = HashMap::new();
    
    // need to split it so we do one half, elephant does other
    // could use state and make call to work twice (but only going over half the options...)
    // this means we need to loop through each start state though, so workload is going to
    // dramatically increase...
    let mut max_so_far = 0;
    let possibilities = (1 << valve_index.len()) - 1;
    for i in 0..possibilities/2 { // if we do one have, elephant must do other...
        max_so_far = std::cmp::max(max_so_far, 
            dfs(cache.clone(), 
                TimedValveState { time: START_TIME, valve: start_at, state: i }, 
                &distance_map, 
                &valve_index, 
                &scans).0 + 
            dfs(cache.clone(), 
                // same except need to invert state...
                TimedValveState { time: START_TIME, valve: start_at, state: possibilities ^ i }, 
                &distance_map, 
                &valve_index, 
                &scans).0
        ); // the cache isn't being used effectively... need to refactor to allow shared state...
    }
    // this is going to be super slow... it took minutes, urgh.
    max_so_far
}

// I'm passing in a lot of hashmaps here am up to 4 as of time of writing this - seems inelegant
fn dfs<'a>( 
    cache: HashMap<TimedValveState<'a>, i64>, 
    step: TimedValveState<'a>,
    distance_map: &'a HashMap<&'a str, HashMap<&'a str, i64>>,
    valve_index: &'a HashMap<&'a str, u64>,
    flow_rate: &'a HashMap<&'a str, Valve<'a>>) -> (i64, HashMap<TimedValveState<'a>, i64>) {    
    let mut cache = cache;
    if cache.contains_key(&step) { return (cache[&step], cache); }
    let mut result:i64 = 0;
    
    for potential in distance_map[step.valve].iter() {
        // logic AND to see if valve is on, if so we don't need to visit diectly again
        if step.state & (1 << valve_index[potential.0]) > 0 { continue; }

        // do we have enough time to get to valve (potential.1) and turn it on (-1)
        let potential_time: i64 = step.time - potential.1 - 1;
        if potential_time <= 0 { continue; }

        let temp = TimedValveState { 
            time: potential_time, 
            valve: potential.0, 
            state: step.state | 1 << valve_index[potential.0] 
        };
        let (value, modified_cache) = dfs(cache, temp.clone(), distance_map, valve_index, flow_rate); 
        result = std::cmp::max(result, value + flow_rate[potential.0].flow as i64 * potential_time);
        cache = modified_cache;
    }

    cache.insert(step.clone(), result);

    (result, cache)
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct TimedValveState<'a> {
    time: i64,
    valve: &'a str,
    state: u64
}


#[derive(Debug, Clone)]
struct OwnedDataValve {
    name: String,
    flow: u64,
    paths: String
}

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow: u64,
    paths: Vec<&'a str>
}

impl<'a> From<&'a OwnedDataValve> for Valve<'a> {
    fn from(value: &'a OwnedDataValve) -> Self {
        Valve {
            name: value.name.as_str(),
            flow: value.flow,
            paths: value.paths.split(", ").collect(),
        }
    }
}

fn create_distance_graph<'a>(
    scans: &'a HashMap<&str, Valve>, 
    useful_valves: &Vec<&str>,
    start_at: &str) -> HashMap<&'a str, HashMap<&'a str, i64>> {

    let mut distances: HashMap<&str, HashMap<&str, i64>> = HashMap::new();

    for valve in scans.values() {
        let mut to_visit: VecDeque<(&str, u64)> = 
            VecDeque::from_iter(valve.paths.iter().map(|x| (*x, 1)));
        if useful_valves.contains(&valve.name) || valve.name == start_at {
            distances.insert(valve.name, HashMap::new());
        }
        let mut processed: Vec<&str> = vec![valve.name];

        while !to_visit.is_empty() {
             
            let Some((next, distance)) = to_visit.pop_front() else { 
                panic!("the to_visit HashMap said it wasn't empty but pop front failed...") 
            };

            if useful_valves.contains(&next) {
                distances.entry(valve.name)
                    .and_modify(|map| { map.entry(next).or_insert(distance as i64); } );
            }

            for path in &scans[next].paths { 
                if !processed.contains(path) {      
                    processed.push(path);
                    to_visit.push_back((path, distance + 1));
                }
            }
        }
    }
    distances
}

fn create_state_index<'a>(start_at: &'a str, useful_valves: &Vec<&'a str>) -> HashMap<&'a str, u64> {
    let mut result: HashMap<&str, u64> = HashMap::new();
    let mut counter = 1;
    result.insert(start_at, counter);
    for valve in useful_valves {
        counter += 1;
        result.insert(valve, counter);
    }
    result
}

fn parse_input_as_owned_data(data: &str) -> Vec<OwnedDataValve> {
    let mut result: Vec<OwnedDataValve> = vec![];
    let re =
        Regex::new(r"Valve (..) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? (.*)")
            .unwrap();

    for capture in re.captures_iter(data) {
        result.push(OwnedDataValve { 
            name: capture[1].to_owned(), 
            flow: capture[2].parse::<u64>().unwrap(),
            paths: capture[3].to_owned(),
        });
    }
    result
}

fn parse_input_as_hashmap<'a>(data: &'a Vec<OwnedDataValve>) -> HashMap<&'a str, Valve> {
    data.iter()
        .map(|odv| (odv.name.as_str(), Valve::from(odv)))
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input");

    #[test]
    fn day16_1_result_1_test() {
        assert_eq!(day16_1_result(TEST_INPUT, "AA"), 1651);
    }

    #[test]
    fn day16_1_result_live_test() {
        assert_eq!(day16_1_result(INPUT, "AA"), 1724);
    }

    #[test]
    fn day16_2_result_1_test() {
        assert_eq!(day16_2_result(TEST_INPUT, "AA"), 1707);
    }

    //#[test] - commented out as its around a 4 minute operation :scream:
    fn day16_2_result_live_test() {
        assert_eq!(day16_2_result(INPUT, "AA"), 2283);
    }
    const TEST_INPUT: &str = r###"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"###;
}
