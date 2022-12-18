use std::{collections::HashMap, fs, str};

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

fn rock_only_move(rock: &mut Vec<u8>, dir: &Direction) {
    let left_edge = 0b1_000_000;
    let right_edge = 0b1;

    match dir {
        Direction::Left => {
            let x = rock
                .iter()
                .filter(|&&x| x & left_edge == left_edge)
                .collect::<Vec<_>>();
            if x.len() == 0 {
                rock.into_iter().for_each(|x| {
                    *x = *x << 1;
                });
            }
        }
        Direction::Right => {
            let x = rock
                .iter()
                .filter(|&&x| x & right_edge == right_edge)
                .collect::<Vec<_>>();
            if x.len() == 0 {
                rock.into_iter().for_each(|x| {
                    *x = *x >> 1;
                });
            }
        }
        Direction::Down => rock.push(0),
    }
}

fn simulate<'a, BI, RI>(
    chamber: &mut Vec<u8>,
    bytes: &mut BI,
    rocks: &mut RI,
    rock_generation: u64,
) -> u64
where
    BI: Iterator<Item = &'a Direction>,
    RI: Iterator<Item = Vec<u8>>,
{
    let mut pattern_map = HashMap::new();

    let mut skipped = 0;

    let mut step_to_rest = 0;
    let mut rock = rocks.next().unwrap().clone();
    for _i in 0..rock.len() {
        chamber.push(0);
    }
    let mut count = 1;
    loop {
        let dir = bytes.next().unwrap();

        step_to_rest += 1;
        if step_to_rest < 4 {
            rock_only_move(&mut rock, dir);
            continue;
        }

        let mut rock0 = rock.clone();
        rock_only_move(&mut rock0, dir);

        let rock_iter = rock0.iter().rev();
        let chamber_iter = chamber.into_iter().rev();
        let zipped = rock_iter.zip(chamber_iter);

        let mut collision0 = 0;
        for (r, c) in zipped {
            if *r & *c > 0 {
                collision0 = 1;
                break;
            }
        }

        let mut rock1 = if collision0 == 1 {
            rock.clone()
        } else {
            rock0.clone()
        };
        rock_only_move(&mut rock1, &Direction::Down);

        let rock_iter = rock1.iter().rev();
        let chamber_iter = chamber.into_iter().rev();
        let zipped = rock_iter.zip(chamber_iter);

        let mut collision1 = 0;
        for (r, c) in zipped {
            if *r & *c > 0 {
                collision1 = 1;
                break;
            }
        }

        // println!("{} {} {:?}", collision0, collision1, rock);

        match (collision0, collision1) {
            (1, 1) | (0, 1) => {
                let rock_iter = if collision0 == 0 {
                    rock0.iter().rev()
                } else {
                    rock.iter().rev()
                };

                let chamber_iter = chamber.into_iter().rev();
                let zipped = rock_iter.zip(chamber_iter);
                for (r, c) in zipped {
                    *c = *c | *r;
                }
                step_to_rest = 0;
                remove_all_floating_zeros(chamber);

                if count % 5 == 0 {
                    let chamber_iter = chamber.iter().rev();
                    // let mut c = 0;
                    let mut pattern = Vec::new();
                    for b in chamber_iter {
                        pattern.push(*b);
                        if pattern.len() == 20 {
                            break;
                        }
                    }

                    if let Some((prev_count, prev_height)) = pattern_map.get(&pattern) {
                        let current = (count, chamber.len());

                        let repeat_count = current.0 - *prev_count;
                        let repeat_height = current.1 - *prev_height;

                        let remain_count = rock_generation - count;

                        let repeat_cycles: u64 = remain_count / repeat_count;
                        // let repeat_remain = remain_count - (repeat_cycles * repeat_count);

                        skipped = (repeat_height as u64) * repeat_cycles;
                        count += repeat_cycles * repeat_count;

                        pattern_map.clear();
                    } else {
                        pattern_map.insert(pattern, (count, chamber.len()));
                    }
                }

                count += 1;

                // print_chamber(chamber);
                // println!("\t---");

                if count == rock_generation {
                    break;
                }
                rock = rocks.next().unwrap().clone();

                for _i in 0..rock.len() {
                    chamber.push(0);
                }
            }
            (0, 0) | (1, 0) => {
                rock = rock1;
            }
            _ => {}
        }
    }

    skipped
}

