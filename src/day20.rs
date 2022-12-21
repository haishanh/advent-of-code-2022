use std::cell::RefCell;
use std::fmt::Debug;
use std::fs;
use std::rc::Rc;
use std::str;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Debug + Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, v: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value: v,
            next: None,
            prev: None,
        }));
        if let Some(b) = &self.head {
            new_node.borrow_mut().next = Some(b.clone());
            b.borrow_mut().prev = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        if let Some(b) = &self.tail {
            new_node.borrow_mut().prev = Some(b.clone());
            b.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            self.tail = Some(new_node.clone());
        }
    }

    pub fn move_node(&mut self, node: Rc<RefCell<Node<T>>>, step: i32) {
        if step == 0 {
            return ();
        }
        let mut prev = node.clone().borrow().prev.as_ref().unwrap().clone();
        let mut next = node.clone().borrow().next.as_ref().unwrap().clone();
        prev.clone().borrow_mut().next = Some(next.clone());
        next.clone().borrow_mut().prev = Some(prev.clone());

        if step > 0 {
            let mut count = 0;
            loop {
                count += 1;
                if count >= step {
                    break;
                }
                next = next.clone().borrow().next.as_ref().unwrap().clone();
            }
            let next_next = next.clone().borrow().next.as_ref().unwrap().clone();
            next.clone().borrow_mut().next = Some(node.clone());
            node.clone().borrow_mut().prev = Some(next.clone());
            node.clone().borrow_mut().next = Some(next_next.clone());
            next_next.clone().borrow_mut().prev = Some(node.clone());
        } else {
            let mut count = step;
            loop {
                count += 1;
                if count >= 0 {
                    break;
                }
                prev = prev.clone().borrow().prev.as_ref().unwrap().clone();
            }
            let pp = prev.clone().borrow().prev.as_ref().unwrap().clone();
            node.clone().borrow_mut().next = Some(prev.clone());
            prev.clone().borrow_mut().prev = Some(node.clone());
            node.clone().borrow_mut().prev = Some(pp.clone());
            pp.clone().borrow_mut().next = Some(node.clone());
        }
    }

    pub fn look_forward(&mut self, node: Rc<RefCell<Node<T>>>, step: i32) -> T {
        if step == 0 {
            return node.borrow().value;
        }

        let mut next = node.clone().borrow().next.as_ref().unwrap().clone();
        let mut count = 0;
        loop {
            count += 1;
            if count >= step {
                return next.borrow().value;
            }
            next = next.clone().borrow().next.as_ref().unwrap().clone();
        }
    }
}

pub fn solve(part: u8, filepath: &str) -> i64 {
    let mut list: List<i64> = List::new();
    let mut v = Vec::new();

    let mut zeros = Vec::new();

    let cnt = fs::read_to_string(filepath).unwrap();
    for line in cnt.lines() {
        let x = line.parse::<i64>().unwrap();
        list.push_back(x);
        v.push(list.tail.as_ref().unwrap().clone());
        if x == 0 {
            zeros.push(list.tail.as_ref().unwrap().clone());
        }
    }

    let len = v.len() as i64;

    let mut multiplier = 1;
    let mut mix_times = 1;

    if part == 2 {
        multiplier = 811589153;
        mix_times = 10;
    }

    for _ in 0..mix_times {
        for item in v.iter() {
            let mut step = item.borrow().value;
            step = (step * multiplier) % (len - 1);
            list.move_node(item.clone(), step as i32)
        }
    }

    let a = list.look_forward(zeros[0].clone(), (1000 % len) as i32);
    println!("{a}");
    let b = list.look_forward(zeros[0].clone(), (2000 % len) as i32);
    println!("{b}");
    let c = list.look_forward(zeros[0].clone(), (3000 % len) as i32);
    println!("{c}");

    (a + b + c) * multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3, solve(1, "data/day20-sample.txt"));
        assert_eq!(8372, solve(1, "data/day20.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1623178306, solve(2, "data/day20-sample.txt"));
        assert_eq!(7865110481723, solve(2, "data/day20.txt"));
    }
}
