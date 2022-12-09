use rusty_advent::*;

use std::collections::HashSet;
use std::cmp::max;
use std::cmp::min;

fn run_rope(filename: &str, l: usize) -> usize {
    let mut visited = HashSet::new();
    let mut x: Vec<i64> = vec![0; l+1];
    let mut y: Vec<i64> = vec![0; l+1];
    visited.insert((x[l], y[l]));
    for words in file_vec_vec_word(filename) {
        let dir = words[0].as_str();
        let move_x = match dir { "L" => -1, "R" => 1, _ => 0 };
        let move_y = match dir { "D" => -1, "U" => 1, _ => 0 };
        for _ in 0..words[1].parse().unwrap() {
            x[0] += move_x;
            y[0] += move_y;
            for i in 0..l {
                let dx = x[i] - x[i+1];
                let dy = y[i] - y[i+1];
                if dx.abs() > 1 || dy.abs() > 1 {
                    x[i+1] += max(-1, min(dx, 1));
                    y[i+1] += max(-1, min(dy, 1));
                }
            }
            visited.insert((x[l], y[l]));
        }
    }
    return visited.len();
}

fn part1(filename: &str) {
    let answer = run_rope(filename, 1);
    println!("{}", answer);
    assert_eq!(answer, 6642);
}

fn part2(filename: &str) {
    let answer = run_rope(filename, 9);
    println!("{}", answer);
    assert_eq!(answer, 2765);
}

fn main() {
    part1("input/d9.txt");
    part2("input/d9.txt");
}
