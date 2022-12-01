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
    let cals: Vec<i64> = cals.into_iter().rev().collect();
    let p1: i64 = cals[0];
    let p2: i64 = cals[0..3].into_iter().sum::<i64>();
    println!("p1: {}", p1);
    println!("p2: {}", p2);
    assert_eq!(p1, 67027);
    assert_eq!(p2, 197291);
}
