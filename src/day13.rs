use std::cmp::Ordering;
use std::fs;
use std::str;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Node {
    Number(Box<u32>),
    List(Box<Vec<Node>>),
    Empty,
}

enum CompareResult {
    Gt,
    Lt,
    Eq,
}

fn parse_line(line: &str) -> Vec<Node> {
    let mut vec_stack: Vec<Vec<Node>> = Vec::new();
    let mut bytes = line.bytes();

    let mut digits = Vec::new();
    let mut prev_element = Node::Empty;
    while let Some(b) = bytes.next() {
        match b {
            b'[' => {
                vec_stack.push(Vec::new());
            }
            b']' => {
                if !digits.is_empty() {
                    let n: u32 = str::from_utf8(&digits).unwrap().parse().unwrap();
                    digits.clear();
                    prev_element = Node::Number(Box::new(n));
                }
                let mut current = vec_stack.pop().unwrap();
                current.push(prev_element);
                if vec_stack.is_empty() {
                    return current;
                }
                prev_element = Node::List(Box::new(current));
            }
            b',' => {
                if !digits.is_empty() {
                    let n: u32 = str::from_utf8(&digits).unwrap().parse().unwrap();
                    digits.clear();
                    prev_element = Node::Number(Box::new(n));
                }
                let mut current = vec_stack.pop().unwrap();
                current.push(prev_element);
                prev_element = Node::Empty;
                vec_stack.push(current);
            }
            _ => digits.push(b),
        }
    }
    Vec::new()
}

fn compare_vectors(a: &Vec<Node>, b: &Vec<Node>) -> CompareResult {
    let mut ai = a.iter();
    let mut bi = b.iter();
    loop {
        let (ax, bx) = (ai.next(), bi.next());
        match (ax, bx) {
            (None, None) => {
                return CompareResult::Eq;
            }
            (None, Some(_)) => {
                return CompareResult::Lt;
            }
            (Some(_), None) => {
                return CompareResult::Gt;
            }
            (Some(l), Some(r)) => match (l, r) {
                (Node::Number(lb), Node::Number(rb)) => {
                    if **lb < **rb {
                        return CompareResult::Lt;
                    } else if **lb > **rb {
                        return CompareResult::Gt;
                    }
                }
                (Node::Empty, Node::Empty) => {}
                (Node::Empty, Node::Number(_) | Node::List(_)) => {
                    return CompareResult::Lt;
                }
                (Node::Number(_) | Node::List(_), Node::Empty) => {
                    return CompareResult::Gt;
                }
                (Node::Number(lb), Node::List(rb)) => {
                    let x = vec![Node::Number(Box::new(**lb))];
                    let result = compare_vectors(&x, &*rb);
                    match result {
                        CompareResult::Gt => {
                            return CompareResult::Gt;
                        }
                        CompareResult::Lt => {
                            return CompareResult::Lt;
                        }
                        CompareResult::Eq => {}
                    }
                }
                (Node::List(lb), Node::Number(rb)) => {
                    let x = vec![Node::Number(Box::new(**rb))];
                    let result = compare_vectors(&*lb, &x);
                    match result {
                        CompareResult::Gt => {
                            return CompareResult::Gt;
                        }
                        CompareResult::Lt => {
                            return CompareResult::Lt;
                        }
                        CompareResult::Eq => {}
                    }
                }
                (Node::List(lb), Node::List(rb)) => {
                    let result = compare_vectors(&*lb, &*rb);
                    match result {
                        CompareResult::Gt => {
                            return CompareResult::Gt;
                        }
                        CompareResult::Lt => {
                            return CompareResult::Lt;
                        }
                        CompareResult::Eq => {}
                    }
                }
            },
        }
    }
}

fn parse_file(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).unwrap();

    let mut buf = Vec::new();
    let mut count = 0;
    let mut pair_idx = 0;
    let mut pair_in_order = Vec::new();
    for line in cnt.lines() {
        if count == 2 {
            count = 0;
            continue;
        }
        let v = parse_line(line);
        if count == 0 {
            buf.push(v);
        } else {
            pair_idx += 1;
            let a = buf.pop().unwrap();
            match compare_vectors(&a, &v) {
                CompareResult::Lt => {
                    pair_in_order.push(pair_idx);
                }
                _ => {}
            }
        }
        count += 1;
    }

    pair_in_order.iter().sum()
}

pub fn part1(filepath: &str) -> u32 {
    parse_file(filepath)
}

struct Packet {
    inner: Vec<Node>,
    is_divider: bool,
}

fn read_to_sorted_packets(filepath: &str) -> Vec<Packet> {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut buf = Vec::new();
    let mut count = 0;
    for line in cnt.lines() {
        if count == 2 {
            count = 0;
            continue;
        }
        let v = parse_line(line);
        buf.push(Packet {
            inner: v,
            is_divider: false,
        });
        count += 1;
    }

    let two = Node::Number(Box::new(2));
    let wrapped_two = vec![Node::List(Box::new(vec![two]))];
    buf.push(Packet {
        inner: wrapped_two,
        is_divider: true,
    });
    let six = Node::Number(Box::new(6));
    let wrapped_six = vec![Node::List(Box::new(vec![six]))];
    buf.push(Packet {
        inner: wrapped_six,
        is_divider: true,
    });

    buf.sort_by(|a, b| {
        let ret = compare_vectors(&a.inner, &b.inner);
        match ret {
            CompareResult::Gt => Ordering::Greater,
            CompareResult::Lt => Ordering::Less,
            CompareResult::Eq => Ordering::Equal,
        }
    });
    buf
}

fn find_divier(a: &Vec<Packet>) -> Vec<u32> {
    let mut idx = 1;
    let mut divider_idx = Vec::new();
    for item in a {
        if item.is_divider {
            divider_idx.push(idx);
        }
        idx += 1;
    }
    divider_idx
}

pub fn part2(filepath: &str) -> u32 {
    let buf = read_to_sorted_packets(filepath);
    let divider_idx = find_divier(&buf);
    divider_idx.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            vec![
                Node::List(Box::new(vec![Node::Number(Box::new(1))])),
                Node::List(Box::new(vec![Node::List(Box::new(vec![Node::Empty])),])),
                Node::Number(Box::new(0))
            ],
            parse_line("[[1],[[]],0]")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(13, part1("data/day13-sample.txt"));
        assert_eq!(6428, part1("data/day13.txt"));
    }

    #[test]
    fn test_part2() {
        let buf = read_to_sorted_packets("data/day13-sample.txt");
        let divider_idx = find_divier(&buf);
        assert_eq!([10, 14], &divider_idx[..]);
        let buf = read_to_sorted_packets("data/day13.txt");
        let divider_idx = find_divier(&buf);
        assert_eq!([117, 192], &divider_idx[..]);
    }
}
