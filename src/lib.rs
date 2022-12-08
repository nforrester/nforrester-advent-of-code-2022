use std::vec::Vec;
//use std::collections::HashSet;
//use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_vec_stuff () {
        assert!(vec_char("hello")[4] == 'o');
        assert!(vec_word("hello world")[1] == "world");
        assert!(vec_by_sep("hello,world", ",")[1] == "world");
    }

    #[test]
    fn test_file_stuff () {
        assert!(file_lines("input/test.txt")[0] == "// test!");
        assert!(file_vec_vec_char("input/test.txt")[0][1] == '/');
        assert!(file_vec_vec_word("input/test.txt")[0][1] == "test!");
        assert!(file_vec_vec_by_sep("input/test.txt", "t")[0][1] == "es");
    }

    #[test]
    fn test_recap () {
        let c = recap(r"^(?P<a>\S+) (?P<b>\S+) (?P<c>\S+)$", "hello 36 false");
        assert!(c.getstr("a") == "hello");
        assert!(c.get::<String>("a") == "hello");
        assert!(c.get::<u64>("b") == 36);
        assert!(c.get::<f64>("b") == 36.0);
        assert!(c.get::<bool>("c") == false);
    }

    #[test]
    fn test_transpose() {
        let m1 = vec![
            vec![1, 2, 3, 4],
            vec![3, 4, 5, 6],
            vec![6, 7, 8, 9],
        ];
        let m2 = vec![
            vec![1, 3, 6],
            vec![2, 4, 7],
            vec![3, 5, 8],
            vec![4, 6, 9],
        ];
        assert_eq!(m1, transpose(&m2));
        assert_eq!(m2, transpose(&m1));

        let r1 = vec![
            vec![1, 2, 3, 4],
            vec![3, 4],
            vec![6, 7, 8],
        ];
        let r2 = vec![
            vec![1, 3, 6],
            vec![2, 4, 7],
            vec![3, 0, 8],
            vec![4, 0, 0],
        ];
        assert_eq!(r2, transpose_ragged(&r1, 0));
    }
}

// string.split_whitespace()
// string.split("separator")

pub fn vec_word(string: &str) -> Vec<String> {
    string.split_whitespace().map(|word|{word.to_string()}).collect()
}

pub fn vec_char(string: &str) -> Vec<char> {
    Vec::from_iter(string.chars())
}

pub fn vec_by_sep(string: &str, sep: &str) -> Vec<String> {
    string.split(sep).map(|word|{word.to_string()}).collect()
}

pub fn file_string(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

pub fn file_lines(filename: &str) -> Vec<String> {
    file_string(filename).lines().map(|line|{line.to_string()}).collect()
}

pub fn file_vec_vec_word(filename: &str) -> Vec<Vec<String>> {
    file_lines(filename).iter().map(|line|{vec_word(line.as_str())}).collect()
}

pub fn file_vec_vec_char(filename: &str) -> Vec<Vec<char>> {
    file_lines(filename).iter().map(|line|{vec_char(line.as_str())}).collect()
}

pub fn file_vec_vec_by_sep(filename: &str, sep: &str) -> Vec<Vec<String>> {
    file_lines(filename).iter().map(|line|{vec_by_sep(line.as_str(), sep)}).collect()
}

pub struct ReCap<'captures_lifetime> {
    captures: regex::Captures<'captures_lifetime>,
}

impl<'captures_lifetime> ReCap<'captures_lifetime> {
    pub fn getstr(&self, name: &str) -> &str {
        self.captures.name(name).unwrap().as_str()
    }

    pub fn get<Parsed: std::str::FromStr>(&self, name: &str) -> Parsed where <Parsed as std::str::FromStr>::Err: std::fmt::Debug {
        self.getstr(name).parse().unwrap()
    }
}

pub fn recap<'captures_lifetime>(re: &str, string: &'captures_lifetime str) -> ReCap<'captures_lifetime> {
    ReCap {
        captures: regex::Regex::new(re).unwrap().captures(string).unwrap(),
    }
}

pub fn transpose<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|column|{
            matrix.iter().map(|row|{
                row[column]
            }).collect()
        }).collect()
}

pub fn transpose_ragged<T: Copy>(matrix: &Vec<Vec<T>>, fill: T) -> Vec<Vec<T>> {
    (0..matrix.iter().map(|row|{row.len()}).max().unwrap())
        .map(|column|{
            matrix.iter().map(|row|{
                    if column < row.len() {
                        row[column]
                    } else {
                        fill
                    }
                }).collect()
        }).collect()
}
