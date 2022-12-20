use std::{collections::VecDeque, fs, iter::Peekable, str};

struct GeodeCount {
    max: u32,
}

type EntityCount = [u32; 4];

struct State {
    robot: [u32; 4],
    inventory: [u32; 4],
    remaining_minute: u32,
}

fn bfs(costs: &[[u32; 3]; 4], max_time: u32) -> u32 {
    let mut max_robot = [0; 4];
    for i in 0..3 {
        max_robot[i] = costs.iter().map(|x| x[i]).max().unwrap();
    }
    max_robot[3] = u32::MAX;

    let mut queue = VecDeque::from([State {
        robot: [1, 0, 0, 0],
        inventory: [0, 0, 0, 0],
        remaining_minute: max_time,
    }]);
    let mut max = 0;
    while let Some(State {
        robot,
        inventory,
        remaining_minute,
    }) = queue.pop_front()
    {
        if inventory[3] > max {
            max = inventory[3]
        }

        // if robot[0] == 1 && robot[1] == 4 {
        //     println!(
        //         "robot={:?} inventory={:?} remaining_minute={}",
        //         robot, inventory, remaining_minute
        //     );
        // }

        if remaining_minute == 0 {
            continue;
        }

        let mut build = false;
        for which_robot in (0..4).rev() {
            if robot[which_robot] >= max_robot[which_robot] {
                continue;
            }

            let cost = costs[which_robot];
            let time_needed_to_build =
                if cost[0] <= inventory[0] && cost[1] <= inventory[1] && cost[2] <= inventory[2] {
                    1
                } else if (robot[0] == 0 && cost[0] > 0)
                    || (robot[1] == 0 && cost[1] > 0)
                    || (robot[2] == 0 && cost[2] > 0)
                {
                    max_time
                } else {
                    let mut max_wait = 0;
                    // let mut need_wait = Vec::new();
                    for i in 0..3 {
                        if cost[i] > inventory[i] {
                            let rounded = if (cost[i] - inventory[i]) % robot[i] == 0 {
                                0
                            } else {
                                1
                            };
                            let wait = rounded + (cost[i] - inventory[i]) / robot[i];
                            if wait > max_wait {
                                max_wait = wait;
                            }
                        }
                    }
                    max_wait + 1
                };
            if time_needed_to_build >= remaining_minute {
                continue;
            }

            let mut robot_next = robot.clone();
            robot_next[which_robot] += 1;
            let inventory_next = [
                inventory[0] + (time_needed_to_build) * robot[0] - cost[0],
                inventory[1] + (time_needed_to_build) * robot[1] - cost[1],
                inventory[2] + (time_needed_to_build) * robot[2] - cost[2],
                inventory[3] + (time_needed_to_build) * robot[3],
            ];
            build = true;
            queue.push_back(State {
                robot: robot_next,
                inventory: inventory_next,
                remaining_minute: remaining_minute - time_needed_to_build,
            });

            // if we can build a geode robot imediately, we skip building other robots
            if which_robot == 3 && time_needed_to_build == 1 {
                break;
            }
        }

        if !build {
            let geode = remaining_minute * robot[3] + inventory[3];
            if geode > max {
                max = geode;
            }
        }
    }
    max
}

fn parse_number<I>(iter: &mut Peekable<I>) -> u32
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

fn parse_line(line: &str) -> [[u32; 3]; 4] {
    let bytes = line.bytes();
    let mut iter = bytes.peekable();
    let mut numbers = Vec::new();
    while let Some(b) = iter.peek() {
        match *b {
            (b'0'..=b'9') => {
                numbers.push(parse_number(&mut iter));
            }
            _ => {
                iter.next();
            }
        }
    }
    [
        [numbers[1], 0, 0],
        [numbers[2], 0, 0],
        [numbers[3], numbers[4], 0],
        [numbers[5], 0, numbers[6]],
    ]
}

pub fn part1(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut all_costs = Vec::new();
    for line in cnt.lines() {
        all_costs.push(parse_line(line));
    }

    let mut quality_levels = Vec::new();
    for (i, costs) in all_costs.iter().enumerate() {
        quality_levels.push(bfs(costs, 24) * (i as u32 + 1));
    }

    quality_levels.iter().sum::<u32>()
}

pub fn part2(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).unwrap();
    let mut all_costs = Vec::new();
    let mut count = 0;
    for line in cnt.lines() {
        count += 1;
        if count > 3 {
            break;
        }
        all_costs.push(parse_line(line));
    }

    let mut quality_levels = Vec::new();
    for costs in all_costs.iter() {
        quality_levels.push(bfs(costs, 32));
    }

    quality_levels.iter().product::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(33, part1("data/day19-sample.txt"));
        assert_eq!(1565, part1("data/day19.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3472, part2("data/day19-sample.txt"));
        assert_eq!(10672, part2("data/day19.txt"));
    }
}
