use std::fs;

pub fn index_of(v: &Vec<u8>, b: &u8) -> Option<u32> {
    let mut index = 0;
    for item in v {
        if *item == *b {
            return Some(index);
        }
        index += 1;
    }
    None
}

pub fn process(distinct_count: u32, bytes: &Vec<u8>) -> u32 {
    let len = bytes.len() as u32;
    let mut iter = bytes.iter();

    let mut unique = Vec::new();
    let mut count = 0;

    loop {
        if let Some(b) = iter.next() {
            if let Some(idx) = index_of(&unique, b) {
                // clear 0..=idx
                unique.drain(0..=(idx as usize));
            }
            unique.push(*b);
            if unique.len() >= distinct_count as usize {
                return count + 1;
            }
            count += 1;
        } else {
            return len;
        }
    }
}

// 1093
pub fn part1() -> u32 {
    let bytes = fs::read("data/day6.txt").expect("expect file");
    process(4, &bytes)
}

// 3534
pub fn part2() -> u32 {
    let bytes = fs::read("data/day6.txt").expect("expect file");
    process(14, &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let bytes = Vec::from(*b"bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(5, process(4, &bytes));
        let bytes = Vec::from(*b"nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(6, process(4, &bytes));
        let bytes = Vec::from(*b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(10, process(4, &bytes));
        let bytes = Vec::from(*b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(11, process(4, &bytes));
    }

    #[test]
    fn test_part2() {
        let bytes = Vec::from(*b"mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(19, process(14, &bytes));
        let bytes = Vec::from(*b"bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(23, process(14, &bytes));
        let bytes = Vec::from(*b"nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(23, process(14, &bytes));
        let bytes = Vec::from(*b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(29, process(14, &bytes));
        let bytes = Vec::from(*b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(26, process(14, &bytes));
    }
}
