use std::collections::HashSet;
use std::fs;
use std::str;

extern crate test;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn move_tail(head: &Position, tail: &mut Position) {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff == 2 {
        tail.x += 1;
        tail.y += calc_move(y_diff);
    } else if x_diff == -2 {
        tail.x += -1;
        tail.y += calc_move(y_diff);
    } else if y_diff == 2 {
        tail.y += 1;
        tail.x += calc_move(x_diff);
    } else if y_diff == -2 {
        tail.y += -1;
        tail.x += calc_move(x_diff);
    }
}

#[inline]
fn calc_move(diff: i32) -> i32 {
    if diff >= 1 {
        1
    } else if diff <= -1 {
        -1
    } else {
        0
    }
}

pub fn part1(filepath: &str) -> u32 {
    let mut visited = HashSet::new();
    let mut head_pos = Position { x: 0, y: 0 };
    let mut tail_pos = Position { x: 0, y: 0 };
    visited.insert(tail_pos.clone());

    let cnt = fs::read_to_string(filepath).expect("expect file");
    for line in cnt.lines() {
        let mut pairs = line.split(' ');
        let dir = pairs.next().unwrap();
        let steps: u32 = pairs.next().unwrap().parse().unwrap();
        match dir {
            "U" => {
                for _i in 0..steps {
                    head_pos.y += 1;
                    move_tail(&head_pos, &mut tail_pos);
                    visited.insert(tail_pos.clone());
                }
            }
            "D" => {
                for _i in 0..steps {
                    head_pos.y -= 1;
                    move_tail(&head_pos, &mut tail_pos);
                    visited.insert(tail_pos.clone());
                }
            }
            "L" => {
                for _i in 0..steps {
                    head_pos.x -= 1;
                    move_tail(&head_pos, &mut tail_pos);
                    visited.insert(tail_pos.clone());
                }
            }
            "R" => {
                for _i in 0..steps {
                    head_pos.x += 1;
                    move_tail(&head_pos, &mut tail_pos);
                    visited.insert(tail_pos.clone());
                }
            }
            _ => {}
        }
    }
    visited.len().try_into().unwrap()
}

fn move_tails(tails: &mut [Position]) {
    for i in 0..9 {
        let head = tails.get(i).unwrap().clone();
        let tail = tails.get(i + 1).unwrap().clone();
        let tail_mut = tails.get_mut(i + 1).unwrap();
        move_tail(&head, tail_mut);
        // if tail is not changed afer move_tail
        // we can break out the loop to skip calculation for nodes after this one
        if *tail_mut == tail {
            break;
        }
    }
}

pub fn part2(filepath: &str) -> u32 {
    let mut visited = HashSet::new();
    let mut tails = Vec::new();
    for _i in 0..10 {
        tails.push(Position { x: 0, y: 0 });
    }
    visited.insert(tails[9].clone());

    let cnt = fs::read_to_string(filepath).expect("expect file");
    for line in cnt.lines() {
        let mut pairs = line.split(' ');
        let dir = pairs.next().unwrap();
        let steps: u32 = pairs.next().unwrap().parse().unwrap();
        match dir {
            "U" => {
                for _i in 0..steps {
                    tails[0].y += 1;
                    move_tails(&mut tails);
                    let last = tails[9].clone();
                    visited.insert(last);
                }
            }
            "D" => {
                for _i in 0..steps {
                    tails[0].y -= 1;
                    move_tails(&mut tails);
                    let last = tails[9].clone();
                    visited.insert(last);
                }
            }
            "L" => {
                for _i in 0..steps {
                    tails[0].x -= 1;
                    move_tails(&mut tails);
                    let last = tails[9].clone();
                    visited.insert(last);
                }
            }
            "R" => {
                for _i in 0..steps {
                    tails[0].x += 1;
                    move_tails(&mut tails);
                    let last = tails[9].clone();
                    visited.insert(last);
                }
            }
            _ => {
                panic!("invalid input, encounter dir other than U/D/L/R")
            }
        }
    }

    // uncomment this line to print the viz
    // viz(&visited);

    visited.len().try_into().unwrap()
}

#[allow(dead_code)]
fn viz(visited: &HashSet<Position>) {
    let iter = visited.iter();
    let mut x_bound = (-1, 1);
    let mut y_bound = (-1, 1);
    for i in iter {
        if i.x < x_bound.0 {
            x_bound.0 = i.x;
        } else if i.x > x_bound.1 {
            x_bound.1 = i.x;
        }
        if i.y < y_bound.0 {
            y_bound.0 = i.y;
        } else if i.y > y_bound.1 {
            y_bound.1 = i.y;
        }
    }

    let mut grid = Vec::new();
    let mut row = Vec::new();

    for _x in x_bound.0..=x_bound.1 {
        row.push(b'.');
    }

    for _y in y_bound.0..=y_bound.1 {
        grid.push(row.clone())
    }

    for i in visited.iter() {
        let row = y_bound.1 - i.y;
        let col = i.x - x_bound.0;
        grid[row as usize][col as usize] = b'#';
    }

    for r in grid {
        let line = &r[..];
        let s = str::from_utf8(line).unwrap();
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(13, part1("data/day9-sample.txt"));
        assert_eq!(6464, part1("data/day9.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, part2("data/day9-sample2.txt"));
        assert_eq!(2604, part2("data/day9.txt"));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2("data/day9.txt"));
    }
}
