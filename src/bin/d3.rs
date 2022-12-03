use rusty_advent::*;

use std::collections::HashSet;

fn priority(c: char) -> i32 {
    c.to_lowercase().next().unwrap() as i32
    - 'a' as i32
    + 1
    + if c.is_uppercase() { 26 } else { 0 }
}

fn part1(filename: &str) {
    let mut answer = 0;
    for line in file_vec_vec_char(filename) {
        let h = line.len() / 2;
        let c1: HashSet<char> = line[..h].iter().map(|&c|{c}).collect();
        let c2: HashSet<char> = line[h..].iter().map(|&c|{c}).collect();
        answer += priority(*c1.intersection(&c2).next().unwrap());
    }
    println!("part 1: {answer}");
    assert_eq!(answer, 8176);
}

fn part2(filename: &str) {
    let mut answer = 0;
    let mut lines = file_vec_vec_char(filename).into_iter();
    while let Some(line) = lines.next() {
        let mut intersection: HashSet<char> = line.into_iter().collect();
        for _ in 0..2 {
            let next_elf: HashSet<char> = lines.next().unwrap().into_iter().collect();
            intersection.retain(|c| { next_elf.contains(c) });
        }
        answer += priority(intersection.into_iter().next().unwrap());
    }
    println!("part 2: {answer}");
    assert_eq!(answer, 2689);
}

fn main() {
    part1("input/d3.txt");
    part2("input/d3.txt");
}
