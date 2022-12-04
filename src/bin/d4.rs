use rusty_advent::*;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in file_lines("input/d4.txt") {
        let cap = recap(r"(?P<a>\d+)-(?P<b>\d+),(?P<c>\d+)-(?P<d>\d+)", line.as_str());
        let a = cap.get::<i64>("a");
        let b = cap.get::<i64>("b");
        let c = cap.get::<i64>("c");
        let d = cap.get::<i64>("d");

        if a <= c && d <= b {
            part1 += 1;
            part2 += 1;
        } else if c <= a && b <= d {
            part1 += 1;
            part2 += 1;
        } else if c <= a && a <= d {
            part2 += 1;
        } else if c <= b && b <= d {
            part2 += 1;
        } else if a <= c && c <= b {
            part2 += 1;
        } else if a <= d && d <= b {
            part2 += 1;
        }
    }
    println!("part 1: {part1}");
    println!("part 2: {part2}");
    assert_eq!(part1, 475);
    assert_eq!(part2, 825);
}
