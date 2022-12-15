use std::collections::{HashMap, HashSet};
use std::{fs, str};

type Point = (i32, i32);
type Range = (i32, i32);

// this will consume an extra byte (not a digit)
// it's fine in our case tho ¯\_(ツ)_/¯
fn parse_number<I>(iter: &mut I) -> i32
where
    I: Iterator<Item = u8>,
{
    let mut digits = Vec::new();
    while let Some(b) = iter.next() {
        if b == b'-' || (b >= b'0' && b <= b'9') {
            digits.push(b);
        } else {
            break;
        }
    }
    str::from_utf8(&digits).unwrap().parse().unwrap()
}

#[inline]
fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_file(filepath: &str) -> (Vec<Point>, HashMap<Point, Point>) {
    let cnt = fs::read_to_string(filepath).unwrap();

    let mut sensors = Vec::new();
    let mut sensor_beacon = HashMap::new();

    for line in cnt.lines() {
        let mut bytes = line.bytes();
        let mut numbers = Vec::new();
        while let Some(b) = bytes.next() {
            if b == b'=' {
                numbers.push(parse_number(&mut bytes));
            }
        }
        let s = (numbers[0], numbers[1]);
        sensors.push(s);
        sensor_beacon.insert(s, (numbers[2], numbers[3]));
    }

    (sensors, sensor_beacon)
}

fn calc_sensor_cover_range_on_row(sensor: &Point, beacon: &Point, row: i32) -> Option<Range> {
    let dis = manhattan_distance(&sensor, &beacon);
    let y = (sensor.1 - row).abs();
    if y > dis {
        return None;
    }
    let diff = dis - y;
    Some((sensor.0 - diff, sensor.0 + diff))
}

fn calc_beacon_free_range_on_row(
    sensors: &Vec<Point>,
    sensor_beacon: &HashMap<Point, Point>,
    row: i32,
) -> Vec<Range> {
    sensors
        .iter()
        .map(|s| calc_sensor_cover_range_on_row(&s, &sensor_beacon.get(&s).unwrap(), row))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

pub fn part1(filepath: &str, row: i32) -> u32 {
    let (sensors, sensor_beacon) = parse_file(filepath);

    let mut ranges = calc_beacon_free_range_on_row(&sensors, &sensor_beacon, row);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut seen = HashSet::new();

    let beacon_on_row = sensors.iter().fold(0, |acc, x| {
        let b = sensor_beacon.get(&x).unwrap();
        if b.1 == row {
            if seen.contains(&b) {
                acc
            } else {
                seen.insert(b);
                acc + 1
            }
        } else {
            acc
        }
    });

    let mut prev_max = ranges[0].1;
    let mut count = ranges[0].1 - ranges[0].0 + 1;
    let mut iter = ranges.iter();
    iter.next();
    for r in iter {
        if r.0 > prev_max {
            count += r.1 - r.0 + 1;
            prev_max = r.1;
        } else if r.1 > prev_max {
            count += r.1 - prev_max;
            prev_max = r.1;
        }
    }
    count as u32 - beacon_on_row
}

#[allow(dead_code)]
fn find_bound_of_sensor(sensor: &Point, beacon: &Point) -> (Point, Point, Point, Point) {
    let distance = manhattan_distance(sensor, beacon);
    (
        (sensor.0, sensor.1 - distance),
        (sensor.0 + distance, sensor.1),
        (sensor.0, sensor.1 + distance),
        (sensor.0 - distance, sensor.1),
    )
}

//pub fn part2_v2(filepath: &str, bound: i32) -> i32 {
//    let (sensors, sensor_beacon) = parse_file(filepath);
//    let bounds: Vec<(Point, Point, Point, Point)> = sensors
//        .iter()
//        .map(|s| find_bound_of_sensor(&s, &sensor_beacon.get(&s).unwrap()))
//        .collect();
//    let mut iter = sensors.iter().zip(bounds.iter());
//    for i in iter {
//        let (sensor, frame) = i;
//        // top right edge
//        let mut start = (frame.0 .0, frame.0 .1 - 1);
//        let mut stop = (frame.1 .0 + 1, frame.1 .1);
//        loop {
//            //
//        }
//    }
//    0
//}

pub fn part2(filepath: &str, bound: i32) -> Point {
    let (sensors, sensor_beacon) = parse_file(filepath);
    // this is bruteforce...
    for y in 0..=bound {
        let mut ranges = calc_beacon_free_range_on_row(&sensors, &sensor_beacon, y);
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        // prev to represent a x-axis range (inclued both .0 and .1) where another beacon can still
        // be placed
        let mut prev = (0, bound);
        for r in ranges {
            if r.1 < 0 {
                continue;
            }
            if r.0 > bound {
                break;
            }
            if r.0 > prev.0 {
                let x = prev.0;
                return (x, y);
                // ret = 1;
                // break;
            }
            if r.1 >= prev.0 {
                prev.0 = r.1 + 1;
                if prev.0 >= bound {
                    break;
                }
            }
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(26, part1("data/day15-sample.txt", 10));
        assert_eq!(5108096, part1("data/day15.txt", 2000000));
    }

    #[test]
    fn test_part2() {
        assert_eq!((14, 11), part2("data/day15-sample.txt", 20));
        // warning: this is slow
        // assert_eq!((2638485, 2650264), part2("data/day15.txt", 4000000));
    }
}
