use rusty_advent::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main () {
    let mut rules = HashMap::new();

    for line in file_vec_vec_word("input/practice-2020d7.txt") {
        let mut line = line.iter();
        let mut ruler_description = String::new();
        loop {
            let word = line.next().unwrap();
            if word == "bags" {
                assert!(line.next().unwrap() == "contain");
                break;
            }
            if ruler_description != "" {
                ruler_description.push_str(" ");
            }
            ruler_description.push_str(word);
        }
        let mut these_rules = HashMap::new();
        'outer: loop {
            let word = line.next().unwrap();
            let count:i64;
            if word == "no" {
                count = 0;
            } else {
                count = word.parse().unwrap();
            }
            let mut ruled_description = String::new();
            loop {
                let word = line.next().unwrap();
                if word.ends_with(",") {
                    these_rules.insert(ruled_description, count);
                    break;
                }
                if word.ends_with(".") {
                    these_rules.insert(ruled_description, count);
                    break 'outer;
                }
                if ruled_description != "" {
                    ruled_description.push_str(" ");
                }
                ruled_description.push_str(word);
            }
        }
        these_rules.remove("other");
        rules.insert(ruler_description, these_rules);
        assert!(line.next() == None);
    }

    let mut possible_bags = HashSet::new();
    for (bag, contents) in rules.iter() {
        if contents.contains_key("shiny gold") {
            possible_bags.insert(bag);
        }
    }
    let mut prev_size = possible_bags.len();
    loop {
        let mut new_possible_bags = HashSet::new();
        for (bag, contents) in rules.iter() {
            for inner_bag in possible_bags.iter() {
                if contents.contains_key(inner_bag.as_str()) {
                    new_possible_bags.insert(bag);
                }
            }
        }
        for new_bag in new_possible_bags.iter() {
            possible_bags.insert(new_bag);
        }
        if prev_size == possible_bags.len() {
            break;
        }
        prev_size = possible_bags.len();
    }
    println!("{}", prev_size);

    println!("part2:");
    let mut bag_count = -1;
    let mut unfilled_bags = HashMap::new();
    unfilled_bags.insert("shiny gold", 1);
    loop {
        let (bag, num) = unfilled_bags.iter().next().unwrap();
        let bag = String::from(*bag);
        let num = *num;
        bag_count += num;
        unfilled_bags.remove(bag.as_str());
        println!("{}", bag);
        for (new_bag, count) in rules.get(bag.as_str()).unwrap() {
            if unfilled_bags.contains_key(new_bag.as_str()) {
                unfilled_bags.insert(new_bag.as_str(), unfilled_bags.get(new_bag.as_str()).unwrap() + num * count);
            } else {
                unfilled_bags.insert(new_bag, num * count);
            }
        }
        if unfilled_bags.len() == 0 {
            break;
        }
    }
    println!("{}", bag_count);
}
