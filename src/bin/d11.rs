use rusty_advent::*;

use std::rc::Rc;
use std::vec::Vec;
use std::collections::VecDeque;
//use std::collections::HashSet;
//use std::collections::HashMap;
//use std::cmp::max;
//use std::cmp::min;

type MonkeyModuli = Vec<i64>;
type Worry = Vec<i64>;
type WorryOp = Rc<dyn Fn(i64) -> i64>;
type TestFun = Rc<dyn Fn(i64) -> usize>;

struct Monkey {
    items: VecDeque<Worry>,
    operation: WorryOp,
    test: TestFun,
    inspections: i64,
}

fn parse_monkeys(filename: &str) -> (Vec<Monkey>, MonkeyModuli) {
    let words = file_vec_vec_word(filename);
    let mut num_monkeys = 0;
    for line in 0..words.len() {
        if words[line].len() == 0 {
            continue;
        }
        if words[line][0] == "Monkey" {
            num_monkeys += 1;
        }
    }

    let mut monkeys = Vec::new();
    let mut moduli = Vec::new();
    let mut line: usize = 0;
    'outer: while line < words.len() {
        while words[line].len() == 0 || words[line][0] != "Monkey" {
            line += 1;
            if line >= words.len() {
                break 'outer;
            }
        }
        line += 1;
        assert_eq!(words[line][0], "Starting");
        let mut items: VecDeque<Worry> = VecDeque::new();
        for idx in 2..words[line].len() {
            let mut word = words[line][idx].to_string();
            if word.ends_with(",") {
                word.pop();
            }
            let w = word.parse().unwrap();
            items.push_back((0..num_monkeys).map(|_|{w}).collect());
        }
        line += 1;
        let strs = words[line].iter().map(|s|{s.as_str()}).collect::<Vec<&str>>();
        let operation: WorryOp = match strs.as_slice() {
            ["Operation:", "new", "=", "old", "+", "old"] => Rc::new(move |old: i64| -> i64 { old + old }),
            ["Operation:", "new", "=", "old", "*", "old"] => Rc::new(move |old: i64| -> i64 { old * old }),
            ["Operation:", "new", "=", "old", "+", x] => { let x = x.parse::<i64>().unwrap(); Rc::new(move |old: i64| -> i64 { old + x }) },
            ["Operation:", "new", "=", "old", "*", x] => { let x = x.parse::<i64>().unwrap(); Rc::new(move |old: i64| -> i64 { old * x }) },
            _ => panic!(""),
        };
        line += 1;
        let strs = words[line].iter().map(|s|{s.as_str()}).collect::<Vec<&str>>();
        let x: i64 = match strs.as_slice() {
            ["Test:", "divisible", "by", x] => x.parse().unwrap(),
            _ => panic!(""),
        };
        line += 1;
        let strs = words[line].iter().map(|s|{s.as_str()}).collect::<Vec<&str>>();
        let mt: usize = match strs.as_slice() {
            ["If", "true:", "throw", "to", "monkey", x] => x.parse().unwrap(),
            _ => panic!(""),
        };
        line += 1;
        let strs = words[line].iter().map(|s|{s.as_str()}).collect::<Vec<&str>>();
        let mf: usize = match strs.as_slice() {
            ["If", "false:", "throw", "to", "monkey", x] => x.parse().unwrap(),
            _ => panic!(""),
        };
        let test: TestFun = 
            Rc::new(move |worry: i64| -> usize {
                    if worry % x == 0 {
                        mt
                    } else {
                        mf
                    }
                });
        monkeys.push(Monkey { items, operation, test, inspections: 0 });
        moduli.push(x);
    }
    return (monkeys, moduli);
}

//fn part1(filename: &str) {
//    let (mut monkeys, moduli) = parse_monkeys(filename);
//
//    for round in 0..20 {
//        for m in 0..monkeys.len() {
//            while !monkeys[m].items.is_empty() {
//                let item_worry: Worry = monkeys[m].items.pop_front().unwrap();
//                let item_worry: Worry = std::iter::zip(item_worry, moduli).map(|(w, modulus)|{(monkeys[m].operation)(w)%modulus}).collect();
//                let destination = (monkeys[m].test)(item_worry);
//                monkeys[destination].items.push_back(item_worry);
//                monkeys[m].inspections += 1;
//            }
//        }
//    }
//
//    let mut stuff: Vec<i64> = monkeys.iter().map(|m|{m.inspections}).collect();
//    stuff.sort();
//    let stuff: Vec<i64> = stuff.into_iter().rev().collect();
//
//    let mut monkey_business = stuff[0] * stuff[1];
//    println!("{}", monkey_business);
//}

fn part2(filename: &str) {
    let (mut monkeys, moduli) = parse_monkeys(filename);

    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            while !monkeys[m].items.is_empty() {
                let item_worry: Worry = monkeys[m].items.pop_front().unwrap();
                let item_worry: Worry = std::iter::zip(item_worry, moduli.clone()).map(|(w, modulus)|{(monkeys[m].operation)(w)%modulus}).collect();
                let destination = (monkeys[m].test)(item_worry[m]);
                monkeys[destination].items.push_back(item_worry);
                monkeys[m].inspections += 1;
            }
        }
    }

    let mut stuff: Vec<i64> = monkeys.iter().map(|m|{m.inspections}).collect();
    stuff.sort();
    let stuff: Vec<i64> = stuff.into_iter().rev().collect();

    let monkey_business = stuff[0] * stuff[1];
    println!("{}", monkey_business);
}

fn main() {
    //part1("input/d11.ex");
    //part1("input/d11.txt");
    part2("input/d11.ex");
    part2("input/d11.txt");
}
