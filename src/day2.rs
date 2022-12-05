use std::fs;

// cheating

fn calc_score_part1(row: &str) -> u32 {
    match row {
        "A X" => 1 + 3,
        "B X" => 1,
        "C X" => 1 + 6,
        "A Y" => 2 + 6,
        "B Y" => 2 + 3,
        "C Y" => 2,
        "A Z" => 3,
        "B Z" => 3 + 6,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn calc_score_part2(row: &str) -> u32 {
    match row {
        // lose
        "A X" => 3,
        "B X" => 1,
        "C X" => 2,
        // draw
        "A Y" => 3 + 1,
        "B Y" => 3 + 2,
        "C Y" => 3 + 3,
        // win
        "A Z" => 6 + 2,
        "B Z" => 6 + 3,
        "C Z" => 6 + 1,
        _ => 0,
    }
}

pub fn part1() -> u32 {
    let contents = fs::read_to_string("data/day2.txt").expect("expect file");
    let mut score = 0;
    for line in contents.lines() {
        score += calc_score_part1(line)
    }
    score
}

pub fn part2() -> u32 {
    let contents = fs::read_to_string("data/day2.txt").expect("expect file");
    let mut score = 0;
    for line in contents.lines() {
        score += calc_score_part2(line)
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(11449, part1());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(13187, part2());
    }
}
