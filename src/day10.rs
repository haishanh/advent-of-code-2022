use std::fs;
use std::str;

pub fn part1(filepath: &str) -> i32 {
    let cnt = fs::read_to_string(filepath).expect("expect file");

    let mut strength = Vec::new();
    let mut cycle = 0;
    let mut register = 1;

    let mut register_prev = 1;

    // the 1st circle is the one between circle 0 to circle 1

    // circles to evalute
    let mut check_points = vec![220, 180, 140, 100, 60, 20];

    for line in cnt.lines() {
        let mut pairs = line.split(' ');
        let instruction = pairs.next().unwrap();
        match instruction {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                let value: i32 = pairs.next().unwrap().parse().unwrap();
                register += value;
                cycle += 2;
            }
            _ => panic!("invalid input"),
        }

        if let Some(v) = check_points.pop() {
            if cycle >= v {
                strength.push(v * register_prev);
            } else {
                check_points.push(v);
            }
        } else {
            break;
        }

        register_prev = register;
    }
    // println!("{:?}", strength);
    let x: i32 = strength.iter().sum();
    x
}

fn push_pixel(row: &mut Vec<u8>, pos: i32, register: i32) {
    if pos >= register - 1 && pos <= register + 1 {
        row.push(b'#');
    } else {
        row.push(b'.');
    }
}

fn render_part2<'a, I>(lines: I, grid: &mut Vec<Vec<u8>>)
where
    I: Iterator<Item = &'a str>,
{
    let mut row = Vec::new();
    let mut register = 1;
    let mut pos = 0;

    for line in lines {
        let mut pairs = line.split(' ');
        let instruction = pairs.next().unwrap();
        match instruction {
            "noop" => {
                push_pixel(&mut row, pos, register);
                pos += 1;
                if pos == 40 {
                    pos = 0;
                    grid.push(row.clone());
                    row.clear();
                }
            }
            "addx" => {
                push_pixel(&mut row, pos, register);
                pos += 1;
                if pos == 40 {
                    pos = 0;
                    grid.push(row.clone());
                    row.clear();
                }
                push_pixel(&mut row, pos, register);
                pos += 1;
                if pos == 40 {
                    pos = 0;
                    grid.push(row.clone());
                    row.clear();
                }

                let value: i32 = pairs.next().unwrap().parse().unwrap();
                register += value;
            }
            _ => panic!("invalid input"),
        }
    }
}

pub fn part2(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).expect("expect file");

    let mut grid = Vec::new();

    render_part2(cnt.lines(), &mut grid);

    for r in grid {
        let s = str::from_utf8(&r).unwrap();
        println!("{}", s);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13140, part1("data/day10-sample.txt"));
        assert_eq!(13720, part1("data/day10.txt"));
    }

    #[test]
    fn test_part2_sample() {
        let cnt = fs::read_to_string("data/day10-sample.txt").expect("expect file");
        let mut grid = Vec::new();
        render_part2(cnt.lines(), &mut grid);
        assert_eq!(
            vec![
                "##..##..##..##..##..##..##..##..##..##..".as_bytes(),
                "###...###...###...###...###...###...###.".as_bytes(),
                "####....####....####....####....####....".as_bytes(),
                "#####.....#####.....#####.....#####.....".as_bytes(),
                "######......######......######......####".as_bytes(),
                "#######.......#######.......#######.....".as_bytes(),
            ],
            grid
        );
    }

    #[test]
    fn test_part2() {
        let cnt = fs::read_to_string("data/day10.txt").expect("expect file");
        let mut grid = Vec::new();
        render_part2(cnt.lines(), &mut grid);
        assert_eq!(
            vec![
                "####.###..#..#.###..#..#.####..##..#..#.".as_bytes(),
                "#....#..#.#..#.#..#.#..#....#.#..#.#..#.".as_bytes(),
                "###..###..#..#.#..#.####...#..#....####.".as_bytes(),
                "#....#..#.#..#.###..#..#..#...#....#..#.".as_bytes(),
                "#....#..#.#..#.#.#..#..#.#....#..#.#..#.".as_bytes(),
                "#....###...##..#..#.#..#.####..##..#..#.".as_bytes(),
            ],
            grid
        );
    }
}
