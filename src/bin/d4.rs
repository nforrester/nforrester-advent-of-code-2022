use rusty_advent::*;

use std::str::FromStr;

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
}

impl FromStr for ElfRange {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("-");
        let lo = s.next().ok_or(false)?.parse().map_err(|_|{false})?;
        let hi = s.next().ok_or(false)?.parse().map_err(|_|{false})?;
        return Ok(ElfRange { lo, hi });
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in file_vec_vec_by_sep("input/d4.txt", ",") {
        let a: ElfRange = line[0].parse().unwrap();
        let b: ElfRange = line[1].parse().unwrap();

        if a.inside_of(&b) || b.inside_of(&a) {
            part1 += 1;
        }
        if a.overlaps_with(&b) {
            part2 += 1;
        }
    }
    println!("part 1: {part1}");
    println!("part 2: {part2}");
    assert_eq!(part1, 475);
    assert_eq!(part2, 825);
}
