use std::collections::HashSet;
use std::fs;
use std::iter::Peekable;
use std::str;

#[derive(Debug)]
struct Row {
    bound: (usize, usize),
    walls: HashSet<usize>,
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

fn parse_file(filepath: &str) {
    let bytes = fs::read(filepath).unwrap();
    let mut iter = bytes.into_iter().peekable();

    let mut rows = Vec::new();

    loop {
        let mut min = usize::MAX;
        let mut max = 0usize;
        let mut idx = 0usize;
        let mut walls = HashSet::new();

        if let Some(b'\n') = iter.peek() {
            break;
        }

        while let Some(b) = iter.peek() {
            match *b {
                b' ' => {
                    iter.next();
                }
                b'.' | b'#' => {
                    if idx < min {
                        min = idx;
                    } else if idx > max {
                        max = idx;
                    }
                    if *b == b'#' {
                        walls.insert(idx);
                    }
                    iter.next();
                }
                b'\n' => {
                    rows.push(Row {
                        bound: (min, max),
                        walls,
                    });
                    iter.next();
                    break;
                }
                _ => panic!("Invalid input"),
            }
            idx += 1;
        }
    }

    iter.next();

    // right 0, down 1, left 2, up 3
    let mut state = (rows[0].bound.0, 0, 0); // (x, y, facing)
    while let Some(b) = iter.peek() {
        if *b >= b'0' && *b <= b'9' {
            let n = parse_number(&mut iter);
            // println!("n={n:?} state.2={}", state.2);
            match state.2 {
                0 => {
                    let row = &rows[state.1];
                    let mut x1 = state.0;
                    let mut count = 0;
                    while count < n {
                        if x1 == row.bound.1 {
                            x1 = row.bound.0;
                        } else {
                            x1 += 1;
                        }
                        if row.walls.contains(&x1) {
                            break;
                        } else {
                            count += 1;
                            state.0 = x1;
                        }
                    }
                }
                2 => {
                    let row = &rows[state.1];
                    let mut x1 = state.0;
                    let mut count = 0;
                    while count < n {
                        if x1 == row.bound.0 {
                            x1 = row.bound.1;
                        } else {
                            x1 -= 1;
                        }
                        if row.walls.contains(&x1) {
                            break;
                        } else {
                            count += 1;
                            state.0 = x1;
                        }
                    }
                }
                1 => {
                    let mut y1 = state.1;
                    let mut count = 0;
                    while count < n {
                        if y1 == rows.len() - 1 {
                            y1 = 0;
                        } else {
                            y1 += 1;
                        }

                        if state.0 < rows[y1].bound.0 || state.0 > rows[y1].bound.1 {
                            continue;
                        }

                        if rows[y1].walls.contains(&state.0) {
                            break;
                        } else {
                            count += 1;
                            state.1 = y1;
                        }
                    }
                }
                3 => {
                    let mut count = 0;
                    let mut y1 = state.1;
                    while count < n {
                        if y1 == 0 {
                            y1 = rows.len() - 1;
                        } else {
                            y1 -= 1;
                        }

                        if state.0 < rows[y1].bound.0 || state.0 > rows[y1].bound.1 {
                            continue;
                        }

                        if rows[y1].walls.contains(&state.0) {
                            break;
                        } else {
                            count += 1;
                            state.1 = y1;
                        }
                    }
                }
                _ => panic!("Invalid input"),
            }
        } else if *b == b'R' {
            iter.next();
            if state.2 == 3 {
                state.2 = 0;
            } else {
                state.2 += 1;
            }
        } else if *b == b'L' {
            iter.next();
            if state.2 == 0 {
                state.2 = 3;
            } else {
                state.2 -= 1;
            }
        } else {
            iter.next();
        }
    }

    let ret = (state.1 + 1) * 1000 + (state.0 + 1) * 4 + state.2;
    println!("{:?}", ret);
}

pub fn part1(filepath: &str) -> i64 {
    parse_file(filepath);
    0
}
