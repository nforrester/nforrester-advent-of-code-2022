use rusty_advent::*;

use std::collections::HashSet;

fn deframer(filename: &str, marker_length: usize) -> usize {
    let input: Vec<char> = vec_char(&std::fs::read_to_string(filename).unwrap());
    for i in marker_length..input.len() {
        let mut s = HashSet::new();
        for j in 1..=marker_length {
            s.insert(input[i-j]);
        }
        if s.len() == marker_length {
            return i;
        }
    }
    panic!("No marker found!");
}

fn part1(filename: &str) {
    let answer = deframer(filename, 4);
    println!("{}", answer);
    assert_eq!(answer, 1210);
}

fn part2(filename: &str) {
    let answer = deframer(filename, 14);
    println!("{}", answer);
    assert_eq!(answer, 3476);
}

fn main() {
    part1("input/d6.txt");
    part2("input/d6.txt");
}
