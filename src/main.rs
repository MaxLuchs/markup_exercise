use std::env::args;

use markup::MyMarkup;

fn main() {
    let text = args().nth(1).unwrap();
    let mut my_mark_up = MyMarkup::new();
    let output = my_mark_up.process(text);
    println!("Output: {}", output);
}
