use rusty_advent::*;

//use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use priority_queue::PriorityQueue;

type Map = HashMap<i64, (i64, Vec<i64>, u64)>;

type Shortcuts = HashMap<i64, HashMap<i64, i64>>;

fn name_to_number(name: &str) -> i64 {
    let mut chars = name.chars();
    let a = chars.next().unwrap() as u8 as i64;
    let b = chars.next().unwrap() as u8 as i64;
    a * 1000 + b
}

fn parse_input(filename: &str) -> Map {
    let mut map = HashMap::new();
    for line in file_lines(filename) {
        let c = recap(r"^Valve (?P<room>[^ ]+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)$", line.as_str());
        let room: i64 = name_to_number(c.getstr("room"));
        let rate: i64 = c.get("rate");
        let tunnels: String = c.get("tunnels");
        let tunnels: Vec<i64> = vec_by_sep(tunnels.as_str(), ", ").iter().map(String::as_str).map(name_to_number).collect();
        map.insert(room, (rate, tunnels, 1u64 << map.len()));
    }
    return map;
}

#[derive(Hash, Eq, Copy, PartialEq, Clone)]
struct Room {
    name: i64,
}

impl AStarLocation for Room {
    type Scenario = (Map, i64);
    type Cost = i64;

    fn check_success(&self, scn: &Self::Scenario) -> bool {
        self.name == scn.1
    }

    fn nexts_with_incremental_costs(&self, scn: &Self::Scenario) -> Vec<(Room, i64)> {
        scn.0.get(&self.name).unwrap().1.iter().map(|r|{(Room{name:r.clone()}, 1)}).collect()
    }

    fn heuristic_remaining_cost(&self, scn: &Self::Scenario) -> i64 {
        scn.0.len() as i64
    }
}

fn time_to_reach_and_open_valve(map: &Map, start: i64, end: i64) -> i64 {
    a_star(&Room{name:start}, &(map.clone(), end)).unwrap().1 + 1
}

fn compute_shortcuts(map: &Map, dests: &Vec<i64>) -> Shortcuts {
    let mut shortcuts: Shortcuts = HashMap::new();
    for start in dests {
        let mut these: HashMap<i64, i64> = HashMap::new();
        for end in dests {
            if let Some(those) = shortcuts.get(start) {
                if let Some(d) = those.get(end) {
                    these.insert(*end, *d);
                    continue;
                }
            }
            these.insert(*end, time_to_reach_and_open_valve(map, *start, *end));
        }
        shortcuts.insert(*start, these);
    }
    return shortcuts;
}

fn shortcut(shortcuts: &Shortcuts, start: i64, end: i64) -> i64 {
    *shortcuts.get(&start).unwrap().get(&end).unwrap()
}

