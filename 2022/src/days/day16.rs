use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use run_aoc::runner_fn;
use utils::{nom_usize, simple_struct};

fn parse_valve_descr(input: &str) -> Vec<ValveDescription> {
    let (leftover, vd) = valve_descriptions(input).expect("Could not parse valve descriptions");
    assert_eq!(leftover, "");
    vd
}

fn valve_descriptions(input: &str) -> IResult<&str, Vec<ValveDescription>> {
    terminated(separated_list1(newline, valve_description), newline)(input)
}

fn valve_description(input: &str) -> IResult<&str, ValveDescription> {
    map(tuple((id, flow_rate, tunnels)), |(i, fr, t)| {
        ValveDescription::new(i, fr, t)
    })(input)
}

fn id(input: &str) -> IResult<&str, String> {
    preceded(tag("Valve "), valve_id)(input)
}

fn flow_rate(input: &str) -> IResult<&str, usize> {
    preceded(tag(" has flow rate="), nom_usize)(input)
}

fn tunnels(input: &str) -> IResult<&str, Vec<String>> {
    preceded(
        tuple((
            tag("; "),
            alt((tag("tunnels lead"), tag("tunnel leads"))),
            tag(" to "),
            alt((tag("valve "), tag("valves "))),
        )),
        separated_list1(tag(", "), valve_id),
    )(input)
}

fn valve_id(input: &str) -> IResult<&str, String> {
    map(alpha1, |s| String::from(s))(input)
}

simple_struct!(Valve; flow_rate: usize, tunnels: Vec<usize>);

// temporarily used to hold Valve descriptions,
// before converting IDs to indices
simple_struct!(ValveDescription; id: String, flow_rate: usize, tunnels: Vec<String>);

impl ValveDescription {
    fn into_valve(self, id_to_index: &HashMap<String, usize>) -> Valve {
        Valve::new(
            self.flow_rate,
            self.tunnels
                .into_iter()
                .map(|id| id_to_index.get(&id).unwrap().clone())
                .collect::<Vec<usize>>(),
        )
    }
}

// map valve IDs to indices
// (instead of looking them up in a hashmap every time)
fn map_ids_to_index(vds: &Vec<ValveDescription>) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    vds.iter().enumerate().for_each(|(i, vd)| {
        map.insert(vd.id.clone(), i);
    });
    map
}

