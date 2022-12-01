use rusty_advent::*;
use std::vec::Vec;

fn main () {
    let mut cals = Vec::new();
    let mut cal: i64 = 0;
    for line in file_lines("input/d1.txt") {
        if let Ok(x) = line.parse::<i64>() {
            cal += x;
        } else {
            cals.push(cal);
            cal = 0;
        }
    }
    cals.push(cal);
    cals.sort();
    let top_one: i64 = cals[cals.len()-1];
    let top_three: i64 = cals[cals.len()-3..].into_iter().sum::<i64>();
    println!("part 1: {top_one}");
    println!("part 2: {top_three}");
    assert_eq!(top_one, 67027);
    assert_eq!(top_three, 197291);
}