fn parse_file(filepath: &str) -> (Vec<Vec<u8>>, Vec<Direction>) {
    let bytes = fs::read(filepath).unwrap();
    let mut dirs = Vec::new();
    for b in bytes {
        match b {
            b'>' => dirs.push(Direction::Right),
            b'<' => dirs.push(Direction::Left),
            _ => {}
        }
    }

    (
        vec![
            vec![0b001111_0],
            vec![0b000_1_000, 0b00_111_00, 0b000_1_000],
            vec![0b00_111_00, 0b0000_1_00, 0b0000_1_00],
            vec![0b00_1_0000, 0b00_1_0000, 0b00_1_0000, 0b00_1_0000],
            vec![0b00_11_000, 0b00_11_000],
        ],
        dirs,
    )
}

fn calc2<'a, BI, RI>(dirs_iter: &mut BI, rocks: &mut RI, rock_generation: u64) -> u64
where
    BI: Iterator<Item = &'a Direction>,
    RI: Iterator<Item = Vec<u8>>,
{
    let mut chamber = Vec::new();
    chamber.push(0b1_111_111);
    let skipped = simulate(&mut chamber, dirs_iter, rocks, rock_generation);
    remove_all_floating_zeros(&mut chamber);
    // println!("skipped={}", skipped);
    chamber.len() as u64 - 1 + skipped
}

fn calc<'a, BI, RI>(
    chamber: &mut Vec<u8>,
    dirs_iter: &mut BI,
    rocks: &mut RI,
    rock_generation: u64,
) -> usize
where
    BI: Iterator<Item = &'a Direction>,
    RI: Iterator<Item = Vec<u8>>,
{
    simulate(chamber, dirs_iter, rocks, rock_generation);
    remove_all_floating_zeros(chamber);
    chamber.len()
}

fn process(filepath: &str, rock_generation: u64) -> u64 {
    let (rocks_vec, dirs_vec) = parse_file(filepath);
    let mut rocks = rocks_vec.into_iter().cycle();
    let mut dirs_iter = dirs_vec.iter().cycle();
    calc2(&mut dirs_iter, &mut rocks, rock_generation  + 1)
}

pub fn part1(filepath: &str) -> u64 {
    process(filepath, 2022)
    // let (rocks_vec, dirs_vec) = parse_file(filepath);
    // let mut rocks = rocks_vec.into_iter().cycle();
    // let mut dirs_iter = dirs_vec.iter().cycle();
    // calc2(&mut dirs_iter, &mut rocks, 1000000000000 as u64 + 1)
}

pub fn part2(filepath: &str) -> u64 {
    process(filepath, 1000000000000)
}

#[inline]
fn remove_all_floating_zeros(chamber: &mut Vec<u8>) {
    loop {
        if let Some(b) = chamber.pop() {
            if b != 0 {
                chamber.push(b);
                break;
            }
        }
    }
}

#[inline]
fn print_chamber(chamber: &mut Vec<u8>) {
    for i in chamber.iter().rev() {
        println!("{:#09b}", *i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_only_move() {
        let mut rock = vec![0b010_00, 0b111_00, 0b010_00];
        rock_only_move(&mut rock, &Direction::Left);
        assert_eq!(vec![0b010_000, 0b111_000, 0b010_000,], rock);
        rock_only_move(&mut rock, &Direction::Left);
        assert_eq!(vec![0b010_0000, 0b111_0000, 0b010_0000,], rock);
        rock_only_move(&mut rock, &Direction::Left);
        assert_eq!(vec![0b010_0000, 0b111_0000, 0b010_0000,], rock);

        let mut rock = vec![0b10];
        rock_only_move(&mut rock, &Direction::Right);
        assert_eq!(vec![0b1], rock);
        rock_only_move(&mut rock, &Direction::Right);
        assert_eq!(vec![0b1], rock);
    }

    #[test]
    fn test_part1() {
        assert_eq!(3068, part1("data/day17-sample.txt"));
        assert_eq!(3119, part1("data/day17.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1514285714288, part2("data/day17-sample.txt"));
        assert_eq!(1536994219669, part2("data/day17.txt"));
    }
}
