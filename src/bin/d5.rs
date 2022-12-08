use rusty_advent::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn parse_stacks(filename: &str) -> Vec<Vec<char>> {
    // Read the file up until the first blank line. This extracts the image of the stacks.
    let stack_image: Vec<Vec<char>> =
        BufReader::new(File::open(filename).unwrap()).lines().map(|s|{s.unwrap().chars().collect()}).take_while(|row: &Vec<_>|{row.len() > 0}).collect();

    (0..stack_image[0].len())                                                    // For each column,
        .map(|col_idx|{ stack_image.iter().map(|row|{row[col_idx]}).collect() }) // collect it into a vector,
        .map(|col: Vec<char>|{ col.into_iter().rev().collect() })                // reverse it so the bottom of the stack is at the top,
        .filter(|col: &Vec<char>|{ col.len() > 0 && col[0] != ' ' })             // drop all the columns that start with a space (the ones that aren't stacks),
        .map(|row|{
                row[1..]                                                         // drop the stack number,
                .iter()
                .map(|x|{*x}).take_while(|ch|{*ch != ' '})                       // drop the spaces at the bottom,
                .collect()
            })
        .collect()                                                               // and that's it!
}

fn part1(filename: &str) {
    let mut stacks: Vec<Vec<char>> = parse_stacks(filename);

    for line in file_vec_vec_word(filename) {
        if line.len() == 0 || line[0] != "move" {
            continue;
        }
        let num_crates = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        for _ in 0..num_crates {
            let top = stacks[from].len() - 1;
            let this_crate = stacks[from][top];
            stacks[to].push(this_crate);
            stacks[from].pop();
        }
    }
    let mut answer = String::new();
    for s in stacks {
        answer.push(s[s.len()-1]);
    }
    println!("{}", answer);
    assert_eq!(answer, "ZSQVCCJLL");
}

fn part2(filename: &str) {
    let mut stacks: Vec<Vec<char>> = parse_stacks(filename);

    for line in file_vec_vec_word(filename) {
        if line.len() == 0 || line[0] != "move" {
            continue;
        }
        let num_crates = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        for idx in stacks[from].len()-num_crates..stacks[from].len() {
            let this_crate = stacks[from][idx];
            stacks[to].push(this_crate);
        }
        for _ in stacks[from].len()-num_crates..stacks[from].len() {
            stacks[from].pop();
        }
    }
    let mut answer = String::new();
    for s in stacks {
        answer.push(s[s.len()-1]);
    }
    println!("{}", answer);
    assert_eq!(answer, "QZFJRWHGS");
}


fn main() {
    part1("input/d5.txt");
    part2("input/d5.txt");
}
