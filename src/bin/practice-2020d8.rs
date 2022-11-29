use rusty_advent::*;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

fn main () {
    let mut prog: Vec<(String, i64)> = Vec::new();
    for line in file_vec_vec_word("input/practice-2020d8.txt") {
        prog.push((line[0].clone(), line[1].parse().unwrap()));
    }
    let prog = prog;
    let mut pc = 0;
    let mut acc = 0;
    let mut prev_pcs = HashSet::new();
    loop {
        if prev_pcs.contains(&pc) {
            break;
        }
        prev_pcs.insert(pc);
        let op = &prog[pc as usize].0;
        let v = prog[pc as usize].1;
        if op == "jmp" {
            pc += v;
        } else if op == "acc" {
            acc += v;
            pc += 1;
        } else {
            assert!(op == "nop");
            pc += 1;
        }
    }
    println!("{}", acc);
}
