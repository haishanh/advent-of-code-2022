use std::fs;
use std::str;

fn is_digit(b: &u8) -> bool {
    *b >= b'0' && *b <= b'9'
}

fn calc_block_max(input: &[u8], take: u32) {
    let mut all_block_totals = Vec::new();
    let mut block_total: u32 = 0;
    let mut digits = Vec::new();
    let mut prev_char = &b'0';

    for ch in input.iter() {
        if is_digit(ch) {
            digits.push(*ch);
        } else {
            if is_digit(prev_char) {
                let number_str = str::from_utf8(&digits).unwrap();
                let acc_line = number_str.parse::<u32>().unwrap();
                block_total += acc_line;
                // reset
                digits = Vec::new();
            } else {
                all_block_totals.push(block_total);
                block_total = 0;
            }
        }
        prev_char = ch;
    }
    all_block_totals.sort_unstable();
    all_block_totals.reverse();

    let mut count = 0;
    let mut total = 0;
    for num in all_block_totals.iter() {
        total += num;
        count += 1;
        if count >= take {
            break;
        }
    }
    println!("total={}", total);
}

pub fn part1() {
    let binding = fs::read("input.txt").unwrap();
    calc_block_max(&binding, 1);
}

pub fn part2() {
    let binding = fs::read("input.txt").unwrap();
    calc_block_max(&binding, 3);
}
