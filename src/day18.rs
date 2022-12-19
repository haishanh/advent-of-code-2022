use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, str,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Position { x, y, z }
    }

    fn from_str(input: &str) -> Self {
        let mut iter = input.split(",").map(|x| x.parse::<u32>().unwrap());
        // move all points 2 unit away from (0,0,0)
        // so in part2 all cubes ban be reached
        let x = iter.next().unwrap() + 2;
        let y = iter.next().unwrap() + 2;
        let z = iter.next().unwrap() + 2;
        Position { x, y, z }
    }
}

fn parse_file(filepath: &str) -> Vec<Position> {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut pos = Vec::new();
    let mut bound = 0;
    for line in cnt.lines() {
        let p = Position::from_str(line);
        if p.x > bound {
            bound = p.x;
        }
        if p.y > bound {
            bound = p.y;
        }
        if p.z > bound {
            bound = p.z;
        }
        pos.push(p);
    }
    println!("{}", bound);
    pos
}

fn shift_cubes(positions: &mut Vec<Position>) {
    for pos in positions.iter_mut() {
        let x = pos.x;
        pos.x = pos.y;
        pos.y = pos.z;
        pos.z = x;
    }
}

fn unshift_cubes(positions: &mut Vec<Position>) {
    for pos in positions.iter_mut() {
        let z = pos.z;
        pos.z = pos.y;
        pos.y = pos.x;
        pos.x = z;
    }
}

fn count_faces(positions: &Vec<Position>) -> u32 {
    let mut z_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for pos in positions {
        let key = (pos.x, pos.y);
        if let Some(z) = z_map.get_mut(&key) {
            z.push(pos.z);
        } else {
            z_map.insert(key, vec![pos.z]);
        }
    }

    let mut count = 0;

    for (_k, v) in z_map.iter_mut() {
        count += 2;

        v.sort();
        let mut iter = v.iter();
        let mut prev = iter.next().unwrap();
        while let Some(x) = iter.next() {
            if x - prev > 1 {
                count += 2;
            }
            prev = x;
        }
    }

    count
}

fn find_z_axis_bound(positions: &Vec<Position>) -> HashMap<(u32, u32), (u32, u32)> {
    let mut z_map: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    for pos in positions {
        let key = (pos.x, pos.y);
        if let Some(z) = z_map.get_mut(&key) {
            if pos.z < z.0 {
                z.0 = pos.z;
            } else if pos.z > z.1 {
                z.1 = pos.z;
            }
        } else {
            z_map.insert(key, (u32::MAX, 0));
        }
    }
    z_map
}

fn find_empty_space(positions: &Vec<Position>) -> Vec<Position> {
    let mut empty_positions = Vec::new();

    let mut z_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for pos in positions {
        let key = (pos.x, pos.y);
        if let Some(z) = z_map.get_mut(&key) {
            z.push(pos.z);
        } else {
            z_map.insert(key, vec![pos.z]);
        }
    }

    for (k, v) in z_map.iter_mut() {
        v.sort();

        let mut iter = v.iter();
        let mut prev = iter.next().unwrap();
        while let Some(x) = iter.next() {
            if x - prev > 1 {
                for z in (*prev + 1)..(*x) {
                    empty_positions.push(Position { x: k.0, y: k.1, z });
                }
            }
            prev = x;
        }
    }

    empty_positions
}

fn expand_point(point: &Position) -> Vec<Position> {
    let Position { x, y, z } = point;
    let mut v = vec![
        Position::new(*x + 1, *y, *z),
        Position::new(*x, *y + 1, *z),
        Position::new(*x, *y, *z + 1),
    ];
    if *x > 0 {
        v.push(Position::new(*x - 1, *y, *z));
    }
    if *y > 0 {
        v.push(Position::new(*x, *y - 1, *z));
    }
    if *z > 0 {
        v.push(Position::new(*x, *y, *z - 1));
    }
    v
}

fn is_reachable(
    point: &Position,
    all: &HashSet<&Position>,
    x_bound_lookup: &HashMap<(u32, u32), (u32, u32)>,
    y_bound_lookup: &HashMap<(u32, u32), (u32, u32)>,
    z_bound_lookup: &HashMap<(u32, u32), (u32, u32)>,
    position_reachbility_lookup: &mut HashMap<Position, bool>,
) -> bool {
    if let Some(v) = position_reachbility_lookup.get(point) {
        return *v;
    }

    let mut frontiers = expand_point(point);
    let mut visited: HashSet<Position> = HashSet::new();

    loop {
        let mut next_frontiers = Vec::new();
        for frontier in frontiers.into_iter() {
            if visited.contains(&frontier) {
                continue;
            }
            visited.insert(frontier.clone());
            if all.contains(&frontier) {
                continue;
            }
            if let Some(z_bound) = z_bound_lookup.get(&(frontier.x, frontier.y)) {
                if frontier.z > z_bound.1 || frontier.z < z_bound.0 {
                    position_reachbility_lookup.insert(point.clone(), true);
                    return true;
                }
            }
            if let Some(y_bound) = y_bound_lookup.get(&(frontier.z, frontier.x)) {
                if frontier.y > y_bound.1 || frontier.y < y_bound.0 {
                    position_reachbility_lookup.insert(point.clone(), true);
                    return true;
                }
            }
            if let Some(x_bound) = x_bound_lookup.get(&(frontier.y, frontier.z)) {
                if frontier.x > x_bound.1 || frontier.x < x_bound.0 {
                    position_reachbility_lookup.insert(point.clone(), true);
                    return true;
                }
            }
            let points = expand_point(&frontier);
            for p in points {
                next_frontiers.push(p);
            }
        }
        if next_frontiers.len() == 0 {
            position_reachbility_lookup.insert(point.clone(), false);
            return false;
        }
        frontiers = next_frontiers;
    }
}

fn count_all_faces(pos: &mut Vec<Position>) -> u32 {
    let mut count = 0;
    count += count_faces(pos);
    shift_cubes(pos);
    count += count_faces(pos);
    shift_cubes(pos);
    count += count_faces(pos);

    count
}

pub fn part1(filepath: &str) -> u32 {
    let mut pos = parse_file(filepath);
    count_all_faces(&mut pos)
}

pub fn part2(filepath: &str) -> u32 {
    let pos = parse_file(filepath);
    let all = pos.iter().collect::<HashSet<_>>();

    let bound = 25;

    let mut count = 0;
    // thinking water is flooding from (0,0,0)
    let start = Position { x: 0, y: 0, z: 0 };
    let mut queue = VecDeque::from([start]);
    let mut visited = HashSet::new();
    while let Some(p) = queue.pop_front() {
        // println!("{:?}", p);
        if visited.contains(&p) {
            continue;
        } else {
            visited.insert(p.clone());
        }
        for node in expand_point(&p) {
            if node.x > bound || node.y > bound || node.z > bound {
                continue;
            }

            if all.contains(&node) {
                count += 1;
                continue;
            }

            if !visited.contains(&node) {
                queue.push_back(node);
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(64, part1("data/day18-sample.txt"));
        assert_eq!(3390, part1("data/day18.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(58, part2("data/day18-sample.txt"));
        assert_eq!(2058, part2("data/day18.txt"));
    }
}
