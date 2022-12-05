use std::fs;
use std::str;

pub fn part1() {
    let content = fs::read_to_string("data/day4.txt").expect("expect file");
    let lines = content.lines();
    let mut count = 0;
    for line in lines {
        let (p1, p2) = parse_row(line);
        if (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1) {
            count += 1;
        }
    }
    println!("count={}", count);
}
