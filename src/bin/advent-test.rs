// test!

use rusty_advent::*;

fn main () {
    assert!(vec_char("hello")[4] == 'o');
    assert!(vec_word("hello world")[1] == "world");
    assert!(vec_by_sep("hello,world", ",")[1] == "world");

    assert!(file_lines("src/bin/advent-test.rs")[0] == "// test!");
    assert!(file_vec_vec_char("src/bin/advent-test.rs")[0][1] == '/');
    assert!(file_vec_vec_word("src/bin/advent-test.rs")[0][1] == "test!");
    assert!(file_vec_vec_by_sep("src/bin/advent-test.rs", "t")[0][1] == "es");

    let c = recap(r"^(?P<a>\S+) (?P<b>\S+) (?P<c>\S+)$", "hello 36 false");
    assert!(c.getstr("a") == "hello");
    assert!(c.get::<String>("a") == "hello");
    assert!(c.get::<u64>("b") == 36);
    assert!(c.get::<f64>("b") == 36.0);
    assert!(c.get::<bool>("c") == false);

    println!("Tests passed");
}
