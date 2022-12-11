use rusty_advent::*;

use std::rc::Rc;
use std::vec::Vec;
use std::collections::VecDeque;

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
    let lines = file_vec_vec_word(filename);
    let num_monkeys = lines.iter().filter(|line|{ line.len() > 0 && line[0] == "Monkey" }).count();

    let mut monkeys = Vec::new();
    let mut moduli = Vec::new();
    let mut lines = lines.iter();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }
        assert_eq!(line[0], "Monkey");

        let items: VecDeque<Worry> = match slice_of_strs!(lines.next().unwrap()) {
            ["Starting", "items:", xs @ ..] => {
                    xs.iter().map(|word|{
                            let mut word = word.to_string();
                            if word.ends_with(",") {
                                word.pop();
                            }
                            let worry = word.parse().unwrap();
                            vec![worry; num_monkeys]
                        }).collect()
                },
            _ => panic!("Bad items"),
        };

        let operation: WorryOp = match slice_of_strs!(lines.next().unwrap()) {
            ["Operation:", "new", "=", "old", "+", "old"] => Rc::new(move |old: i64| -> i64 { old + old }),
            ["Operation:", "new", "=", "old", "*", "old"] => Rc::new(move |old: i64| -> i64 { old * old }),
            ["Operation:", "new", "=", "old", "+", x] => { let x: i64 = x.parse().unwrap(); Rc::new(move |old: i64| -> i64 { old + x }) },
            ["Operation:", "new", "=", "old", "*", x] => { let x: i64 = x.parse().unwrap(); Rc::new(move |old: i64| -> i64 { old * x }) },
            _ => panic!("Bad operation"),
        };

        let modulus = match slice_of_strs!(lines.next().unwrap()) {
            ["Test:", "divisible", "by", x] => x.parse().unwrap(),
            _ => panic!("Bad test"),
        };

        let monkey_true = match slice_of_strs!(lines.next().unwrap()) {
            ["If", "true:", "throw", "to", "monkey", x] => x.parse().unwrap(),
            _ => panic!("Bad true outcome"),
        };

        let monkey_false = match slice_of_strs!(lines.next().unwrap()) {
            ["If", "false:", "throw", "to", "monkey", x] => x.parse().unwrap(),
            _ => panic!("Bad false outcome"),
        };

        let test = Rc::new(move |worry: i64| -> usize {
                if worry % modulus == 0 {
                    monkey_true
                } else {
                    monkey_false
                }
            });

        monkeys.push(Monkey { items, operation, test, inspections: 0 });
        moduli.push(modulus);
    }
    return (monkeys, moduli);
}

fn run_monkeys<F>(filename: &str, rounds: u64, reduce_worry: F) -> i64
    where F: Fn(i64, i64) -> i64 {
    let (mut monkeys, moduli) = parse_monkeys(filename);

    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            while !monkeys[m].items.is_empty() {
                let item_worry: Worry = monkeys[m].items.pop_front().unwrap();
                let item_worry: Worry = std::iter::zip(item_worry, moduli.clone())
                    .map(|(w, modulus)|{
                            reduce_worry((monkeys[m].operation)(w), modulus)
                        })
                    .collect();
                let destination = (monkeys[m].test)(item_worry[m]);
                monkeys[destination].items.push_back(item_worry);
                monkeys[m].inspections += 1;
            }
        }
    }

    let mut inspection_counts: Vec<i64> = monkeys.iter().map(|m|{m.inspections}).collect();
    inspection_counts.sort();
    return inspection_counts.into_iter().rev().take(2).fold(1, |a, b|{a * b});
}

fn part1(filename: &str) {
    let monkey_business = run_monkeys(filename, 20, |worry, _|{worry/3});
    println!("{}", monkey_business);
    assert_eq!(monkey_business, 101436);
}

fn part2(filename: &str) {
    let monkey_business = run_monkeys(filename, 10000, |worry, modulus|{worry%modulus});
    println!("{}", monkey_business);
    assert_eq!(monkey_business, 19754471646);
}

fn main() {
    part1("input/d11.txt");
    part2("input/d11.txt");
}
