use rusty_advent::*;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

fn execute (prog: &Vec<(String, i64)>) -> Result<i64, i64> {
    let mut pc: i64 = 0;
    let mut acc = 0;
    let mut prev_pcs = HashSet::new();
    loop {
        if pc >= prog.len() as i64 {
            return Ok(acc);
        }
        if prev_pcs.contains(&pc) {
            return Err(acc);
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

    if let Err(acc) = execute(&prog) {
        println!("part 1: {acc}");
        assert_eq!(acc, 1753);
    } else {
        panic!("The original code terminates.");
    }

    for idx in 0..prog.len() {
        if prog[idx].0 == "jmp" {
            prog[idx].0 = String::from("nop");
        } else if prog[idx].0 == "nop" {
            prog[idx].0 = String::from("jmp");
        }

        if let Ok(acc) = execute(&prog) {
            println!("part 2: {acc}");
            assert_eq!(acc, 733);
            break;
        }

        if prog[idx].0 == "jmp" {
            prog[idx].0 = String::from("nop");
        } else if prog[idx].0 == "nop" {
            prog[idx].0 = String::from("jmp");
        }
    }
}
