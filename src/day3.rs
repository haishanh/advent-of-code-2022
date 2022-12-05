use std::collections::HashSet;
use std::fs;
use std::str;

fn calc_priority(b: &u8) -> u32 {
    if *b > b'a' {
        1 + (*b - b'a') as u32
    } else if *b > b'A' {
        27 + (*b - b'A') as u32
    } else {
        0
    }
}

fn find_common_item(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mid = len / 2;

    let mut left = HashSet::new();
    let mut right = HashSet::new();

    let mut i = 0;

    while i < mid {
        let l = bytes[i];
        let r = bytes[i + mid];

        left.insert(l);
        right.insert(r);
        i += 1;

        if let Some(b) = right.get(&l) {
            return calc_priority(b);
        } else if let Some(b) = left.get(&r) {
            return calc_priority(b);
        }
    }

    return 0;
}

fn build_set(line: &str) -> HashSet<&u8> {
    let mut h = HashSet::new();
    for item in line.as_bytes() {
        h.insert(item);
    }
    h
}

fn find_common_of_groups(line0: &str, line1: &str, line2: &str) -> u32 {
    // let h0 = build_set;
    let h1 = build_set(line1);
    let h2 = build_set(line2);

    let mut ret = 0;

    for c in line0.as_bytes() {
        if h1.contains(c) && h2.contains(c) {
            ret = calc_priority(c);
        }
    }
    ret
}

pub fn part1() -> u32 {
    let contents = fs::read_to_string("data/day3.txt").expect("expect file");
    let mut total = 0;
    for line in contents.lines() {
        let s = find_common_item(line);
        total += s;
    }
    total
}

pub fn part2() -> u32 {
    let contents = fs::read_to_string("data/day3.txt").expect("expect file");
    let mut total = 0;
    let mut lines = contents.lines();
    loop {
        if let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
            total += find_common_of_groups(a, b, c);
        } else {
            break;
        }
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(8123, part1());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2620, part2());
    }
}
