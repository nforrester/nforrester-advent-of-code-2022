use rusty_advent::*;

use std::cmp::min;
use std::cmp::max;

struct ElfRange {
    lo: i64,
    hi: i64,
}

impl ElfRange {
    fn inside_of(&self, other: &Self) -> bool {
        other.lo <= self.lo && self.hi <= other.hi
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        (other.lo <= self.lo && self.lo <= other.hi) || (other.lo <= self.hi && self.hi <= other.hi) || other.inside_of(self)
    }

    fn merge(&self, other: &Self) -> ElfRange {
        assert!(self.overlaps_with(other));
        ElfRange{lo: min(self.lo, other.lo), hi: max(self.hi, other.hi)}
    }
}

type Sensors = Vec<((i64, i64), (i64, i64))>;

fn parse_input(filename: &str) -> Sensors {
    let mut sensors = Vec::new();
    for line in file_lines(filename) {
        let c = recap(r"Sensor at x=(?P<sx>-?[0-9]+), y=(?P<sy>-?[0-9]+): closest beacon is at x=(?P<bx>-?[0-9]+), y=(?P<by>-?[0-9]+)", line.as_str());
        let sx: i64 = c.get("sx");
        let sy: i64 = c.get("sy");
        let bx: i64 = c.get("bx");
        let by: i64 = c.get("by");
        sensors.push(((sx, sy), (bx, by)));
    }
    return sensors;
}

fn dist((ax, ay): (i64, i64), (bx, by): (i64, i64)) -> i64 {
    (ax-bx).abs() + (ay-by).abs()
}

fn blocked_ranged_in_row(sensors: &Sensors, y: i64) -> Vec<ElfRange> {
    let mut blocked_ranges: Vec<ElfRange> = Vec::new();
    for (s, b) in sensors {
        let d = dist(*s, *b);
        let x_radius = d - (y-s.1).abs();
        if x_radius >= 0 {
            let blocked_min = s.0 - x_radius;
            let blocked_max = s.0 + x_radius;
            let r = ElfRange{lo: blocked_min, hi: blocked_max};
            let mut merged = false;
            for i in 0..blocked_ranges.len() {
                if blocked_ranges[i].overlaps_with(&r) {
                    blocked_ranges[i] = blocked_ranges[i].merge(&r);
                    merged = true;
                    break;
                }
            }
            if !merged {
                blocked_ranges.push(r);
            }
        }
    }
    loop {
        let mut no_overlaps = true;
        'this_iter: for i in 0..blocked_ranges.len() {
            for j in 0..blocked_ranges.len() {
                if i == j {
                    continue;
                }
                if blocked_ranges[i].overlaps_with(&blocked_ranges[j]) {
                    blocked_ranges[i] = blocked_ranges[i].merge(&blocked_ranges[j]);
                    blocked_ranges.remove(j);
                    no_overlaps = false;
                    break 'this_iter;
                }
            }
        }
        if no_overlaps {
            break;
        }
    }
    return blocked_ranges;
}

fn part1(filename: &str, y: i64) {
    let sensors = parse_input(filename);
    let blocked_ranges = blocked_ranged_in_row(&sensors, y);
    let answer = blocked_ranges.iter().map(|r|{r.hi-r.lo}).sum::<i64>();
    println!("{}", answer);
    assert_eq!(answer, 5083287);
}

fn part2(filename: &str, max_coord: i64) {
    let sensors = parse_input(filename);
    let mut answer = 0;
    'outer: for y in 0..=max_coord {
        let blocked_ranges = blocked_ranged_in_row(&sensors, y);
        let mut x = 0;
        let mut r = 0;
        while blocked_ranges[r].hi < x {
            r += 1;
        }
        while x <= max_coord {
            if r >= blocked_ranges.len() || x < blocked_ranges[r].lo  {
                answer = x * 4000000 + y;
                break 'outer;
            } else {
                x = blocked_ranges[r].hi + 1;
                r += 1;
            }
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 13134039205729);
}


fn main() {
    part1("input/d15.txt", 2000000);
    part2("input/d15.txt", 4000000);
}
