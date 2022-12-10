use rusty_advent::*;

struct State {
    x: i64,
    cycle: i64,
    signal_strength_total: i64,
    display: String,
}

impl State {
    fn next_cycle(&mut self) {
        if self.cycle % 40 == 0 {
            self.display.push('\n');
        }

        if (self.x - self.cycle % 40).abs() <= 1 {
            self.display.push('#');
        } else {
            self.display.push('.');
        }

        self.cycle += 1;

        if (self.cycle - 20) % 40 == 0 {
            self.signal_strength_total += self.cycle * self.x;
        }
    }
}

fn main() {
    let mut state = State {
            x: 1,
            cycle: 0,
            signal_strength_total: 0,
            display: String::new(),
        };

    for words in file_vec_vec_word("input/d10.txt") {
        if words[0] == "noop"
        {
            state.next_cycle();
        }
        else
        {
            state.next_cycle();
            state.next_cycle();
            state.x += words[1].parse::<i64>().unwrap();
        }
    }

    println!("{}", state.signal_strength_total);
    assert_eq!(state.signal_strength_total, 15260);

    println!("{}", state.display);
    assert_eq!(state.display, "
###...##..#..#.####..##..#....#..#..##..
#..#.#..#.#..#.#....#..#.#....#..#.#..#.
#..#.#....####.###..#....#....#..#.#....
###..#.##.#..#.#....#.##.#....#..#.#.##.
#....#..#.#..#.#....#..#.#....#..#.#..#.
#.....###.#..#.#.....###.####..##...###.");
}