fn benefit_and_remaining_time_after_valve(map: &Map, shortcuts: &Shortcuts, remaining_time: i64, start: i64, end: i64) -> (i64, i64) {
    let t = shortcut(shortcuts, start, end);
    let remaining_time = remaining_time - t;
    (map.get(&end).unwrap().0 * remaining_time, remaining_time)
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct SubStateA {
    location: i64,
    closed_valves: u64,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct SubStateB {
    pressure_out: i64,
    time_left: i64,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct GameState {
    a: SubStateA,
    b: SubStateB,
}

fn get_closed_valves(game_state: &GameState, map: &Map) -> Vec<(i64, u64)> {
    let mut closed_valves = Vec::new();
    for (valve, (_, _, mask)) in map {
        if game_state.a.closed_valves & mask != 0 {
            closed_valves.push((*valve, *mask));
        }
    }
    return closed_valves;
}

fn heuristic(game_state: &GameState, map: &Map, shortcuts: &Shortcuts) -> i64 {
    let mut h = game_state.b.pressure_out;
    for (valve, _) in get_closed_valves(game_state, map) {
        let (b, t) = benefit_and_remaining_time_after_valve(map, shortcuts, game_state.b.time_left, game_state.a.location, valve);
        if t >= 0 {
            h += b;
        }
    }
    return h;
}

fn solve_for_valves_in_time(map: &Map, shortcuts: &Shortcuts,  my_valves: u64, allowed_time: i64, best_proven: i64) -> i64 {
    let init_state = GameState {
            a: SubStateA {
                location: name_to_number("AA"),
                closed_valves: my_valves,
            },
            b: SubStateB {
                pressure_out: 0,
                time_left: allowed_time,
            },
        };

    let mut to_visit: PriorityQueue<GameState, i64> = PriorityQueue::new();
    let mut already_visited: HashMap<SubStateA, SubStateB> = HashMap::new();
    to_visit.push(init_state, heuristic(&init_state, &map, shortcuts));
    let mut best_proven = best_proven;
    let mut iters = 0;
    loop {
        if let Some((game_state, _)) = to_visit.pop() {
            iters += 1;
            if iters % 10000 == 0 {
                dbg!((iters, best_proven, game_state.b.time_left, to_visit.len()));
            }

            if game_state.b.time_left < 0 {
                continue;
            }
            best_proven = max(best_proven, game_state.b.pressure_out);
            already_visited.insert(game_state.a, game_state.b);
            if game_state.b.time_left == 0 {
                continue;
            }
            let closed_valves = get_closed_valves(&game_state, &map);
            for (valve, mask) in closed_valves {
                let (b, t) = benefit_and_remaining_time_after_valve(&map, shortcuts, game_state.b.time_left, game_state.a.location, valve);
                if t < 0 {
                    continue;
                }
                let new_state = GameState {
                        a: SubStateA {
                            location: valve,
                            closed_valves: game_state.a.closed_valves & (!mask),
                        },
                        b: SubStateB {
                            pressure_out: game_state.b.pressure_out + b,
                            time_left: t,
                        },
                    };
                let h = heuristic(&new_state, &map, shortcuts);
                if h > best_proven {
                    if let Some(already_b) = already_visited.get(&new_state.a) {
                        if already_b.pressure_out < new_state.b.pressure_out || already_b.time_left < new_state.b.time_left {
                            to_visit.push(new_state, h);
                        }
                    } else {
                        to_visit.push(new_state, h);
                    }
                }
            }
        } else {
            break;
        }
    }
    return best_proven;
}


fn part1(filename: &str, expected: i64) {
    let map = parse_input(filename);
    let good_valves: Vec<i64> = map.iter().filter(|(_, (rate, _, _))|{*rate > 0}).map(|x|{*x.0}).collect();
    let mut gvps = good_valves.clone();
    gvps.push(name_to_number("AA"));
    let shortcuts = compute_shortcuts(&map, &gvps);

    let mut init_closed_valves = 0u64;
    for v in &good_valves {
        init_closed_valves |= map.get(v).unwrap().2;
    }
    let init_closed_valves = init_closed_valves;

    let answer = solve_for_valves_in_time(&map, &shortcuts, init_closed_valves, 30, 0);

    println!("{}", answer);
    assert_eq!(answer, expected);
}

fn part2(filename: &str, expected: i64) {
    let map = parse_input(filename);
    let good_valves: Vec<i64> = map.iter().filter(|(_, (rate, _, _))|{*rate > 0}).map(|x|{*x.0}).collect();
    let mut gvps = good_valves.clone();
    gvps.push(name_to_number("AA"));
    let shortcuts = compute_shortcuts(&map, &gvps);

    let mut init_closed_valves = 0u64;
    for v in &good_valves {
        init_closed_valves |= map.get(v).unwrap().2;
    }
    let init_closed_valves = init_closed_valves;

    let mut best_proven = 0;
    for compressed_my_valves in 0..(1<<(good_valves.len()-1)) {
        let mut my_valves = 0;
        let mut elephant_valves = 0;
        for bit in 0..good_valves.len() {
            let mine = (compressed_my_valves >> bit) & 1 == 1;
            let mut bits_seen = 0;
            let mut mask = 1;
            loop {
                if init_closed_valves & mask != 0{
                    bits_seen += 1;
                }
                if bits_seen == bit + 1 {
                    break;
                }
                mask <<= 1;
            }
            if mine {
                my_valves |= mask;
            } else {
                elephant_valves |= mask;
            }
        }
        println!("{:X}, {:X}, {}", my_valves, elephant_valves, best_proven);
        let my_score = solve_for_valves_in_time(&map, &shortcuts, my_valves, 26, 0);
        let elephant_score = solve_for_valves_in_time(&map, &shortcuts, elephant_valves, 26, best_proven - my_score);
        best_proven = max(best_proven, my_score + elephant_score);
    }

    println!("{}", best_proven);
    assert_eq!(best_proven, expected);
}

fn main() {
    part1("input/d16.ex", 1651);
    part1("input/d16.txt", 1754);
    part2("input/d16.ex", 1707);
    part2("input/d16.txt", 2474);
}
