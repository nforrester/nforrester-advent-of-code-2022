use rusty_advent::*;
use std::collections::HashMap;

fn main () {
    let mut sum_of_counts = 0;
    let mut group_size = 0;
    let mut questions = HashMap::new();
    for line in file_vec_vec_char("input/practice-2020d6.txt") {
        if line.len() == 0 {
            for (q, c) in questions.iter() {
                if *c == group_size {
                    sum_of_counts += 1
                }
            }
            questions = HashMap::new();
            group_size = 0;
            continue;
        }
        group_size += 1;
        for q in line {
            if !questions.contains_key(&q) {
                questions.insert(q, 1);
            } else {
                questions.insert(q, questions.get(&q).unwrap() + 1usize);
            }
        }
    }
    for (q, c) in questions.iter() {
        if *c == group_size {
            sum_of_counts += 1
        }
    }
    println!("{}", sum_of_counts);
}
