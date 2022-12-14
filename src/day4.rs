use std::fs;
use std::str;

fn parse_range(input: &str) -> (u32, u32) {
    let mut nums = input.split('-');
    let left: u32 = nums.next().unwrap().parse().unwrap();
    let right: u32 = nums.next().unwrap().parse().unwrap();
    (left, right)
}

// parse "6-7,7-9" to ((6,7), (7,9))
fn parse_row(input: &str) -> ((u32, u32), (u32, u32)) {
    let mut pairs = input.split(',');
    let p1 = parse_range(pairs.next().unwrap());
    let p2 = parse_range(pairs.next().unwrap());
    (p1, p2)
}

pub fn part1() -> u32 {
    let content = fs::read_to_string("data/day4.txt").expect("expect file");
    let lines = content.lines();
    let mut count = 0;
    for line in lines {
        let (p1, p2) = parse_row(line);
        if (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1) {
            count += 1;
        }
    }
    count
}

pub fn part2() -> u32 {
    let content = fs::read_to_string("data/day4.txt").expect("expect file");
    let lines = content.lines();
    let mut count = 0;
    for line in lines {
        let (p1, p2) = parse_row(line);
        if !(p1.1 < p2.0 || p1.0 > p2.1) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(573, part1());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(867, part2());
    }
}
