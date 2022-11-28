use std::vec::Vec;

// string.split_whitespace()
// string.split("separator")

pub fn vec_char(string: &str) -> Vec<char> {
    Vec::from_iter(string.chars())
}

pub fn file_lines(filename: &str) -> Vec<String> {
    let mut ret = Vec::new();
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        ret.push(String::from(line));
    }
    return ret;
}

pub fn file_vec_char_char(filename: &str) -> Vec<Vec<char>> {
    let mut ret = Vec::new();
    for line in file_lines(filename) {
        ret.push(vec_char(line.as_str()));
    }
    return ret;
}

pub struct ReCap<'captures_lifetime> {
    captures: regex::Captures<'captures_lifetime>,
}

impl<'captures_lifetime> ReCap<'captures_lifetime> {
    pub fn recap(re: &str, string: &'captures_lifetime str) -> ReCap<'captures_lifetime> {
        ReCap {
            captures: regex::Regex::new(re).unwrap().captures(string).unwrap(),
        }
    }

    pub fn getstr(&self, name: &str) -> &str {
        self.captures.name(name).unwrap().as_str()
    }

    pub fn get<Parsed: std::str::FromStr>(&self, name: &str) -> Parsed where <Parsed as std::str::FromStr>::Err: std::fmt::Debug {
        self.getstr(name).parse().unwrap()
    }
}

pub fn recap<'captures_lifetime>(re: &str, string: &'captures_lifetime str) -> ReCap<'captures_lifetime> {
    ReCap::recap(re, string)
}
