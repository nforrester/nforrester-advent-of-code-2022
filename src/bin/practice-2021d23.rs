use std::cmp;
use std::collections::BinaryHeap;
use std::iter;

type Cost = i64;

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
enum Amph { A, B, C, D }

impl Amph {
    fn cost_multiplier(&self) -> Cost {
        match self {
            Amph::A => 1,
            Amph::B => 10,
            Amph::C => 100,
            Amph::D => 1000,
        }
    }

    fn is_happy_at(&self, pos: &Pos) -> bool {
        match pos {
            Pos::Hall(_)  => false,
            Pos::Room2(_) => matches!(self, Amph::A),
            Pos::Room4(_) => matches!(self, Amph::B),
            Pos::Room6(_) => matches!(self, Amph::C),
            Pos::Room8(_) => matches!(self, Amph::D),
        }
    }

    fn happy_places(&self) -> &[Pos; 4] {
        match self {
            Amph::A => &[Pos::Room2(3), Pos::Room2(2), Pos::Room2(1), Pos::Room2(0)],
            Amph::B => &[Pos::Room4(3), Pos::Room4(2), Pos::Room4(1), Pos::Room4(0)],
            Amph::C => &[Pos::Room6(3), Pos::Room6(2), Pos::Room6(1), Pos::Room6(0)],
            Amph::D => &[Pos::Room8(3), Pos::Room8(2), Pos::Room8(1), Pos::Room8(0)],
        }
    }

    fn depict(&self) -> &str {
        match self {
            Amph::A => "A",
            Amph::B => "B",
            Amph::C => "C",
            Amph::D => "D",
        }
    }
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Clone)]
#[derive(Copy)]
struct State {
    energy_used: Cost,
    hall:  [Option<Amph>; 11], // index left to right
    room2: [Option<Amph>; 4],  // index top to bottom
    room4: [Option<Amph>; 4],
    room6: [Option<Amph>; 4],
    room8: [Option<Amph>; 4],
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Pos {
    Hall(isize),  // index left to right 
    Room2(isize), // index top to bottom 
    Room4(isize),
    Room6(isize),
    Room8(isize),
}

impl Pos {
    fn x(&self) -> isize {
        match self {
            Pos::Hall(x)  => *x,
            Pos::Room2(_) => 2,
            Pos::Room4(_) => 4,
            Pos::Room6(_) => 6,
            Pos::Room8(_) => 8,
        }
    }

    fn y(&self) -> isize {
        match self {
            Pos::Hall(_)  => -1,
            Pos::Room2(y) => *y,
            Pos::Room4(y) => *y,
            Pos::Room6(y) => *y,
            Pos::Room8(y) => *y,
        }
    }

    fn dist(&self, other: &Self) -> isize {
        let xcost = (self.x() - other.x()).abs();
        let ycost = if xcost == 0 { (self.y() - other.y()).abs() } else { self.y() + other.y() + 2 };
        xcost + ycost
    }

    fn but_with_y(&self, new_y: isize) -> Pos {
        match self {
            Pos::Hall(_)  => panic!("AAAAAAAAAAAAAA!"),
            Pos::Room2(_) => Pos::Room2(new_y),
            Pos::Room4(_) => Pos::Room4(new_y),
            Pos::Room6(_) => Pos::Room6(new_y),
            Pos::Room8(_) => Pos::Room8(new_y),
        }
    }

