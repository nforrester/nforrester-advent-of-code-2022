use rusty_advent::*;

use std::cmp::min;
use std::cmp::max;
use std::iter::zip;

type Cave = Vec<Vec<char>>;

fn u(x: i64) -> usize {x as usize}

fn parse_input(filename: &str, start_x: i64, start_y: i64) -> (Cave, usize, usize) {
    let input = file_vec_vec_by_sep(filename, " -> ");
    let mut traces = Vec::new();
    for line in input {
        let mut trace: Vec<(i64, i64)> = Vec::new();
        for point in line {
            match slice_of_strs!(vec_by_sep(point.as_str(), ",")) {
                [x, y] => {
                    trace.push((x.parse().unwrap(), y.parse().unwrap()));
                },
                _ => panic!(""),
            }
        }
        traces.push(trace);
    }
    let traces = traces;
    
    let mut min_x: i64 = start_x;
    let mut min_y: i64 = start_y;
    let mut max_x: i64 = start_x;
    let mut max_y: i64 = start_y;
    for trace in traces.clone() {
        for (x, y) in trace {
            min_x = min(min_x, x);
            min_y = min(min_y, y);
            max_x = max(max_x, x);
            max_y = max(max_y, y);
        }
    }
    max_y += 1;
    min_x = min(min_x, start_x - (max_y - min_y));
    max_x = max(max_x, start_x + (max_y - min_y));

    let mut cave: Cave = (min_x..=max_x).map(|_|{vec!['.'; (max_y-min_y+1) as usize]}).collect();

    for trace in traces.clone() {
        for i in 0..(trace.len()-1) {
            let mut x = trace[i].0 - min_x;
            let mut y = trace[i].1 - min_y;
            let x2 = trace[i+1].0 - min_x;
            let y2 = trace[i+1].1 - min_y;
            cave[u(x)][u(y)] = '#';
            while x != x2 || y != y2 {
                x += max(-1, min(1, x2-x));
                y += max(-1, min(1, y2-y));
                cave[u(x)][u(y)] = '#';
            }
        }
    }

    return (cave, (start_x-min_x) as usize, (start_y-min_y) as usize);
}

fn cave_eq(ls: &Cave, rs: &Cave) -> bool {
    for (l, r) in zip(ls, rs) {
        for (a, b) in zip(l, r) {
            if a != b {
                return false;
            }
        }
    }
    return true;
}

fn flow_sand(cave: &mut Cave, start_x: usize, start_y: usize, has_floor: bool) -> Option<i64> {
    let prev_cave = cave.clone();

    let mut resting_count = 0;

    cave[start_x][start_y] = 'o';
    for y in (0..cave[0].len()).rev() {
        for x in 0..cave.len() {
            if cave[x][y] == 'o' {
                if y + 1 == cave[x].len() {
                    if has_floor {
                        resting_count += 1;
                    } else {
                        cave[x][y] = '.';
                    }
                } else if cave[x][y+1] == '.' {
                    cave[x][y+1] = 'o';
                    cave[x][y] = '.';
                } else if x == 0 {
                    cave[x][y] = '.';
                } else if cave[x-1][y+1] == '.' {
                    cave[x-1][y+1] = 'o';
                    cave[x][y] = '.';
                } else if x == cave.len() - 1 {
                    cave[x][y] = '.';
                } else if cave[x+1][y+1] == '.' {
                    cave[x+1][y+1] = 'o';
                    cave[x][y] = '.';
                } else {
                    resting_count += 1
                }
            }
        }
    }

    if cave_eq(&cave, &prev_cave) {
        return Some(resting_count);
    }
    return None;
}

fn solve(filename: &str, has_floor: bool) -> i64 {
    let start_x: i64 = 500;
    let start_y: i64 = 0;

    let (cave, start_x, start_y) = parse_input(filename, start_x, start_y);
    let mut cave = cave.clone();
    loop {
        match flow_sand(&mut cave, start_x, start_y, has_floor) {
            Some(x) => return x,
            None => continue,
        }
    }
}

fn part1(filename: &str) {
    let answer = solve(filename, false);
    println!("{}", answer);
    assert_eq!(answer, 719);
}

fn part2(filename: &str) {
    let answer = solve(filename, true);
    println!("{}", answer);
    assert_eq!(answer, 23390);
}


fn main() {
    part1("input/d14.txt");
    part2("input/d14.txt");
}
