use rusty_advent::*;

type Field = Vec<Vec<u8>>;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl AStarLocation for Point {
    type Scenario = (Field, Point);
    type Cost = i64;

    fn check_success(&self, (_, end): &(Field, Point)) -> bool {
        self == end
    }

    fn nexts_with_incremental_costs(&self, (field, _): &(Field, Point)) -> Vec<(Point, i64)> {
        let mut new_states = Vec::new();
        new_states.push((Point{x: self.x + 1, y: self.y    }, 1));
        new_states.push((Point{x: self.x - 1, y: self.y    }, 1));
        new_states.push((Point{x: self.x,     y: self.y + 1}, 1));
        new_states.push((Point{x: self.x,     y: self.y - 1}, 1));
        return new_states.into_iter().filter(|(new, _)|{
                if new.x < 0 {
                    return false;
                }
                if new.y < 0 {
                    return false;
                }
                if new.x >= field.len() as i64 {
                    return false;
                }
                if new.y >= field[0].len() as i64 {
                    return false;
                }
                let old_elev = field[self.x as usize][self.y as usize] as i64;
                let new_elev = field[new.x as usize][new.y as usize] as i64;
                if new_elev - old_elev > 1 {
                    return false;
                }
                return true;
            }).collect();
    }

    fn heuristic_remaining_cost(&self, (_, end): &(Field, Point)) -> i64 {
        (self.x - end.x).abs() + (self.y - end.y).abs()
    }

    fn zero_cost() -> i64 {
        0
    }
}

fn parse_field(filename: &str) -> (Field, Point, Point) {
    let mut field: Field = file_vec_vec_char(filename).iter().map(|line|{line.iter().map(|ch|{*ch as u8}).collect()}).collect();
    let mut start = Point{x: 0, y: 0};
    let mut end = Point{x: 0, y: 0};
    for (x, row) in field.clone().iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 'S' as u8 {
                start.x = x as i64;
                start.y = y as i64;
                field[x][y] = 'a' as u8;
            }
            if *col == 'E' as u8 {
                end.x = x as i64;
                end.y = y as i64;
                field[x][y] = 'z' as u8;
            }
        }
    }
    return (field, start, end);
}

fn find_shortest_path_len(field: &Field, start: Point, end: Point) -> Option<i64> {
    return Some(a_star(&start, &(field.clone(), end))?.1);
}

fn part1(filename: &str) {
    let (field, start, end) = parse_field(filename);
    let answer = find_shortest_path_len(&field, start, end).unwrap();
    println!("{}", answer);
    assert_eq!(answer, 490);
}

fn part2(filename: &str) {
    let (field, _, end) = parse_field(filename);

    let mut answer = 9999;
    for start_x in 0..field.len() {
        for start_y in 0..field[0].len() {
            if field[start_x][start_y] != 'a' as u8 {
                continue;
            }
            let start = Point{x: start_x as i64, y: start_y as i64};
            if let Some(len) = find_shortest_path_len(&field, start, end) {
                answer = std::cmp::min(answer, len);
            }
        }
    }
    println!("{}", answer);
    assert_eq!(answer, 488);
}


fn main() {
    part1("input/d12.txt");
    part2("input/d12.txt");
}
