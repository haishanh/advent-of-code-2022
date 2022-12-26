use std::collections::HashMap;
use std::fs;
use std::iter::Peekable;
use std::str;

#[derive(Debug, Clone)]
enum MathOp {
    Add,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum MathOperand {
    Monkey(String),
    Number(i64),
}

#[derive(Debug)]
struct MathExpression {
    op: MathOp,
    left: MathOperand,
    right: MathOperand,
}

#[derive(Debug)]
enum YellEelement {
    Number(i64),
    MathExpression(MathExpression),
}

fn parse_number<I>(iter: &mut Peekable<I>) -> i64
where
    I: Iterator<Item = u8>,
{
    let mut v = Vec::new();
    while let Some(b) = iter.peek() {
        if *b <= b'9' && *b >= b'0' {
            v.push(*b);
            iter.next();
        } else {
            break;
        }
    }
    str::from_utf8(&v).unwrap().parse().unwrap()
}

fn parse_word<I>(iter: &mut Peekable<I>) -> String
where
    I: Iterator<Item = u8>,
{
    let mut v = Vec::new();
    while let Some(b) = iter.peek() {
        if (*b <= b'Z' && *b >= b'A') || (*b <= b'z' && *b >= b'a') {
            v.push(*b);
            iter.next();
        } else {
            break;
        }
    }
    str::from_utf8(&v).unwrap().into()
}

fn parse_line(line: &str) -> (String, YellEelement) {
    let bytes = line.bytes();
    let mut iter = bytes.peekable();

    let name = parse_word(&mut iter);
    iter.next();
    iter.next();
    let b = iter.peek().unwrap();
    if *b >= b'0' && *b <= b'9' {
        let n = parse_number(&mut iter);
        return (name, YellEelement::Number(n));
    }

    let left = MathOperand::Monkey(parse_word(&mut iter));
    // eat space
    iter.next();
    let b = iter.next().unwrap();
    let op = match b {
        b'+' => MathOp::Add,
        b'-' => MathOp::Minus,
        b'*' => MathOp::Multiply,
        b'/' => MathOp::Divide,
        _ => panic!("Invalid input"),
    };
    // eat space
    iter.next();
    let right = MathOperand::Monkey(parse_word(&mut iter));
    return (
        name,
        YellEelement::MathExpression(MathExpression { op, left, right }),
    );
}

fn calc(
    name: &str,
    lookup: &HashMap<String, YellEelement>,
    values: &mut HashMap<String, i64>,
    parents: &mut HashMap<String, String>,
) -> i64 {
    let e = lookup.get(name).unwrap();
    let v = match e {
        YellEelement::Number(n) => *n,
        YellEelement::MathExpression(MathExpression { op, left, right }) => {
            use MathOperand::*;
            let t = match (left, right) {
                (Monkey(a), Number(b)) => {
                    parents.insert(a.into(), name.into());
                    (calc(a, lookup, values, parents), *b)
                }
                (Monkey(a), Monkey(b)) => {
                    parents.insert(a.into(), name.into());
                    parents.insert(b.into(), name.into());
                    (
                        calc(a, lookup, values, parents),
                        calc(b, lookup, values, parents),
                    )
                }
                (Number(a), Number(b)) => (*a, *b),
                (Number(a), Monkey(b)) => {
                    parents.insert(b.into(), name.into());
                    (*a, calc(b, lookup, values, parents))
                }
            };
            match op {
                MathOp::Add => t.0 + t.1,
                MathOp::Minus => t.0 - t.1,
                MathOp::Multiply => t.0 * t.1,
                MathOp::Divide => t.0 / t.1,
            }
        }
    };
    values.insert(name.into(), v);
    v
}

pub fn solve(filepath: &str) -> (i64, i64) {
    let cnt = fs::read_to_string(filepath).unwrap();

    let mut lookup = HashMap::new();

    for line in cnt.lines() {
        let (name, element) = parse_line(line);
        lookup.insert(name, element);
    }

    let mut values = HashMap::new();
    let mut parents = HashMap::new();

    let part1_answer = calc("root", &lookup, &mut values, &mut parents);

    let mut wrong_monkeys = vec!["humn"];
    let mut start = "humn";
    while let Some(p) = parents.get(start) {
        wrong_monkeys.push(p);
        start = p;
    }

    // m should be "root"
    let mut m = wrong_monkeys.pop().unwrap();

    loop {
        if m == "humn" {
            break;
        }
        let e = lookup.get(m).unwrap();
        match e {
            YellEelement::MathExpression(MathExpression { op, left, right }) => {
                let n = wrong_monkeys.pop().unwrap();
                match (left, right) {
                    (MathOperand::Monkey(l), MathOperand::Monkey(r)) => {
                        if n == l {
                            let v = if m == "root" {
                                *values.get(r).unwrap()
                            } else {
                                let mv = values.get(m).unwrap();
                                let rv = values.get(r).unwrap();
                                match op {
                                    MathOp::Add => mv - rv,
                                    MathOp::Minus => mv + rv,
                                    MathOp::Multiply => mv / rv,
                                    MathOp::Divide => mv * rv,
                                }
                            };
                            values.insert(l.into(), v);
                        } else if n == r {
                            let v = if m == "root" {
                                *values.get(l).unwrap()
                            } else {
                                let mv = values.get(m).unwrap();
                                let lv = values.get(l).unwrap();
                                match op {
                                    MathOp::Add => mv - lv,
                                    MathOp::Minus => lv - mv,
                                    MathOp::Multiply => mv / lv,
                                    MathOp::Divide => lv / mv,
                                }
                            };
                            values.insert(r.into(), v);
                        }
                    }
                    _ => panic!("invalid input"),
                }
                m = n;
            }
            _ => panic!("expect MathExpression"),
        }
    }

    (part1_answer, *values.get("humn").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!((152, 301), solve("data/day21-sample.txt"));
        assert_eq!((324122188240430, 3412650897405), solve("data/day21.txt"));
    }
}
