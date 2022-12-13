use rusty_advent::*;

use std::collections::VecDeque;
use std::iter::zip;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
enum Item {
    Integer(i64),
    List(Vec<Item>),
}

fn parse_item(input: &mut VecDeque<char>) -> Item {
    if input[0] == '[' {
        input.pop_front();
        let mut vec = Vec::new();
        if input[0] == ']' {
            input.pop_front();
            return Item::List(vec);
        }
        loop {
            vec.push(parse_item(input));
            let ch = input.pop_front().unwrap();
            if ch == ']' {
                break;
            }
            assert!(ch == ',');
        }
        return Item::List(vec);
    } else {
        let mut int = String::new();
        while ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&input[0]) {
            int.push(input.pop_front().unwrap());
        }
        let int: i64 = int.parse().unwrap();
        return Item::Integer(int);
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Integer(x), Item::Integer(y)) => {
                    if x < y {
                        Ordering::Less
                    } else if x > y {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                },
            (Item::List(xs), Item::List(ys)) => {
                    for (x, y) in zip(xs, ys) {
                        match x.cmp(y) {
                            Ordering::Equal => continue,
                            c => return c,
                        }
                    }
                    let x = xs.len();
                    let y = ys.len();
                    if x < y {
                        Ordering::Less
                    } else if x > y {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                },
            (Item::Integer(x), Item::List(ys)) => Item::cmp(&Item::List(vec![Item::Integer(*x)]), &Item::List(ys.clone())),
            (Item::List(xs), Item::Integer(y)) => Item::cmp(&Item::List(xs.clone()), &Item::List(vec![Item::Integer(*y)])),
        }
    }
}

fn part1(filename: &str) {
    let input = file_vec_vec_char(filename);
    let mut input = input.iter();
    let mut answer = 0;
    let mut idx = 1;
    loop {
        let mut left = input.next().unwrap().iter().map(|x|{*x}).collect();
        let left = parse_item(&mut left);
        let mut right = input.next().unwrap().iter().map(|x|{*x}).collect();
        let right = parse_item(&mut right);
        if left.cmp(&right) != Ordering::Greater {
            answer += idx;
        }
        idx += 1;
        if let None = input.next() {
            break;
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 5717);
}

fn part2(filename: &str) {
    let input = file_vec_vec_char(filename);
    let mut input = input.iter();
    let mut items = Vec::new();
    loop {
        for _ in 0..2 {
            let mut item = input.next().unwrap().iter().map(|x|{*x}).collect();
            items.push(parse_item(&mut item));
        }
        if let None = input.next() {
            break;
        }
    }
    let d2 = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
    let d6 = Item::List(vec![Item::List(vec![Item::Integer(6)])]);
    items.push(d2.clone());
    items.push(d6.clone());
    items.sort();
    let mut answer = 1;
    for (x, item) in items.iter().enumerate() {
        if [&d2, &d6].contains(&item) {
            answer *= x + 1;
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 25935);
}


fn main() {
    part1("input/d13.txt");
    part2("input/d13.txt");
}
