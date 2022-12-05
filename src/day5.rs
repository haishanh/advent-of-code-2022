use std::fs;
use std::str;

///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///
///  should become
///
///  [[b'Z', b'N'], [b'M', b'C', b'D'], [b'P']]
fn parse_drawing(lines: &mut Vec<&str>) -> Vec<Vec<u8>> {
    let mut chart = Vec::new();

    let bytes = lines.pop().unwrap().as_bytes();
    for b in bytes {
        if *b <= b'Z' && *b >= b'A' {
            chart.push(vec![*b]);
        }
    }

    // looping backward
    loop {
        let mut count = 0;
        if let Some(line) = lines.pop() {
            let bytes = line.as_bytes();
            for b in bytes {
                if *b <= b'Z' && *b >= b'A' {
                    // len of "[A] " is 4
                    chart[count / 4].push(*b);
                }
                count += 1;
            }
        } else {
            break;
        }
    }
    chart
}

// parse "move 1 from 2 to 3" into vector [1, 2, 3]
fn parse_instruction(input: &str) -> Vec<usize> {
    let bytes = input.as_bytes();
    let mut nums = Vec::new();
    let mut result = Vec::new();
    for b in bytes {
        if *b >= b'0' && *b <= b'9' {
            nums.push(*b);
        } else if !nums.is_empty() {
            let ret: usize = str::from_utf8(&nums[..]).unwrap().parse().unwrap();
            nums.clear();
            result.push(ret);
        }
    }

    if !nums.is_empty() {
        let ret: usize = str::from_utf8(&nums[..]).unwrap().parse().unwrap();
        nums.clear();
        result.push(ret);
    }
    result
}

fn run_instruction_part1(chart: &mut Vec<Vec<u8>>, instruction: &Vec<usize>) {
    let mut count = instruction[0];
    // the pos in instruction is 1-based
    let from = instruction[1] - 1;
    let to = instruction[2] - 1;

    while count > 0 {
        if let Some(b) = chart[from].pop() {
            chart[to].push(b);
        }
        count -= 1;
    }
}

fn run_instruction_part2(chart: &mut Vec<Vec<u8>>, instruction: &Vec<usize>) {
    let count = instruction[0];
    let from = instruction[1] - 1;
    let to = instruction[2] - 1;
    let from_len = chart[from].len();

    let mut u: Vec<u8> = chart[from].drain(from_len - count..).collect();
    chart[to].append(&mut u);
}

fn process(run_instruction: fn(&mut Vec<Vec<u8>>, &Vec<usize>)) -> String {
    let content = fs::read_to_string("data/day5.txt").expect("expect file");
    let lines = content.lines();

    let mut lines_iter = lines.into_iter();

    let mut chart_lines = Vec::new();
    loop {
        if let Some(line) = lines_iter.next() {
            if !line.starts_with(" 1") {
                chart_lines.push(line);
            } else {
                break;
            }
        }
    }

    let mut chart = parse_drawing(&mut chart_lines);

    for line in lines_iter {
        if line.starts_with("move") {
            let ins = parse_instruction(line);
            run_instruction(&mut chart, &ins);
            // run_instruction_part2(&mut chart, &ins);
        }
    }

    let mut r = Vec::new();
    for mut v in chart {
        let x = v.pop().unwrap();
        r.push(x);
    }
    str::from_utf8(&r).unwrap().into()
}

pub fn part1() -> String {
    process(run_instruction_part1)
}

pub fn part2() -> String {
    process(run_instruction_part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = part1();
        assert_eq!("JDTMRWCQJ", input);
    }

    #[test]
    fn test_part_two() {
        let input = part2();
        assert_eq!("VHJDDCWRD", input);
    }
}