    fn is_blocking(&self, state: &State) -> bool {
        if let Pos::Hall(_) = self {
            return false;
        }
        for blocked_y in (self.y()+1)..=3 {
            let blocked_pos = self.but_with_y(blocked_y);
            if let Some(amph) = state.at(&blocked_pos) {
                if !amph.is_happy_at(&blocked_pos) {
                    return true;
                }
            }
        }
        return false;
    }
}

const POSITIONS: [Pos; 27] = [
    Pos::Hall(0),
    Pos::Hall(1),
    Pos::Hall(2),
    Pos::Hall(3),
    Pos::Hall(4),
    Pos::Hall(5),
    Pos::Hall(6),
    Pos::Hall(7),
    Pos::Hall(8),
    Pos::Hall(9),
    Pos::Hall(10),
    Pos::Room2(0),
    Pos::Room2(1),
    Pos::Room2(2),
    Pos::Room2(3),
    Pos::Room4(0),
    Pos::Room4(1),
    Pos::Room4(2),
    Pos::Room4(3),
    Pos::Room6(0),
    Pos::Room6(1),
    Pos::Room6(2),
    Pos::Room6(3),
    Pos::Room8(0),
    Pos::Room8(1),
    Pos::Room8(2),
    Pos::Room8(3),
];

const ROOM_POSITIONS: [Pos; 16] = [
    Pos::Room2(0),
    Pos::Room2(1),
    Pos::Room2(2),
    Pos::Room2(3),
    Pos::Room4(0),
    Pos::Room4(1),
    Pos::Room4(2),
    Pos::Room4(3),
    Pos::Room6(0),
    Pos::Room6(1),
    Pos::Room6(2),
    Pos::Room6(3),
    Pos::Room8(0),
    Pos::Room8(1),
    Pos::Room8(2),
    Pos::Room8(3),
];

#[derive(Debug)]
struct Move {
    start: Pos,
    end: Pos,
}

impl Move {
    fn move_cost(&self, state: &State) -> Cost {
        state.at(&self.start).as_ref().unwrap().cost_multiplier() * (self.start.dist(&self.end) as Cost)
    }

    fn unblocked(&self, state: &State) -> bool {
        let xmin = cmp::min(self.start.x(), self.end.x());
        let xmax = cmp::max(self.start.x(), self.end.x());
        for x in xmin..=xmax {
            if x == self.start.x() {
                continue;
            }
            if state.at(&Pos::Hall(x)).is_some() {
                return false;
            }
        }
        if self.start.y() >= 0 {
            for transit_y in 0..self.start.y() {
                if state.at(&self.start.but_with_y(transit_y)).is_some() {
                    return false;
                }
            }
        }
        if self.end.y() >= 0 {
            for transit_y in 0..=self.end.y() {
                if state.at(&self.end.but_with_y(transit_y)).is_some() {
                    return false;
                }
            }
        }
        if state.at(&self.end).is_some() {
            return false;
        }
        return true;
    }
}

impl State {
    fn at(&self, pos: &Pos) -> &Option<Amph> {
        match pos {
            Pos::Hall(x)  => &self.hall[*x as usize],
            Pos::Room2(y) => &self.room2[*y as usize],
            Pos::Room4(y) => &self.room4[*y as usize],
            Pos::Room6(y) => &self.room6[*y as usize],
            Pos::Room8(y) => &self.room8[*y as usize],
        }
    }

    fn list_valid_moves(&self) -> Vec<Move> {
        let mut valid_moves = Vec::new();
        for start in POSITIONS {
            if let Some(amph) = self.at(&start) {
                if amph.is_happy_at(&start) {
                    if !start.is_blocking(&self) {
                        continue;
                    }
                } else {
                    for end in amph.happy_places() {
                        if end.is_blocking(&self) {
                            continue;
                        }
                        let mv = Move{start, end: *end};
                        if mv.unblocked(&self) {
                            valid_moves.push(mv);
                            break;
                        }
                    }
                }
                if let Pos::Hall(_) = start {
                    continue;
                }
                for x in 0..11 {
                    if x == 2 || x == 4 || x == 6 || x == 8 {
                        continue;
                    }
                    let mv = Move{start, end: Pos::Hall(x)};
                    if mv.unblocked(&self) {
                        valid_moves.push(mv);
                    }
                }
            }
        }
        valid_moves
    }

    fn make_move(&self, mv: &Move) -> State {
        let mut hall = self.hall;
        let mut room2 = self.room2;
        let mut room4 = self.room4;
        let mut room6 = self.room6;
        let mut room8 = self.room8;
        match mv.end {
            Pos::Hall(x)  => {hall[x as usize]  = *self.at(&mv.start)},
            Pos::Room2(y) => {room2[y as usize] = *self.at(&mv.start)},
            Pos::Room4(y) => {room4[y as usize] = *self.at(&mv.start)},
            Pos::Room6(y) => {room6[y as usize] = *self.at(&mv.start)},
            Pos::Room8(y) => {room8[y as usize] = *self.at(&mv.start)},
        }
        match mv.start {
            Pos::Hall(x)  => {hall[x as usize]  = None},
            Pos::Room2(y) => {room2[y as usize] = None},
            Pos::Room4(y) => {room4[y as usize] = None},
            Pos::Room6(y) => {room6[y as usize] = None},
            Pos::Room8(y) => {room8[y as usize] = None},
        }
        State {
            energy_used: self.energy_used + mv.move_cost(&self),
            hall,
            room2,
            room4,
            room6,
            room8,
        }
    }

