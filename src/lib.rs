#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Upper,   // ^
    Comment, // #
    Normal,
    Lower, // _
}

pub struct MyMarkup {
    state: State,
}

impl MyMarkup {
    pub fn new() -> Self {
        MyMarkup {
            state: State::Normal,
        }
    }

    pub fn process(&mut self, text: String) -> String {
        let letters = text.chars();
        let mut marked_up = vec![];
        for letter in letters {
            let result = markup(self.state, letter);
            self.state = result.0;
            if let Some(mapped_letter) = result.1 {
                marked_up.push(mapped_letter);
            }
        }
        return marked_up.iter().collect();
    }
}

fn markup(state: State, letter: char) -> (State, Option<char>) {
    match letter {
        '^' => (
            if state == State::Upper {
                State::Normal
            } else {
                State::Upper
            },
            None,
        ),
        '#' => (
            if state == State::Comment {
                State::Normal
            } else {
                State::Comment
            },
            None,
        ),
        '_' => (
            if state == State::Lower {
                State::Normal
            } else {
                State::Lower
            },
            None,
        ),
        _ if state == State::Upper => (state, letter.to_uppercase().nth(0)),
        _ if state == State::Lower => (state, letter.to_lowercase().nth(0)),
        _ if state == State::Comment => (state, None),
        _ => (state, Some(letter)),
    }
}

pub fn process(text: String) -> String {
    let letters = text.chars();
    let mut state = State::Normal;
    let mut marked_up = vec![];
    for letter in letters {
        let result = markup(state, letter);
        state = result.0;
        if let Some(mapped_letter) = result.1 {
            marked_up.push(mapped_letter);
        }
    }
    return marked_up.iter().collect();
}

#[cfg(test)]
mod tests {
    use crate::MyMarkup;

    #[test]
    fn test_process() {
        let mut my_mark_up = MyMarkup::new();
        let actual = my_mark_up.process(String::from("Hello _WHATS_ ^Your Name^#Wusel#?"));
        let expected = "Hello whats YOUR NAME?";
        assert_eq!(expected, actual);
    }
}
