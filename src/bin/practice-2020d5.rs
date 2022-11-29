use rusty_advent::*;

fn main () {
    let mut max_id = 0;
    for line in file_vec_vec_char("input/practice-2020d5.txt") {
		let row_spec = &line[..7];
		let col_spec = &line[7..];
        let mut row = 0;
        let mut col = 0;
        for bit in row_spec {
            row *= 2;
            if *bit == 'B' {
                row += 1;
            } else {
                assert!(*bit == 'F', "{}", bit);
            }
        }
        for bit in col_spec {
            col *= 2;
            if *bit == 'R' {
                col += 1;
            } else {
                assert!(*bit == 'L', "{}", bit);
            }
        }
        let id = row * 8 + col;
        println!("{}", id);
        max_id = std::cmp::max(max_id, id);
    }
    println!("max = {}", max_id);
}