// Floyd-Warshall algorithm
// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
// TODO: can I generalize and extract this somehow?
fn map_all_distances(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    let mut dist: Vec<Vec<usize>> = Vec::new();

    // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
    let num_vertices = valves.len();
    for _ in 0..num_vertices {
        // (avoid add with overflow)
        let temp_vec = vec![usize::MAX / 3; num_vertices];
        dist.push(temp_vec);
    }

    // for each edge (u, v) do
    //     dist[u][v] ← w(u, v)  // The weight of the edge (u, v)
    for (i, v) in valves.iter().enumerate() {
        for t in v.tunnels.iter() {
            // (for this all edge weights are 1)
            dist[i][*t] = 1;
        }
        // for each vertex v do
        //     dist[v][v] ← 0
        dist[i][i] = 0;
    }

    // for k from 1 to |V|
    //     for i from 1 to |V|
    //         for j from 1 to |V|
    //             if dist[i][j] > dist[i][k] + dist[k][j]
    //                 dist[i][j] ← dist[i][k] + dist[k][j]
    //             end if
    for k in 0..num_vertices {
        for i in 0..num_vertices {
            for j in 0..num_vertices {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }

    dist
}

struct Cave {
    valves: Vec<Valve>,
    // map of Valve name to it's index
    //id_map: HashMap<String, usize>,
    // distance from any Valve to any other Valve
    distance_map: Vec<Vec<usize>>,
    // index of Valve 'AA'
    start_index: usize,
}

impl Cave {
    fn parse(input: &str) -> Self {
        let descriptions = parse_valve_descr(input);
        let id_map: HashMap<String, usize> = map_ids_to_index(&descriptions);
        let valves: Vec<Valve> = descriptions
            .into_iter()
            .map(|vd| vd.into_valve(&id_map))
            .collect();
        let distance_map = map_all_distances(&valves);
        let start_index = *(id_map.get(&"AA".to_string()).unwrap());

        Cave {
            valves,
            //id_map,
            distance_map,
            start_index,
        }
    }

    // max pressure released if one person does it solo
    fn max_pressure(&self, state: ValveState, time_left: usize) -> usize {
        self.valves
            .iter()
            .enumerate()
            .filter_map(|(id, v)| {
                // don't need to try these cases
                if id == state.current || state.is_open(id) || v.flow_rate == 0 {
                    return None;
                }
                // time to travel plus open the valve
                let time_needed = self.distance_map[state.current][id] + 1;

                if time_needed > time_left {
                    return None;
                }
                // where to travel, the time needed, and the flow rate
                Some((id, time_needed, v.flow_rate))
            })
            .map(|(id, time_needed, flow_rate)| {
                let time_left_now = time_left - time_needed;
                let pressure = time_left_now * flow_rate;
                pressure + self.max_pressure(state.travel_and_open(id), time_left_now)
            })
            .max()
            .unwrap_or(0)
    }

    // return set of valves activated and corresponding flow rate
    fn possible_valve_seqs(&self, state: ValveState, time_left: usize) -> HashSet<(u64, usize)> {
        let mut states: HashSet<(u64, usize)> = HashSet::new();

        // most of this is same as above, should refactor (but whatever)
        self.valves
            .iter()
            .enumerate()
            .filter_map(|(id, v)| {
                // don't need to try these cases
                if id == state.current || state.is_open(id) || v.flow_rate == 0 {
                    return None;
                }
                // time to travel plus open the valve
                let time_needed = self.distance_map[state.current][id] + 1;

                if time_needed > time_left {
                    return None;
                }
                // where to travel, the time needed, and the flow rate
                Some((id, time_needed, v.flow_rate))
            })
            .for_each(|(id, time_needed, flow_rate)| {
                let time_left_now = time_left - time_needed;
                let pressure = time_left_now * flow_rate;

                self.possible_valve_seqs(state.travel_and_open(id), time_left_now)
                    .iter()
                    .for_each(|(valve_bitfield, flow)| {
                        states.insert((*valve_bitfield, flow + pressure));
                    });
            });

        // set the valves that were opened for this sequence
        //println!("Found seq {:b}", state.open);
        states.insert((state.open, 0));
        states
    }

    // max pressure with help from one elephant
    fn max_with_elephant(&self, state: ValveState, time_left: usize) -> usize {
        // enumerate all possible valve sequences,
        let all_valve_sequences = self.possible_valve_seqs(state, time_left);
        let num_seq = all_valve_sequences.len();
        println!("Found {} possible sequences", num_seq);

        // find the highest flow rate for each individual sequence
        let mut flow_map: HashMap<u64, usize> = HashMap::new();
        for (bitfield, flow_rate) in all_valve_sequences.into_iter() {
            if let Some(flow) = flow_map.get(&bitfield) {
                if flow_rate > *flow {
                    flow_map.insert(bitfield, flow_rate);
                }
            } else {
                flow_map.insert(bitfield, flow_rate);
            }
        }
        println!("Reduced to {} sequences with highest flows", flow_map.len());

        // then figure out which two sequences that each opened different valves give the highest combined flow
        let mut max_flow = 0;
        for (bitfield1, flow_rate1) in flow_map.iter() {
            for (bitfield2, flow_rate2) in flow_map.iter() {
                if bitfield1 & bitfield2 == 0 {
                    max_flow = max(max_flow, flow_rate1 + flow_rate2);
                }
            }
        }
        max_flow
    }
}

struct ValveState {
    // what Valve are we currently at
    current: usize,
    // which Valves are open
    // (in the full input there are only 58 valves,
    // so I can track the open/close state of each using a 64-bit bitfield)
    open: u64,
}

impl ValveState {
    fn new(current: usize) -> Self {
        ValveState { current, open: 0 }
    }

    fn travel_and_open(&self, to_index: usize) -> Self {
        ValveState {
            current: to_index,
            open: self.open | 1 << to_index,
        }
    }

    fn is_open(&self, valve_index: usize) -> bool {
        self.open & (1 << valve_index) > 0
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let cave = Cave::parse(&file_contents);
    let pressure = cave.max_pressure(ValveState::new(cave.start_index), 30);

    pressure
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let cave = Cave::parse(&file_contents);
    let pressure = cave.max_with_elephant(ValveState::new(cave.start_index), 26);

    pressure
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day16, part1, example, 1651);
    test_fn!(day16, part1, input, 1595);

    test_fn!(day16, part2, example, 1707);
    // TODO: this takes a little too long to run regularly
    // test_fn!(day16, part2, input, 2189);
}
