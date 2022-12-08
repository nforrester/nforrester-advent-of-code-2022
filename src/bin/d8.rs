use rusty_advent::*;

fn part1(filename: &str) {
    let mut answer = 0;
    let forest = file_vec_vec_char(filename);
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            let mut visible_n = true;
            let mut visible_s = true;
            let mut visible_e = true;
            let mut visible_w = true;
            for n in 0..i {
                if forest[n][j] >= forest[i][j] {
                    visible_w = false;
                    break;
                }
            }
            for n in i+1..forest.len() {
                if forest[n][j] >= forest[i][j] {
                    visible_e = false;
                    break;
                }
            }
            for n in 0..j {
                if forest[i][n] >= forest[i][j] {
                    visible_n = false;
                    break;
                }
            }
            for n in j+1..forest[0].len() {
                if forest[i][n] >= forest[i][j] {
                    visible_s = false;
                    break;
                }
            }
            if visible_n || visible_s || visible_e || visible_w {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 1818);
}

fn part2(filename: &str) {
    let mut answer = 0;
    let forest = file_vec_vec_char(filename);
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            let mut view_n = 0;
            let mut view_s = 0;
            let mut view_e = 0;
            let mut view_w = 0;
            for n in (0..i).rev() {
                view_w += 1;
                if forest[n][j] >= forest[i][j] {
                    break;
                }
            }
            for n in i+1..forest.len() {
                view_e += 1;
                if forest[n][j] >= forest[i][j] {
                    break;
                }
            }
            for n in (0..j).rev() {
                view_n += 1;
                if forest[i][n] >= forest[i][j] {
                    break;
                }
            }
            for n in j+1..forest[0].len() {
                view_s += 1;
                if forest[i][n] >= forest[i][j] {
                    break;
                }
            }
            let score = view_n * view_s * view_e * view_w;
            if score > answer {
                answer = score;
            }
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 368368);
}

fn main() {
    part1("input/d8.txt");
    part2("input/d8.txt");
}
