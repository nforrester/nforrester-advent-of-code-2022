use rusty_advent::*;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

fn execute (prog: &Vec<(String, i64)>) -> Option<i64> {
    let mut pc: i64 = 0;
    let mut acc = 0;
    let mut prev_pcs = HashSet::new();
    loop {
        if pc >= prog.len() as i64 {
            return Some(acc);
        }
        if prev_pcs.contains(&pc) {
            return None;
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
}

fn main () {
    let mut prog: Vec<(String, i64)> = Vec::new();
    for line in file_vec_vec_word("input/practice-2020d8.txt") {
        prog.push((line[0].clone(), line[1].parse().unwrap()));
    }
    for idx in 0..prog.len() {
        if prog[idx].0 == "jmp" {
            prog[idx].0 = String::from("nop");
        } else if prog[idx].0 == "nop" {
            prog[idx].0 = String::from("jmp");
        }

        if let Some(acc) = execute(&prog) {
            println!("{}", acc);
            break;
        }

        if prog[idx].0 == "jmp" {
            prog[idx].0 = String::from("nop");
        } else if prog[idx].0 == "nop" {
            prog[idx].0 = String::from("jmp");
        }
    }
}
