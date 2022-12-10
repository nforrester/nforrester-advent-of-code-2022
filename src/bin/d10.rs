use rusty_advent::*;

//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::cmp::max;
//use std::cmp::min;

fn part1(filename: &str) {
    let mut answer = 0;
    let mut x: i64 = 1;
    let mut cycle = 0;
    for words in file_vec_vec_word(filename) {
        if words[0] == "noop"
        {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                let y = cycle * x;
                answer += y;
            }
        }
        else
        {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                let y = cycle * x;
                answer += y;
            }
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                let y = cycle * x;
                answer += y;
            }
            x += words[1].parse::<i64>().unwrap();
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 15260);
}

fn part2(filename: &str) {
    let mut answer = "\n".to_string();
    let mut x: i64 = 1;
    let mut pix = 0;
    for words in file_vec_vec_word(filename) {
        if words[0] == "noop"
        {
            if (x-pix).abs() <= 1 {
                answer.push('#');
            } else {
                answer.push('.');
            }
            pix += 1;
            if pix == 40 {
                pix = 0;
                answer.push('\n');
            }
        }
        else
        {
            if (x-pix).abs() <= 1 {
                answer.push('#');
            } else {
                answer.push('.');
            }
            pix += 1;
            if pix == 40 {
                pix = 0;
                answer.push('\n');
            }
            if (x-pix).abs() <= 1 {
                answer.push('#');
            } else {
                answer.push('.');
            }
            pix += 1;
            if pix == 40 {
                pix = 0;
                answer.push('\n');
            }
            x += words[1].parse::<i64>().unwrap();
        }
    }
    println!("{}", answer);
    assert_eq!(answer, "
###...##..#..#.####..##..#....#..#..##..
#..#.#..#.#..#.#....#..#.#....#..#.#..#.
#..#.#....####.###..#....#....#..#.#....
###..#.##.#..#.#....#.##.#....#..#.#.##.
#....#..#.#..#.#....#..#.#....#..#.#..#.
#.....###.#..#.#.....###.####..##...###.
");
}

fn main() {
    part1("input/d10.txt");
    part2("input/d10.txt");
}
