use rusty_advent::*;
use std::collections::HashMap;

fn pwd_to_path(pwd: &Vec<String>) -> String {
    let mut s = String::from("/");
    for e in pwd {
        s.push_str(e);
        s.push_str("/");
    }
    return s;
}

fn file_to_path(pwd: &Vec<String>, file: &str) -> String {
    let mut s = pwd_to_path(pwd);
    s.push_str(file);
    return s;
}

fn build_filesystem(filename: &str) -> HashMap<String, u64> {
    let mut pwd = Vec::new();
    let mut fs: HashMap<String, u64> = HashMap::new();
    for line in file_vec_vec_word(filename) {
        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] == "/" {
                    pwd.clear();
                } else if line[2] == ".." {
                    pwd.pop();
                } else {
                    pwd.push(line[2].clone());
                }
            }
        } else {
            if line[0] == "dir" {
            } else {
                let size: u64 = line[0].parse().unwrap();
                let file = &line[1];
                let filepath = file_to_path(&pwd, &file);
                fs.insert(filepath, size);
                let mut dir = pwd.clone();
                dir.push("x".to_string());
                loop {
                    dir.pop();
                    let dirpath = pwd_to_path(&dir);
                    if !fs.contains_key(dirpath.as_str()) {
                        fs.insert(dirpath.clone(), 0);
                    }
                    fs.insert(dirpath.clone(), fs.get(dirpath.as_str()).unwrap() + size);
                    if dir.len() == 0 {
                        break;
                    }
                }
            }
        }
    }
    return fs;
}

fn part1(filename: &str) {
    let fs = build_filesystem(filename);
    let mut answer = 0;
    for (item, size) in fs {
        if item.ends_with("/") {
            if size <= 100000 {
                answer += size;
            }
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 1348005);
}

fn part2(filename: &str) {
    let fs = build_filesystem(filename);
    let usage = fs.get("/").unwrap();
    let total = 70000000;
    let need = 30000000 - (total - usage);
    let mut answer = 70000000;
    for (item, size) in fs {
        if item.ends_with("/") {
            if size >= need {
                if size < answer {
                    answer = size;
                }
            }
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 12785886);
}

fn main() {
    part1("input/d7.txt");
    part2("input/d7.txt");
}
