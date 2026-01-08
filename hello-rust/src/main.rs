use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let message = String::from("Kitaaaao du ceu");
    let width = message.chars().count();

    let mut writter = BufWriter::new(stdout.lock());
    say(&message, width, &mut writter).unwrap();
}
