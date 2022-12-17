use std::time::SystemTime;
use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::Peekable,
    str,
};

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    leads: HashSet<String>,
}

struct Max {
    inner: u32,
}

#[derive(Debug, Clone)]
struct State {
    current_valve: String,
    visited: HashSet<String>,
    remain_minute: u32,
    total_pressure: u32,
}

#[derive(Debug, Clone)]
struct FolkState {
    id: u32,
    current_valve: String,
    remain_minute: u32,
}

#[derive(Debug, Clone)]
struct StatePart2 {
    visited: HashSet<String>,
    total_pressure: u32,
    // in part2 we have 2 folks
    folks: Vec<FolkState>,
}

fn parse_label<I>(iter: &mut Peekable<I>) -> String
where
    I: Iterator<Item = u8>,
{
    let mut v = Vec::new();
    while let Some(b) = iter.peek() {
        if *b <= b'Z' && *b >= b'A' {
            v.push(*b);
            iter.next();
        } else {
            break;
        }
    }
    str::from_utf8(&v).unwrap().into()
}

fn parse_number<I>(iter: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = u8>,
{
    let mut v = Vec::new();
    while let Some(b) = iter.peek() {
        if *b <= b'9' && *b >= b'0' {
            v.push(*b);
            iter.next();
        } else {
            break;
        }
    }
    str::from_utf8(&v).unwrap().parse().unwrap()
}

fn parse_line(line: &str) -> Valve {
    let bytes = line.bytes();
    let mut iter = bytes.peekable();
    let mut v = Valve {
        name: "".into(),
        rate: 0,
        leads: HashSet::new(),
    };
    while let Some(b) = iter.peek() {
        match *b {
            b' ' => {
                iter.next();
                if let Some(b) = iter.peek() {
                    if *b <= b'Z' && *b >= b'A' {
                        if v.name == "" {
                            v.name = parse_label(&mut iter);
                        } else {
                            v.leads.insert(parse_label(&mut iter));
                        }
                    }
                }
            }
            b'=' => {
                iter.next();
                v.rate = parse_number(&mut iter);
            }
            _ => {
                iter.next();
            }
        }
    }
    v
}

fn calc_moving_cost(
    from: &String,
    to: &String,
    lookup: &HashMap<String, Valve>,
    moving_cost_lookup: &mut HashMap<(String, String), Option<u32>>,
) -> Option<u32> {
    if let Some(x) = moving_cost_lookup.get(&(from.clone(), to.clone())) {
        return *x;
    }

    let mut count = 0;
    if from == to {
        return Some(count);
    }
    let mut frontiers = vec![from];
    let mut visited = HashSet::new();
    visited.insert(from);
    loop {
        count += 1;
        let mut next_frontiers = Vec::new();
        for item in frontiers {
            let valve = lookup.get(item).unwrap();
            if valve.leads.contains(to) {
                moving_cost_lookup.insert((from.clone(), to.clone()), Some(count));
                return Some(count);
            } else {
                for lead in valve.leads.iter() {
                    if !visited.contains(lead) {
                        visited.insert(lead);
                        next_frontiers.push(lead);
                    }
                }
            }
        }
        if next_frontiers.is_empty() {
            moving_cost_lookup.insert((from.clone(), to.clone()), None);
            return None;
        } else {
            frontiers = next_frontiers;
        }
    }
}

fn dfs_simulate(
    valves: &Vec<String>,
    lookup: &HashMap<String, Valve>,
    state: State,
    path_bit: u32,
    path_map_bit: &mut HashMap<u32, u32>,
    path: Option<String>,
    path_map: &mut HashMap<String, u32>,
    valve_bit_map: &HashMap<String, u32>,
    moving_cost_lookup: &mut HashMap<(String, String), Option<u32>>,
    max: &mut Max,
) {
    if state.total_pressure > max.inner {
        // println!("{total} {:?}", path);
        max.inner = state.total_pressure;
    }

    if path.is_some() {
        let x = path.as_ref().unwrap();
        path_map.insert(x.clone(), state.total_pressure);
    }

    path_map_bit.insert(path_bit, state.total_pressure);

    let visited = state.visited;

    if visited.len() == valves.len() {
        return ();
    }

    for v in valves {
        if visited.contains(v) {
            continue;
        }
        let valve = lookup.get(v).unwrap();
        if let Some(cost) = calc_moving_cost(&state.current_valve, v, lookup, moving_cost_lookup) {
            // cost is only the time cost
            // we need to spend extra 1 minute on openning the valve
            if state.remain_minute < cost + 1 {
                continue;
            } else {
                let v_bit = *valve_bit_map.get(v).unwrap();
                let path_bit0: u32 = path_bit | v_bit;

                let mut path0 = if path.is_some() {
                    path.as_ref().unwrap().clone()
                } else {
                    v.clone()
                };
                if path.is_some() {
                    path0.push(':');
                    path0.push_str(&v.clone());
                }

                let mut visited0 = visited.clone();
                visited0.insert(v.clone());
                let remain_minute0 = state.remain_minute - cost - 1;

                let state = State {
                    current_valve: v.clone(),
                    visited: visited0,
                    remain_minute: remain_minute0,
                    total_pressure: state.total_pressure + (valve.rate * remain_minute0),
                };

                dfs_simulate(
                    valves,
                    lookup,
                    state,
                    path_bit0,
                    path_map_bit,
                    Some(path0),
                    path_map,
                    valve_bit_map,
                    moving_cost_lookup,
                    max,
                )
            }
        }
    }
}

pub fn part1(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut lookup = HashMap::new();
    let mut valves = Vec::new();
    for line in cnt.lines() {
        let v = parse_line(line);
        if v.rate > 0 {
            valves.push(v.name.clone());
        }
        lookup.insert(v.name.clone(), v);
    }

    let mut max = Max { inner: 0 };
    let mut path_map = HashMap::new();
    let mut path_map_bit = HashMap::new();
    // let path = None;
    // for memoization
    let mut moving_cost_lookup = HashMap::new();

    let initial_state = State {
        current_valve: "AA".into(),
        visited: HashSet::new(),
        remain_minute: 30,
        total_pressure: 0,
    };

    let mut valve_bit_map = HashMap::new();
    let mut i = 1u32;
    for v in valves.iter() {
        valve_bit_map.insert(v.clone(), i);
        i *= 2;
    }

    dfs_simulate(
        &valves,
        &lookup,
        initial_state,
        0,
        &mut path_map_bit,
        None,
        &mut path_map,
        &valve_bit_map,
        &mut moving_cost_lookup,
        &mut max,
    );

    max.inner
}

fn dfs_simulate2(
    valves: &Vec<String>,
    lookup: &HashMap<String, Valve>,
    state: StatePart2,
    path: &Vec<String>,
    moving_cost_lookup: &mut HashMap<(String, String), Option<u32>>,
    max: &mut Max,
) {
    if state.total_pressure > max.inner {
        // println!("{} {:?}", state.total_pressure, path);
        max.inner = state.total_pressure;
    }
    // if path.len() > 0 && path[0] != String::from("EU") {
    // println!("{} {:?}", state.total_pressure, path);
    // }

    let visited = state.visited;

    if visited.len() == valves.len() {
        return ();
    }

    for v in valves {
        for f in state.folks.iter() {
            if visited.contains(v) {
                continue;
            }
            let valve = lookup.get(v).unwrap();
            if let Some(cost) = calc_moving_cost(&f.current_valve, v, lookup, moving_cost_lookup) {
                // cost is only the time cost
                // we need to spend extra 1 minute on openning the valve
                if f.remain_minute < cost + 1 {
                    continue;
                } else {
                    let mut path0 = path.clone();
                    path0.push(v.clone());

                    let mut visited0 = visited.clone();
                    visited0.insert(v.clone());
                    let remain_minute0 = f.remain_minute - cost - 1;

                    let mut next_folks = Vec::new();

                    for item in state.folks.iter() {
                        if item.id == f.id {
                            // item.current_valve = v.clone();
                            next_folks.push(FolkState {
                                id: f.id,
                                current_valve: v.clone(),
                                remain_minute: remain_minute0,
                            });
                        } else {
                            next_folks.push(item.clone());
                        }
                    }

                    let s = StatePart2 {
                        visited: visited0,
                        total_pressure: state.total_pressure + (valve.rate * remain_minute0),
                        folks: next_folks,
                    };

                    dfs_simulate2(valves, lookup, s, &mut path0, moving_cost_lookup, max)
                }
            }
        }
    }
}

pub fn part2_v2(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut lookup = HashMap::new();
    let mut valves = Vec::new();
    for line in cnt.lines() {
        let v = parse_line(line);
        if v.rate > 0 {
            valves.push(v.name.clone());
        }
        lookup.insert(v.name.clone(), v);
    }

    println!("{:?}", valves);

    let mut max = Max { inner: 0 };
    let mut path = Vec::new();

    // for memoization
    let mut moving_cost_lookup = HashMap::new();

    let initial_state = StatePart2 {
        visited: HashSet::new(),
        total_pressure: 0,
        folks: vec![
            FolkState {
                id: 0,
                current_valve: "AA".into(),
                remain_minute: 26,
            },
            FolkState {
                id: 1,
                current_valve: "AA".into(),
                remain_minute: 26,
            },
        ],
    };

    dfs_simulate2(
        &valves,
        &lookup,
        initial_state,
        &mut path,
        &mut moving_cost_lookup,
        &mut max,
    );

    max.inner
}

pub fn part2(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut lookup = HashMap::new();
    let mut valves = Vec::new();
    for line in cnt.lines() {
        let v = parse_line(line);
        if v.rate > 0 {
            valves.push(v.name.clone());
        }
        lookup.insert(v.name.clone(), v);
    }

    let mut max = Max { inner: 0 };
    let mut path_map = HashMap::new();
    let mut path_map_bit = HashMap::new();
    let path = None;
    // for memoization
    let mut moving_cost_lookup = HashMap::new();

    let initial_state = State {
        current_valve: "AA".into(),
        visited: HashSet::new(),
        remain_minute: 26,
        total_pressure: 0,
    };

    let mut valve_bit_map = HashMap::new();
    let mut i = 1u32;
    for v in valves.iter() {
        valve_bit_map.insert(v.clone(), i);
        i *= 2;
    }

    dfs_simulate(
        &valves,
        &lookup,
        initial_state,
        0,
        &mut path_map_bit,
        path,
        &mut path_map,
        &valve_bit_map,
        &mut moving_cost_lookup,
        &mut max,
    );

    let all_path = path_map
        .into_iter()
        .map(|(key, value)| {
            (
                key.split(':')
                    .map(|x| valve_bit_map.get(x).unwrap())
                    .fold(0u32, |acc, x| acc | x),
                value,
            )
        })
        .collect::<Vec<_>>();

    let now = SystemTime::now();

    let mut count = 0u128;
    let mut max_pressure = 0;
    for (path0, pressure0) in all_path.iter() {
        for (path1, pressure1) in all_path.iter() {
            if count % 100_000_000 == 0 {
                println!(
                    "iter={} elapsed={}",
                    count,
                    now.elapsed().unwrap().as_secs()
                );
            }

            if path0 & path1 == 0 {
                if pressure0 + pressure1 > max_pressure {
                    max_pressure = pressure0 + pressure1;
                }
            }

            count += 1;
        }
    }
    max_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1651, part1("data/day16-sample.txt"));
        assert_eq!(2056, part1("data/day16.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1707, part2("data/day16-sample.txt"));
        // warnning - this takes 2 minutes to finish
        // assert_eq!(2513, part2("data/day16.txt"));
    }
}
