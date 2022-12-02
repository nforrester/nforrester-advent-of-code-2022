use rusty_advent::*;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Move { Rock, Paper, Scissors }
use Move::Rock;
use Move::Paper;
use Move::Scissors;

impl Move {
    fn from(s: &str) -> Self {
        match s {
            "A" => Rock,
            "X" => Rock,
            "B" => Paper,
            "Y" => Paper,
            "C" => Scissors,
            "Z" => Scissors,
            _ => panic!("Invalid move: {s}"),
        }
    }

    fn score(&self) -> i64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Outcome { Loss, Draw, Win }
use Outcome::Loss;
use Outcome::Draw;
use Outcome::Win;

impl Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Invalid outcome: {s}"),
        }
    }

    fn score(&self) -> i64 {
        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

fn decide_outcome(opponent_move: Move, my_move: Move) -> Outcome {
    match (opponent_move.score() - my_move.score()).rem_euclid(3) {
        0 => Draw,
        1 => Loss,
        2 => Win,
        _ => panic!("Modular arithmetic is broken!"),
    }
}

fn decide_my_move(opponent_move: Move, outcome: Outcome) -> Move {
    for my_move in [Rock, Paper, Scissors] {
        if decide_outcome(opponent_move, my_move) == outcome {
            return my_move;
        }
    }
    panic!("The desired outcome should be possible.");
}

fn part1(opponent_move: Move, my_move: Move) -> i64 {
    my_move.score() + decide_outcome(opponent_move, my_move).score()
}

fn part2(opponent_move: Move, outcome: Outcome) -> i64 {
    decide_my_move(opponent_move, outcome).score() + outcome.score()
}

fn main() {
	let mut total_part1 = 0;
	let mut total_part2 = 0;
    for line in file_vec_vec_word("input/d2.txt") {
        let opponent_move = Move::from(&line[0]);
        total_part1 += part1(opponent_move, Move::from(&line[1]));
        total_part2 += part2(opponent_move, Outcome::from(&line[1]));
    }
    println!("part 1: {}", total_part1);
    println!("part 2: {}", total_part2);
    assert_eq!(total_part1, 13682);
    assert_eq!(total_part2, 12881);
}