    fn optimistic_heuristic(&self) -> Cost {
        let mut lower_bound = self.energy_used;
        for pos in POSITIONS {
            if let Some(amph) = self.at(&pos) {
                if amph.is_happy_at(&pos) {
                    if pos.is_blocking(&self) {
                        lower_bound += 10 * amph.cost_multiplier();
                    }
                } else {
                    let happy_place = amph.happy_places()[1];
                    let mv = Move{start: pos, end: happy_place};
                    lower_bound += mv.move_cost(&self);
                }
            }
        }
        return lower_bound;
    }

    fn all_happy(&self) -> bool {
        for pos in ROOM_POSITIONS {
            if let Some(amph) = self.at(&pos) {
                if !amph.is_happy_at(&pos) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    fn depict(&self) -> [String; 7] {
        let mut depiction: [String; 7] = [
            String::from("#############"),
            String::from("#...........#"),
            String::from("###.#.#.#.###"),
            String::from("  #.#.#.#.#  "),
            String::from("  #.#.#.#.#  "),
            String::from("  #.#.#.#.#  "),
            String::from("  #########  "),
        ];
        for x in 0..11 {
            if let Some(amph) = self.at(&Pos::Hall(x)) {
                let ux = x as usize;
                depiction[1].replace_range((ux+1)..(ux+2), amph.depict());
            }
        }
        for y in 0..4 {
            let uy = y as usize;
            if let Some(amph) = self.at(&Pos::Room2(y)) {
                depiction[uy+2].replace_range(3..4, amph.depict());
            }
            if let Some(amph) = self.at(&Pos::Room4(y)) {
                depiction[uy+2].replace_range(5..6, amph.depict());
            }
            if let Some(amph) = self.at(&Pos::Room6(y)) {
                depiction[uy+2].replace_range(7..8, amph.depict());
            }
            if let Some(amph) = self.at(&Pos::Room8(y)) {
                depiction[uy+2].replace_range(9..10, amph.depict());
            }
        }
        return depiction;
    }
}

fn main() {
    let init = State {
        energy_used: 0,
        hall: [None; 11],
        room2: [Some(Amph::A), Some(Amph::D), Some(Amph::D), Some(Amph::C)],
        room4: [Some(Amph::D), Some(Amph::C), Some(Amph::B), Some(Amph::D)],
        room6: [Some(Amph::C), Some(Amph::B), Some(Amph::A), Some(Amph::B)],
        room8: [Some(Amph::A), Some(Amph::A), Some(Amph::C), Some(Amph::B)],
    };

    for line in init.depict() {
        println!("{}", line);
    }

    let mut todo_list = BinaryHeap::new();
    todo_list.push((-init.optimistic_heuristic(), init/*, Vec::<State>::new()*/));
    let mut iters = 0u64;
    loop {
        iters += 1;
        let (_, state/*, history*/) = todo_list.pop().unwrap();

        if iters % 100000 == 0 {
            for line in state.depict() {
                println!("{}", line);
            }
        }

        if state.all_happy() {
            /*
            println!("HISTORY:");
            for old_state in history {
                for line in old_state.depict() {
                    println!("{}", line);
                }
            }
            for line in state.depict() {
                println!("{}", line);
            }
            */
            // Retrofitting this to produce the answer to part 1 as well is not practical.
            println!("part 2: {}", state.energy_used);
            assert_eq!(state.energy_used, 52055);
            break;
        }
        for mv in state.list_valid_moves() {
            let new_state = state.make_move(&mv);
            /*
            let mut new_history = history.clone();
            new_history.push(state);
            */

            if iters % 10000000 == 0 {
                for (line_before, line_after) in iter::zip(state.depict(), new_state.depict()) {
                    println!("{} {}", line_before, line_after);
                }
            }

            todo_list.push((-new_state.optimistic_heuristic(), new_state/*, new_history*/));
        }
    }
}
