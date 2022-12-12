use rusty_advent::*;

use priority_queue::PriorityQueue;
//use std::rc::Rc;
//use std::vec::Vec;
//use std::collections::VecDeque;
use std::collections::HashMap;
//use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct State {
    x: i64,
    y: i64,
    steps: i64,
}

fn heuristic(state: State, end_x: i64, end_y: i64) -> i64 {
    state.steps + (state.x - end_x).abs() + (state.y - end_y).abs()
}

fn successor_states(state: State, field: &Vec<Vec<u8>>) -> Vec<State> {
    let mut new_states = Vec::new();
    new_states.push(State { x: state.x + 1, y: state.y, steps: state.steps + 1 });
    new_states.push(State { x: state.x - 1, y: state.y, steps: state.steps + 1 });
    new_states.push(State { x: state.x, y: state.y + 1, steps: state.steps + 1 });
    new_states.push(State { x: state.x, y: state.y - 1, steps: state.steps + 1 });
    return new_states.into_iter().filter(|new|{
            if new.x < 0 {
                return false;
            }
            if new.y < 0 {
                return false;
            }
            if new.x >= field.len() as i64 {
                return false;
            }
            if new.y >= field[0].len() as i64 {
                return false;
            }
            let old_elev = field[state.x as usize][state.y as usize] as i64;
            let new_elev = field[new.x as usize][new.y as usize] as i64;
            if new_elev - old_elev > 1 {
                return false;
            }
            return true;
        }).collect();
}

fn part1(filename: &str) {
    let mut field: Vec<Vec<u8>> = file_vec_vec_char(filename).iter().map(|line|{line.iter().map(|ch|{*ch as u8}).collect()}).collect();
    let mut start_x: i64 = 0;
    let mut start_y: i64 = 0;
    let mut end_x: i64 = 0;
    let mut end_y: i64 = 0;
    for (x, row) in field.clone().iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 'S' as u8 {
                start_x = x as i64;
                start_y = y as i64;
                field[x][y] = 'a' as u8;
            }
            if *col == 'E' as u8 {
                end_x = x as i64;
                end_y = y as i64;
                field[x][y] = 'z' as u8;
            }
        }
    }
    let start_x = start_x;
    let start_y = start_y;
    let end_x = end_x;
    let end_y = end_y;
    let init_state = State { x: start_x, y: start_y, steps: 0 };
    let mut states = PriorityQueue::new();
    let mut prev_visits = HashMap::new();
    states.push(init_state, -heuristic(init_state, end_x, end_y));
    let answer;
    let mut cycles = 0;
    loop {
        let (state, _) = states.pop().unwrap();
        //dbg!(&state);
        if state.x == end_x && state.y == end_y {
            answer = state.steps;
            break;
        }
        prev_visits.insert((state.x, state.y), state.steps);
        for new_state in successor_states(state, &field) {
            //dbg!(&new_state);
            if !prev_visits.contains_key(&(new_state.x, new_state.y)) || *prev_visits.get(&(new_state.x, new_state.y)).unwrap() > new_state.steps {
                states.push(new_state, -heuristic(new_state, end_x, end_y));
            }
        }
        cycles += 1;
        if cycles % 1000000 == 0 {
            println!("len: {}", states.len());
            dbg!(state);
        }
    }
    println!("{}", answer);
}

fn part2(filename: &str) {
    let mut field: Vec<Vec<u8>> = file_vec_vec_char(filename).iter().map(|line|{line.iter().map(|ch|{*ch as u8}).collect()}).collect();
    let mut start_x: i64 = 0;
    let mut start_y: i64 = 0;
    let mut end_x: i64 = 0;
    let mut end_y: i64 = 0;
    for (x, row) in field.clone().iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 'S' as u8 {
                start_x = x as i64;
                start_y = y as i64;
                field[x][y] = 'a' as u8;
            }
            if *col == 'E' as u8 {
                end_x = x as i64;
                end_y = y as i64;
                field[x][y] = 'z' as u8;
            }
        }
    }
    let end_x = end_x;
    let end_y = end_y;

    let mut answer = 9999;
    for start_x in 0..field.len() {
        for start_y in 0..field[0].len() {
            if field[start_x][start_y] != 'a' as u8 {
                continue;
            }
            let start_x = start_x as i64;
            let start_y = start_y as i64;
            let init_state = State { x: start_x, y: start_y, steps: 0 };
            let mut states = PriorityQueue::new();
            let mut prev_visits = HashMap::new();
            states.push(init_state, -heuristic(init_state, end_x, end_y));
            let mut cycles = 0;
            loop {
                if let Some((state, _)) = states.pop() {
                    //dbg!(&state);
                    if state.x == end_x && state.y == end_y {
                        answer = std::cmp::min(answer, state.steps);
                        break;
                    }
                    prev_visits.insert((state.x, state.y), state.steps);
                    for new_state in successor_states(state, &field) {
                        //dbg!(&new_state);
                        if !prev_visits.contains_key(&(new_state.x, new_state.y)) || *prev_visits.get(&(new_state.x, new_state.y)).unwrap() > new_state.steps {
                            states.push(new_state, -heuristic(new_state, end_x, end_y));
                        }
                    }
                    cycles += 1;
                    if cycles % 1000000 == 0 {
                        println!("len: {}", states.len());
                        dbg!(state);
                    }
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", answer);
}


fn main() {
    part1("input/d12.ex");
    part1("input/d12.txt");
    part2("input/d12.ex");
    part2("input/d12.txt");
}
