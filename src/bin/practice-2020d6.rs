use rusty_advent::*;
use std::collections::HashMap;

fn process_questions(questions: &mut HashMap<char, i64>,
                     group_size: &mut i64,
                     yes_for_anyone_sum: &mut i64,
                     yes_for_everyone_sum: &mut i64) {
    for (_, c) in questions.iter() {
        *yes_for_anyone_sum += 1;
        if *c == *group_size {
            *yes_for_everyone_sum += 1;
        }
    }
    *questions = HashMap::new();
    *group_size = 0;
}

fn main () {
    let mut yes_for_anyone_sum = 0;
    let mut yes_for_everyone_sum = 0;
    let mut group_size = 0;
    let mut questions = HashMap::new();
    for line in file_vec_vec_char("input/practice-2020d6.txt") {
        if line.len() == 0 {
            process_questions(&mut questions,
                              &mut group_size,
                              &mut yes_for_anyone_sum,
                              &mut yes_for_everyone_sum);
            continue;
        }
        group_size += 1;
        for q in line {
            if !questions.contains_key(&q) {
                questions.insert(q, 1);
            } else {
                questions.insert(q, questions.get(&q).unwrap() + 1);
            }
        }
    }
    process_questions(&mut questions,
                      &mut group_size,
                      &mut yes_for_anyone_sum,
                      &mut yes_for_everyone_sum);
    println!("part 1: {yes_for_anyone_sum}");
    println!("part 2: {yes_for_everyone_sum}");
    assert_eq!(yes_for_anyone_sum, 6504);
    assert_eq!(yes_for_everyone_sum, 3351);
}
