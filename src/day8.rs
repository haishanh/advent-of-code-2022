use std::fs;

fn calc_scenic_score(
    matrix: &Vec<&[u8]>,
    row: usize,
    col: usize,
    row_count: usize,
    col_count: usize,
) -> u32 {
    if row == 0 || row == row_count - 1 || col == 0 || col == col_count - 1 {
        return 0;
    }

    let b = matrix[row][col];

    let mut a0 = 0; // down
    let mut a1 = 0; // up
    let mut a2 = 0; // right
    let mut a3 = 0; // left

    let mut idx = row + 1;
    loop {
        let x = matrix[idx][col];
        a0 += 1;
        if b <= x {
            break;
        }
        if idx == row_count - 1 {
            break;
        } else {
            idx += 1;
        }
    }

    let mut idx = row - 1;
    loop {
        let x = matrix[idx][col];
        a1 += 1;
        if b <= x {
            break;
        }

        if idx == 0 {
            break;
        } else {
            idx -= 1;
        }
    }

    let r = matrix[row];

    let mut idx = col + 1;
    loop {
        let x = r[idx];
        a2 += 1;
        if b <= x {
            break;
        }
        if idx == row_count - 1 {
            break;
        } else {
            idx += 1;
        }
    }

    let mut idx = col - 1;
    loop {
        let x = r[idx];
        a3 += 1;
        if b <= x {
            break;
        }
        if idx == 0 {
            break;
        } else {
            idx -= 1;
        }
    }

    a0 * a1 * a2 * a3
}

fn is_local_minimal(
    matrix: &Vec<&[u8]>,
    row: usize,
    col: usize,
    row_count: usize,
    col_count: usize,
) -> bool {
    if row == 0 || row == row_count - 1 || col == 0 || col == col_count - 1 {
        return false;
    }

    let b = matrix[row][col];

    let mut count = 0;

    let mut idx = row + 1;
    loop {
        let x = matrix[idx][col];
        if b <= x {
            count += 1;
            break;
        }
        if idx == row_count - 1 {
            break;
        } else {
            idx += 1;
        }
    }

    let mut idx = row - 1;
    loop {
        let x = matrix[idx][col];
        if b <= x {
            count += 1;
            break;
        }
        if idx == 0 {
            break;
        } else {
            idx -= 1;
        }
    }

    let r = matrix[row];

    let mut idx = col + 1;
    loop {
        let x = r[idx];
        if b <= x {
            count += 1;
            break;
        }
        if idx == row_count - 1 {
            break;
        } else {
            idx += 1;
        }
    }

    let mut idx = col - 1;
    loop {
        let x = r[idx];
        if b <= x {
            count += 1;
            break;
        }
        if idx == 0 {
            break;
        } else {
            idx -= 1;
        }
    }

    count == 4
}

pub fn part1(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).expect("expect file");

    let mut matrix = Vec::new();

    for line in cnt.lines() {
        let bytes = line.as_bytes();
        matrix.push(bytes);
    }

    let row_count = matrix.len();
    let col_count = matrix[0].len();

    let mut hidden_count = 0;

    let mut row = 0;
    while row < row_count {
        let mut col = 0;
        while col < col_count {
            if is_local_minimal(&matrix, row, col, row_count, col_count) {
                hidden_count += 1;
            }
            col += 1;
        }
        row += 1;
    }

    (row_count as u32) * (col_count as u32) - hidden_count
}

pub fn part2(filepath: &str) -> u32 {
    let cnt = fs::read_to_string(filepath).expect("expect file");

    let mut matrix = Vec::new();

    for line in cnt.lines() {
        let bytes = line.as_bytes();
        matrix.push(bytes);
    }

    let row_count = matrix.len();
    let col_count = matrix[0].len();

    let mut max_score = 0;

    // calc_scenic_score(&matrix, 1, 2, row_count, col_count)

    let mut row = 0;
    while row < row_count {
        let mut col = 0;
        while col < col_count {
            let score = calc_scenic_score(&matrix, row, col, row_count, col_count);
            if score > max_score {
                max_score = score;
            }
            col += 1;
        }
        row += 1;
    }
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(21, part1("data/day8-sample.txt"));
        assert_eq!(1698, part1("data/day8.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2("data/day8-sample.txt"));
        assert_eq!(672280, part2("data/day8.txt"));
    }
}
