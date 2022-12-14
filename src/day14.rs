use std::collections::HashSet;
use std::{fs, str};

type Point = (u32, u32);

fn parse_line(line: &str) -> Vec<Point> {
    let mut bytes = line.bytes();

    let mut digits = Vec::new();
    let mut point: Point = (0, 0);
    let mut points = Vec::new();
    loop {
        let ob = bytes.next();
        match ob {
            Some(b) => match b {
                b'0'..=b'9' => {
                    digits.push(b);
                }
                b',' => {
                    point.0 = str::from_utf8(&digits).unwrap().parse().unwrap();
                    digits.clear();
                }
                _ => {
                    if !digits.is_empty() {
                        point.1 = str::from_utf8(&digits).unwrap().parse().unwrap();
                        points.push(point.clone());
                        digits.clear();
                    }
                }
            },
            None => {
                if !digits.is_empty() {
                    point.1 = str::from_utf8(&digits).unwrap().parse().unwrap();
                    points.push(point.clone());
                    digits.clear();
                }
                break;
            }
        }
    }

    points
}

#[derive(Debug)]
enum SandState {
    Rest,
    FallingForever,
}

fn scan(filepath: &str, obstacles: &mut HashSet<Point>) -> (Point, Point) {
    let cnt = fs::read_to_string(filepath).unwrap();

    let mut bound_x = (500, 500);
    let mut bound_y = (0, 0);

    let mut prev_point = (0, 0);

    for line in cnt.lines() {
        let points = parse_line(line);
        let mut idx = 0;
        for p in points {
            obstacles.insert(p);

            if idx > 0 {
                if prev_point.0 == p.0 {
                    if prev_point.1 < p.1 {
                        for i in prev_point.1..p.1 {
                            obstacles.insert((p.0, i));
                        }
                    } else {
                        for i in p.1..prev_point.1 {
                            obstacles.insert((p.0, i));
                        }
                    }
                } else if prev_point.1 == p.1 {
                    if prev_point.0 < p.0 {
                        for i in prev_point.0..p.0 {
                            obstacles.insert((i, p.1));
                        }
                    } else {
                        for i in p.0..prev_point.0 {
                            obstacles.insert((i, p.1));
                        }
                    }
                }
            }

            prev_point = p;
            idx += 1;

            if p.0 < bound_x.0 {
                bound_x.0 = p.0;
            } else if p.0 > bound_x.1 {
                bound_x.1 = p.0;
            }
            if p.1 > bound_y.1 {
                bound_y.1 = p.1;
            }
        }
    }

    (bound_x, bound_y)
}

pub fn part1(filepath: &str) -> u32 {
    let mut obstacles: HashSet<Point> = HashSet::new();
    let (bound_x, bound_y) = scan(filepath, &mut obstacles);

    let mut count = 0;
    loop {
        let mut current = (500, 0);
        let state: SandState;
        loop {
            if current.1 == bound_y.1 {
                state = SandState::FallingForever;
                break;
            }
            // println!("{:?}", current);
            let next = (current.0, current.1 + 1);
            if !obstacles.contains(&next) {
                current = next;
                continue;
            }
            if current.0 == bound_x.0 {
                state = SandState::FallingForever;
                break;
            }
            let next = (current.0 - 1, current.1 + 1);
            if !obstacles.contains(&next) {
                current = next;
                continue;
            }
            if current.0 == bound_x.1 {
                state = SandState::FallingForever;
                break;
            }
            let next = (current.0 + 1, current.1 + 1);
            if !obstacles.contains(&next) {
                current = next;
                continue;
            }

            state = SandState::Rest;
            obstacles.insert(current);
            break;
        }

        // println!("{:?}", state);
        match state {
            SandState::FallingForever => {
                break;
            }
            SandState::Rest => {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(filepath: &str) -> u32 {
    let mut obstacles: HashSet<Point> = HashSet::new();
    let (_, mut bound_y) = scan(filepath, &mut obstacles);

    bound_y.1 += 2;

    // a stack to record "holes"
    // instead of start from (500, 0) every time, we can just pop() a point
    // from this holes stack and start from there
    // this 28X times faster compare with part2_v1 (without this technique)
    // $ cargo bench day14
    let mut holes = Vec::new();
    holes.push((500, 0));

    let mut count = 0;
    loop {
        let mut current = holes.pop().map_or((500, 0), |f| f);
        let state: SandState;
        loop {
            if obstacles.contains(&current) {
                current = holes.pop().map_or((500, 0), |f| f);
            }

            let mb = (current.0, current.1 + 1);
            let lb = (current.0 - 1, current.1 + 1);
            let rb = (current.0 + 1, current.1 + 1);

            if mb.1 == bound_y.1 {
                state = SandState::Rest;
                obstacles.insert(current);
                break;
            } else if !obstacles.contains(&mb) {
                holes.push(current);
                current = mb;
                continue;
            } else if !obstacles.contains(&lb) {
                holes.push(current);
                current = lb;
                continue;
            } else if !obstacles.contains(&rb) {
                holes.push(current);
                current = rb;
                continue;
            } else {
                state = SandState::Rest;
                obstacles.insert(current);
                break;
            }
        }

        match state {
            SandState::Rest => {
                count += 1;
            }
            SandState::FallingForever => {}
        }
        if obstacles.contains(&(500, 0)) {
            break;
        }
    }
    count
}

#[allow(dead_code)]
pub fn part2_v1(filepath: &str) -> u32 {
    let mut obstacles: HashSet<Point> = HashSet::new();
    let (_, mut bound_y) = scan(filepath, &mut obstacles);

    bound_y.1 += 2;

    let mut count = 0;
    loop {
        let mut current = (500, 0);
        let state: SandState;
        loop {
            if obstacles.contains(&current) {
                current = (500, 0);
            }

            let mb = (current.0, current.1 + 1);
            let lb = (current.0 - 1, current.1 + 1);
            let rb = (current.0 + 1, current.1 + 1);

            if mb.1 == bound_y.1 {
                state = SandState::Rest;
                obstacles.insert(current);
                break;
            } else if !obstacles.contains(&mb) {
                current = mb;
                continue;
            } else if !obstacles.contains(&lb) {
                current = lb;
                continue;
            } else if !obstacles.contains(&rb) {
                current = rb;
                continue;
            } else {
                state = SandState::Rest;
                obstacles.insert(current);
                break;
            }
        }

        match state {
            SandState::Rest => {
                count += 1;
            }
            SandState::FallingForever => {}
        }
        if obstacles.contains(&(500, 0)) {
            break;
        }
    }
    count
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(24, part1("data/day14-sample.txt"));
        assert_eq!(825, part1("data/day14.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(93, part2("data/day14-sample.txt"));
        assert_eq!(26729, part2("data/day14.txt"));
    }

    #[test]
    fn test_part2_v1() {
        assert_eq!(93, part2_v1("data/day14-sample.txt"));
        assert_eq!(26729, part2_v1("data/day14.txt"));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2("data/day14.txt"));
    }

    #[bench]
    fn bench_part2_v1(b: &mut Bencher) {
        b.iter(|| part2_v1("data/day14.txt"));
    }
}
