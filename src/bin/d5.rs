use rusty_advent::*;

fn init() -> Vec<Vec<char>> {
    vec![
        vec!['R','G','J','B','T','V','Z'],
        vec!['J','R','V','L'],
        vec!['S','Q','F'],
        vec!['Z','H','N','L','F','V','Q','G'],
        vec!['R','Q','T','J','C','S','M','W'],
        vec!['S','W','T','C','H','F'],
        vec!['D','Z','C','V','F','N','J'],
        vec!['L','G','Z','D','W','R','F','Q'],
        vec!['J','B','W','V','P'],
    ]
}

fn part1(filename: &str) {
    let mut stacks: Vec<Vec<char>> = init();

    for line in file_vec_vec_word(filename) {
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
    let mut stacks: Vec<Vec<char>> = init();

    for line in file_vec_vec_word(filename) {
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
