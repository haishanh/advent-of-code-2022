use std::cell::RefCell;
use std::fs;
use std::iter::Peekable;
use std::rc::Rc;
use std::str;

#[derive(Debug)]
enum Token {
    Cd(String),
    Ls,
    Dir(String),
    FileStat { size: u32 },
}

fn parse_string<'a, I>(iter: &mut Peekable<I>) -> String
where
    I: Iterator<Item = &'a u8>,
{
    let mut bytes = Vec::new();
    for b in iter {
        if *b != b' ' {
            bytes.push(*b);
        }
    }
    str::from_utf8(&bytes).unwrap().to_owned()
}

fn parse_dir_stat<'a, I>(iter: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = &'a u8>,
{
    Token::Dir(parse_string(iter))
}

fn parse_cmd_cd<'a, I>(iter: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = &'a u8>,
{
    Token::Cd(parse_string(iter))
}

fn parse_cmd<'a, I>(iter: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = &'a u8>,
{
    loop {
        match iter.peek() {
            Some(&&b' ') => {
                iter.next();
            }
            Some(&&b'c') => {
                iter.next();
                let b = iter.peek().unwrap();
                if b == &&b'd' {
                    iter.next();
                    return parse_cmd_cd(iter);
                } else {
                    panic!("invalid input");
                }
            }
            Some(&&b'l') => {
                iter.next();
                let b = iter.peek().unwrap();
                if b == &&b's' {
                    iter.next();
                    return Token::Ls;
                } else {
                    panic!("invalid input");
                }
            }
            _ => panic!("invalid input"),
        }
    }
}

fn parse_file_stat<'a, I>(iter: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = &'a u8>,
{
    let mut digits = Vec::new();
    while let Some(b) = iter.next() {
        if *b >= b'0' && *b <= b'9' {
            digits.push(*b);
        } else {
            break;
        }
    }

    let mut bytes = Vec::new();
    while let Some(b) = iter.next() {
        if *b != b' ' {
            bytes.push(*b);
        } else {
            break;
        }
    }
    // let name = str::from_utf8(&bytes).unwrap();
    Token::FileStat {
        size: str::from_utf8(&digits).unwrap().parse().unwrap(),
    }
}

fn parse_line<'a, I>(iter: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = &'a u8>,
{
    let b = iter.peek().unwrap();
    if b == &&b'$' {
        // eat '$'
        iter.next();
        return parse_cmd(iter);
    }

    if b == &&b'd' {
        // eat 'd' 'i' 'r'
        iter.next();
        iter.next();
        iter.next();
        return parse_dir_stat(iter);
    }

    if **b >= b'0' && **b <= b'9' {
        return parse_file_stat(iter);
    } else {
        panic!("invalid input");
    }
}

fn tokenize(cnt: String) -> Vec<Token> {
    let mut v = Vec::new();
    let lines = cnt.lines();
    for line in lines {
        let bytes = line.as_bytes();
        let mut iter = bytes.iter().peekable();
        let t = parse_line(&mut iter);
        v.push(t);
    }
    v
}

fn scan_tokens(tokens: &Vec<Token>) -> Vec<Rc<RefCell<u32>>> {
    let mut all = Vec::new();
    let mut chain = Vec::new();
    for t in tokens {
        match t {
            Token::Ls => {}
            Token::Cd(name) => {
                if &name[..] == ".." {
                    chain.pop();
                } else {
                    let dir_size = Rc::new(RefCell::new(0));
                    all.push(Rc::clone(&dir_size));
                    chain.push(Rc::clone(&dir_size));
                }
            }
            Token::Dir(_name) => {}
            Token::FileStat { size } => {
                for s in chain.iter() {
                    let mut x = s.borrow_mut();
                    *x += *size;
                }
            }
        }
    }
    all
}

fn calc_part1(dirs: &Vec<Rc<RefCell<u32>>>) -> u32 {
    let mut nums = Vec::new();
    for i in dirs {
        let dir_size = i.borrow();
        if *dir_size <= 100000 {
            nums.push(*dir_size);
        }
    }
    nums.iter().sum()
}

fn calc_part2(dirs: &Vec<Rc<RefCell<u32>>>) -> u32 {
    let mut nums = Vec::new();
    for i in dirs {
        let dir_size = i.borrow();
        nums.push(*dir_size);
    }
    nums.sort();
    let root = nums.pop().unwrap();
    let free = 70000000 - root;
    for n in nums {
        if n + free >= 30000000 {
            return n;
        }
    }
    // we shouldn't reach here
    0
}

pub fn part1(filepath: &str) -> u32 {
    let content = fs::read_to_string(filepath).expect("expect file");
    let tokens = tokenize(content);
    let dirs = scan_tokens(&tokens);
    calc_part1(&dirs)
}

pub fn part2(filepath: &str) -> u32 {
    let content = fs::read_to_string(filepath).expect("expect file");
    let tokens = tokenize(content);
    let dirs = scan_tokens(&tokens);
    calc_part2(&dirs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(95437, part1("data/day7-sample.txt"));
        assert_eq!(1778099, part1("data/day7.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(24933642, part2("data/day7-sample.txt"));
        assert_eq!(1623571, part2("data/day7.txt"));
    }
}
