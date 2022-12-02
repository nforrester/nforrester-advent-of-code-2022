use rusty_advent::*;
use std::vec::Vec;

fn main() {
	let mut a = 0;
	let mut b = 0;
    for line in file_vec_vec_word("input/d2.txt") {
        let player1 = &line[0];
        let player2 = &line[1];
        if player2 == "X" {
            a += 1;
        } else if player2 == "Y" {
            a += 2;
        } else if player2 == "Z" {
            a += 3;
        } else {
            panic!("");
        }
        if player1 == "A" {
        } else if player1 == "B" {
        } else if player1 == "C" {
        } else {
            panic!("");
        }
        if player2 == "X" {
            if player1 == "A" {
                a += 3;
            } else if player1 == "B" {
                a += 0;
            } else if player1 == "C" {
                a += 6;
            }
        } else if player2 == "Y" {
            if player1 == "A" {
                a += 6;
            } else if player1 == "B" {
                a += 3;
            } else if player1 == "C" {
                a += 0;
            }
        } else if player2 == "Z" {
            if player1 == "A" {
                a += 0;
            } else if player1 == "B" {
                a += 6;
            } else if player1 == "C" {
                a += 3;
            }
        }





        let pstrat = player2;
        let player2;
        if pstrat == "X" {
            if player1 == "A" {
                player2 = "Z";
            } else if player1 == "B" {
                player2 = "X";
            } else if player1 == "C" {
                player2 = "Y";
            } else {panic!("");}
        } else if pstrat == "Y" {
            if player1 == "A" {
                player2 = "X";
            } else if player1 == "B" {
                player2 = "Y";
            } else if player1 == "C" {
                player2 = "Z";
            } else {panic!("");}
        } else if pstrat == "Z" {
            if player1 == "A" {
                player2 = "Y";
            } else if player1 == "B" {
                player2 = "Z";
            } else if player1 == "C" {
                player2 = "X";
            } else {panic!("");}
        } else {panic!("");}




        if player2 == "X" {
            b += 1;
        } else if player2 == "Y" {
            b += 2;
        } else if player2 == "Z" {
            b += 3;
        } else {
            panic!("");
        }
        if player1 == "A" {
        } else if player1 == "B" {
        } else if player1 == "C" {
        } else {
            panic!("");
        }
        if player2 == "X" {
            if player1 == "A" {
                b += 3;
            } else if player1 == "B" {
                b += 0;
            } else if player1 == "C" {
                b += 6;
            }
        } else if player2 == "Y" {
            if player1 == "A" {
                b += 6;
            } else if player1 == "B" {
                b += 3;
            } else if player1 == "C" {
                b += 0;
            }
        } else if player2 == "Z" {
            if player1 == "A" {
                b += 0;
            } else if player1 == "B" {
                b += 6;
            } else if player1 == "C" {
                b += 3;
            }
        }
    }
    println!("part 1: {}", a);
    println!("part 2: {}", b);
    assert_eq!(a, 13682);
    assert_eq!(b, 12881);
}
