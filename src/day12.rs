use std::collections::HashSet;
use std::fs;
use std::str;

type Pos = (usize, usize);

fn read_to_grid(filepath: &str) -> (Vec<Vec<u8>>, Pos, Pos, Vec<Pos>) {
    let cnt = fs::read_to_string(filepath).unwrap();

    let mut grid = Vec::new();

    let mut start = (0, 0);
    let mut stop = (0, 0);
    let mut low_elevation_points = Vec::new();

    let mut row = 0;

    for line in cnt.lines() {
        let mut col = 0;
        let bytes = line.as_bytes();
        let mut row_bytes = Vec::new();
        for b in bytes {
            if *b == b'S' {
                start.0 = col;
                start.1 = row;
                row_bytes.push(b'a');
                low_elevation_points.push((col, row));
            } else if *b == b'E' {
                stop.0 = col;
                stop.1 = row;
                row_bytes.push(b'z');
            } else {
                row_bytes.push(*b);
                if *b == b'a' {
                    low_elevation_points.push((col, row));
                }
            }
            col += 1;
        }
        grid.push(row_bytes);
        row += 1;
    }
    (grid, start, stop, low_elevation_points)
}

fn find_path(grid: &Vec<Vec<u8>>, starts: Vec<(usize, usize)>, stop: (usize, usize)) -> u32 {
    let mut frontiers = starts;

    let mut step = 0;
    let mut max_step = 0;

    let row_bound = grid.len() - 1;
    let col_bound = grid[0].len() - 1;

    let mut seen = HashSet::new();
    for p in frontiers.iter() {
        seen.insert(*p);
    }

    loop {
        let mut next_frontiers = Vec::new();
        // if step <= 5 {
        //     println!("{} {:?}", step, frontiers);
        // }
        for point in frontiers {
            if point == stop {
                max_step = step;
                break;
            }
            let x = point.0;
            let y = point.1;
            let v = grid[y][x];

            if x > 0 && !seen.contains(&(x - 1, y)) && (grid[y][x - 1]) <= v + 1 {
                seen.insert((x - 1, y));
                next_frontiers.push((x - 1, y));
            }
            if x < col_bound && !seen.contains(&(x + 1, y)) && (grid[y][point.0 + 1]) <= 1 + v {
                seen.insert((x + 1, y));
                next_frontiers.push((x + 1, y));
            }
            if y > 0 && !seen.contains(&(x, y - 1)) && (grid[y - 1][x]) <= 1 + v {
                seen.insert((x, y - 1));
                next_frontiers.push((x, y - 1));
            }
            if y < row_bound && !seen.contains(&(x, y + 1)) && (grid[y + 1][point.0]) <= 1 + v {
                seen.insert((x, y + 1));
                next_frontiers.push((x, y + 1));
            }
        }
        if max_step > 0 {
            break;
        }

        frontiers = next_frontiers;
        step += 1;
    }
    max_step
}

pub fn part1(filepath: &str) -> u32 {
    let (grid, start, stop, _) = read_to_grid(filepath);
    find_path(&grid, vec![start], stop)
}

pub fn part2(filepath: &str) -> u32 {
    let (grid, _start, stop, low_elevation_points) = read_to_grid(filepath);
    find_path(&grid, low_elevation_points, stop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(31, part1("data/day12-sample.txt"));
        assert_eq!(456, part1("data/day12.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, part2("data/day12-sample.txt"));
        assert_eq!(454, part2("data/day12.txt"));
    }
}
