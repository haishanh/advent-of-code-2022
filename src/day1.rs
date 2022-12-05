use std::fs;
use std::str;

fn is_digit(b: &u8) -> bool {
    *b >= b'0' && *b <= b'9'
}

// parsing byte by byte
// but line by line would be easier
fn calc_block_max(input: &[u8], take: u32) -> u32 {
    let mut all_block_totals = Vec::new();
    let mut block_total: u32 = 0;
    let mut digits = Vec::new();
    let mut prev_char = &b'0';

    for ch in input.iter() {
        if is_digit(ch) {
            digits.push(*ch);
        } else {
            // should be a trailing space or a b'\n'
            if is_digit(prev_char) {
                let number_str = str::from_utf8(&digits).unwrap();
                let acc_line = number_str.parse::<u32>().unwrap();
                block_total += acc_line;
                // reset
                digits = Vec::new();
            } else {
                // should be on an empty line
                all_block_totals.push(block_total);
                block_total = 0;
            }
        }
        prev_char = ch;
    }

    // handle leftover
    if !digits.is_empty() {
        let number_str = str::from_utf8(&digits).unwrap();
        let acc_line = number_str.parse::<u32>().unwrap();
        block_total += acc_line;
    }

    // handle leftover
    if block_total != 0 {
        all_block_totals.push(block_total);
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
    total
}

pub fn part1(file: &str) -> u32 {
    let bytes = fs::read(file).unwrap();
    calc_block_max(&bytes, 1)
}

pub fn part2(file: &str) -> u32 {
    let bytes = fs::read(file).unwrap();
    calc_block_max(&bytes, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(24000, part1("data/day1-sample.txt"));
        assert_eq!(75622, part1("data/day1.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45000, part2("data/day1-sample.txt"));
        assert_eq!(213159, part2("data/day1.txt"));
    }
}
