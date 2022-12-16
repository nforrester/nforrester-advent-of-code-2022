use std::vec::Vec;
use std::hash::Hash;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::ops::Sub;
use std::ops::Add;
use std::ops::Neg;
use num::traits::Zero;

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

    #[test]
    fn test_slice_of_strs() {
        let line: Vec<String> = vec_word("If true: throw to monkey 5");
        let mt: usize = match slice_of_strs!(line) {
            ["If", "true:", "throw", "to", "monkey", x] => x.parse().unwrap(),
            _ => panic!(""),
        };
        assert_eq!(mt, 5);
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
    let r = regex::Regex::new(re).unwrap();
    ReCap {
        captures: r.captures(string).unwrap(),
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

#[macro_export]
macro_rules! slice_of_strs {
    ( $x:expr ) => {
        $x.iter().map(String::as_str).collect::<Vec<_>>().as_slice()
    }
}

pub trait AStarLocation {
    type Scenario;
    type Cost;

    fn check_success(&self, scenario: &Self::Scenario) -> bool;
    fn nexts_with_incremental_costs(&self, scenario: &Self::Scenario) -> Vec<(Self, Self::Cost)>
        where Self: Sized;
    fn heuristic_remaining_cost(&self, scenario: &Self::Scenario) -> Self::Cost;
}

pub fn a_star<Location>(init_state: &Location, scenario: &Location::Scenario) -> Option<(Location, Location::Cost)>
where Location: AStarLocation + Hash + Eq + Copy,
      <Location as AStarLocation>::Cost:
          Hash +
          Ord +
          Copy +
          Zero +
          Add<Location::Cost, Output=Location::Cost> +
          Sub<Location::Cost, Output=Location::Cost> +
          Neg<Output=Location::Cost>,
{
    let mut to_visit = PriorityQueue::new();
    let mut already_visited: HashMap<Location, Location::Cost> = HashMap::new();
    let initial_estimate: Location::Cost = -init_state.heuristic_remaining_cost(scenario);
    to_visit.push((*init_state, Location::Cost::zero()), initial_estimate);
    loop {
        if let Some(((state, cost_of_state), _)) = to_visit.pop() {
            if state.check_success(scenario) {
                return Some((state, cost_of_state));
            }
            already_visited.insert(state, cost_of_state);
            for (next, incremental_cost) in state.nexts_with_incremental_costs(scenario) {
                let cost_of_next: Location::Cost = cost_of_state + incremental_cost;
                if !already_visited.contains_key(&next) || *already_visited.get(&next).unwrap() > cost_of_next {
                    let estimate: Location::Cost = -cost_of_next - next.heuristic_remaining_cost(scenario);
                    to_visit.push((next, cost_of_next), estimate);
                }
            }
        } else {
            return None;
        }
    }
}
