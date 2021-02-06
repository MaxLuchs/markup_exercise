use std::{env::args};

#[derive(PartialEq, Eq)]
enum State {
    Upper, // ^
    Comment, // #
    Normal,
    Lower // _
}

fn map_char_to_state(letter: char) -> Option<State> {}


fn markup(state: State, letter: char) -> (State, char) {
    (state, letter)
}

fn process(text: String) ->String {
    let letters = text.chars();
    let mut state = State::Normal;
    let mut marked_up = vec![];
    for letter in letters {
        let result = markup(state, letter);
        state = result.0;
        marked_up.push(result.1);    
    }
    return marked_up.iter().collect();
}

fn main() {
    let text = args().nth(1).unwrap();
    let output = process(text);
    println!("Output: {}", output);
}
