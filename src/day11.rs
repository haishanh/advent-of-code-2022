use std::cell::RefCell;
use std::fs;
use std::iter::Peekable;
use std::str;

#[derive(Debug)]
enum MonkeyOperator {
    Multiply,
    Add,
}

#[derive(Debug)]
enum MonkeyOperand {
    Old,
    Number(u64),
}

#[derive(Debug)]
struct MonkeyState {
    starts: RefCell<Vec<u64>>,
    operation: (MonkeyOperator, MonkeyOperand),
    test: (u64, u64, u64),
    inspected_count: RefCell<u64>,
}

#[derive(Debug, PartialEq, Eq)]
struct SimpleBigNumber {
    value: Vec<u32>,
}

const BASE: u32 = 1000;

/// not necessary!
impl SimpleBigNumber {
    fn new(x: u32) -> Self {
        let mut value = Vec::new();
        let mut dividend = x;
        loop {
            let reminder = dividend % BASE;
            dividend = dividend / BASE;
            value.push(reminder);
            if dividend == 0 {
                break;
            }
        }
        SimpleBigNumber { value }
    }

    fn add(&self, rhs: &SimpleBigNumber) -> SimpleBigNumber {
        let mut value = Vec::new();
        let mut i = self.value.iter();
        let mut j = rhs.value.iter();

        let mut overflow = 0;

        loop {
            let iv = i.next();
            let jv = j.next();
            match (iv, jv) {
                (Some(a), Some(b)) => {
                    let ret = *a + *b + overflow;
                    overflow = ret / BASE;
                    value.push(ret % BASE);
                }
                (Some(a), None) => {
                    let ret = *a + overflow;
                    overflow = ret / BASE;
                    value.push(ret % BASE);
                }
                (None, Some(b)) => {
                    let ret = *b + overflow;
                    overflow = ret / BASE;
                    value.push(ret % BASE);
                }
                (None, None) => {
                    if overflow > 0 {
                        value.push(overflow);
                    }
                    break;
                }
            }
        }

        SimpleBigNumber { value }
    }

    fn mul(&self, rhs: &SimpleBigNumber) -> SimpleBigNumber {
        let mut values = Vec::new();
        let mut count = 0;
        for a in self.value.iter() {
            let mut overflow = 0;
            let mut value = Vec::new();
            for _i in 0..count {
                value.push(0);
            }
            for b in rhs.value.iter() {
                let ret = *a * *b + overflow;
                overflow = ret / BASE;
                value.push(ret % BASE);
            }
            if overflow > 0 {
                value.push(overflow);
            }
            values.push(SimpleBigNumber { value });
            count += 1;
        }

        let mut ret = SimpleBigNumber { value: vec![0] };
        for i in values.iter() {
            // println!("{:?}", i);
            ret = ret.add(i);
        }
        ret
    }

    fn rem(&self, rhs: u32) -> u32 {
        let mut value = self.value.clone();

        loop {
            if let Some(v) = value.pop() {
                let r = v % rhs;
                if let Some(u) = value.pop() {
                    value.push(r * BASE + u);
                } else {
                    return r;
                }
            } else {
                panic!("should not reach here");
            }
        }
    }
}

fn parse_operation<'a, I>(iter: &mut Peekable<I>) -> (MonkeyOperator, MonkeyOperand)
where
    I: Iterator<Item = &'a u8>,
{
    let mut ret = (MonkeyOperator::Multiply, MonkeyOperand::Old);
    while let Some(b) = iter.next() {
        if *b == b'*' {
            break;
        } else if *b == b'+' {
            ret.0 = MonkeyOperator::Add;
            break;
        }
    }
    // skip space
    iter.next();
    let op = iter.peek().unwrap();
    if op == &&b'o' {
        for b in iter {
            if *b == b'\n' {
                break;
            }
        }

        return ret;
    }

    let mut digits = Vec::new();
    for b in iter {
        if *b >= b'0' && *b <= b'9' {
            digits.push(*b);
        } else if *b == b'\n' {
            break;
        }
    }
    ret.1 = MonkeyOperand::Number(str::from_utf8(&digits).unwrap().parse().unwrap());
    ret
}

