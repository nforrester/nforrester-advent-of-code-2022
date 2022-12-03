use rusty_advent::*;

use std::collections::HashSet;

fn priority(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!(""),
    }
}

fn part1(filename: &str) {
    let mut p1 = 0;
    for line in file_vec_vec_char(filename) {
        let s2 = line.len()/2;
        let mut h1 = HashSet::new();
        let mut h2 = HashSet::new();
        for i in 0..line.len() {
            if i < s2 {
                h1.insert(line[i]);
            } else {
                h2.insert(line[i]);
            }
        }
        let mut c = h1.intersection(&h2);
        p1 += priority(*c.next().unwrap());
    }
    println!("part 1: {p1}");
    assert_eq!(p1, 8176);
}

fn part2(filename: &str) {
    let mut p2 = 0;
    let mut lines = file_vec_vec_char(filename).into_iter();
    loop {
        let mut e1 = HashSet::new();
        if let Some(line) = lines.next() {
            for i in 0..line.len() {
                e1.insert(line[i]);
            }
            let mut e2 = HashSet::new();
            let line = lines.next().unwrap();
            for i in 0..line.len() {
                e2.insert(line[i]);
            }
            let mut e3 = HashSet::new();
            let line = lines.next().unwrap();
            for i in 0..line.len() {
                e3.insert(line[i]);
            }
            let mut e12 = HashSet::new();
            for x in e1.intersection(&e2) {
                e12.insert(*x);
            }
            let mut c = e12.intersection(&e3);
            p2 += priority(*c.next().unwrap());
        } else {
            break;
        }
    }
    println!("part 2: {p2}");
    assert_eq!(p2, 2689);
}

fn main() {
    part1("input/d3.txt");
    part2("input/d3.txt");
}
