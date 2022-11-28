// test!

use rusty_advent::*;

fn main () {
    assert!(vec_char("hello")[4] == 'o');

    assert!(file_lines("src/bin/advent-test.rs")[0] == "// test!");
    assert!(file_vec_char_char("src/bin/advent-test.rs")[0][1] == '/');

    let c = recap(r"^(?P<a>\S+) (?P<b>\S+) (?P<c>\S+)$", "hello 36 false");
    assert!(c.getstr("a") == "hello");
    assert!(c.get::<String>("a") == "hello");
    assert!(c.get::<u64>("b") == 36);
    assert!(c.get::<f64>("b") == 36.0);
    assert!(c.get::<bool>("c") == false);

    println!("Tests passed");
}