fn parse_starting<'a, I>(iter: &mut Peekable<I>) -> Vec<u64>
where
    I: Iterator<Item = &'a u8>,
{
    let mut starts = Vec::new();
    let mut digits = Vec::new();
    while let Some(b) = iter.peek() {
        if b >= &&b'0' && b <= &&b'9' {
            break;
        } else {
            iter.next();
        }
    }
    for b in iter {
        if *b >= b'0' && *b <= b'9' {
            digits.push(*b);
        } else {
            if digits.len() > 0 {
                let v: u64 = str::from_utf8(&digits).unwrap().parse().unwrap();
                starts.push(v);
                digits.clear();
            }
            if *b == b'\n' {
                return starts;
            }
        }
    }
    starts
}

fn parse_number_in_line<'a, I>(iter: &mut Peekable<I>) -> u64
where
    I: Iterator<Item = &'a u8>,
{
    let mut digits = Vec::new();
    for b in iter {
        if *b == b'\n' {
            break;
        } else if *b >= b'0' && *b <= b'9' {
            digits.push(*b);
        }
    }
    str::from_utf8(&digits).unwrap().parse().unwrap()
}

fn parse<'a, I>(iter: &mut Peekable<I>) -> Vec<MonkeyState>
where
    I: Iterator<Item = &'a u8>,
{
    let mut monkeys = Vec::new();
    loop {
        if let Some(b) = iter.peek() {
            if b == &&b'M' {
                parse_number_in_line(iter);
                let starts = parse_starting(iter);
                let operation = parse_operation(iter);
                let t0 = parse_number_in_line(iter);
                let t1 = parse_number_in_line(iter);
                let t2 = parse_number_in_line(iter);
                monkeys.push(MonkeyState {
                    starts: RefCell::new(starts),
                    operation,
                    test: (t0, t1, t2),
                    inspected_count: RefCell::new(0),
                })
            } else {
                iter.next();
            }
        } else {
            break;
        }
    }
    monkeys
}

fn play(monkeys: &mut Vec<MonkeyState>, divisor: u64) {
    let iter = monkeys.iter();
    for m in iter {
        let starts = m.starts.take();
        for level in starts {
            *m.inspected_count.borrow_mut() += 1;
            let mut next_level = match m.operation {
                (MonkeyOperator::Add, MonkeyOperand::Old) => level + level,
                (MonkeyOperator::Add, MonkeyOperand::Number(n)) => level + n,
                (MonkeyOperator::Multiply, MonkeyOperand::Old) => level * level,
                (MonkeyOperator::Multiply, MonkeyOperand::Number(n)) => level * n,
            };
            next_level = next_level % divisor;
            let reminder = next_level % m.test.0;
            if reminder == 0 {
                monkeys[m.test.1 as usize]
                    .starts
                    .borrow_mut()
                    .push(next_level);
            } else {
                monkeys[m.test.2 as usize]
                    .starts
                    .borrow_mut()
                    .push(next_level);
            }
        }
    }
}

pub fn part1(filepath: &str) -> u64 {
    let bytes = fs::read(filepath).unwrap();
    let mut iter = bytes.iter().peekable();
    let mut monkeys = parse(&mut iter);

    for _i in 0..20 {
        play(&mut monkeys, 3);
    }
    // viz(&monkeys);
    let mut x: Vec<u64> = monkeys.iter().map(|x| x.inspected_count.take()).collect();
    x.sort_by(|a, b| b.cmp(a));
    x[0] * x[1]
}

#[allow(dead_code)]
fn viz(monkeys: &Vec<MonkeyState>) {
    for m in monkeys {
        println!("{:?}", m.starts.borrow());
    }
}

pub fn part2(filepath: &str) -> u64 {
    let bytes = fs::read(filepath).unwrap();
    let mut iter = bytes.iter().peekable();
    let mut monkeys = parse(&mut iter);

    let mut x = 1;
    for i in monkeys.iter() {
        x = x * i.test.0
    }

    for _i in 0..10000 {
        play(&mut monkeys, x);
    }
    // viz(&monkeys);
    let mut x: Vec<u64> = monkeys.iter().map(|x| x.inspected_count.take()).collect();
    x.sort_by(|a, b| b.cmp(a));
    x[0] * x[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(2713310158, part2("data/day11-sample.txt"));
        assert_eq!(25272176808, part2("data/day11.txt"));
    }
}
