enum State {
    Space,
    WordL,
    WordU,
    WordA,
}

struct Abbr<'a> {
    current: std::str::Chars<'a>,
    state: State,
}

impl<'a> Abbr<'a> {
    fn new(phrase: &'a str) -> Self {
        Abbr {
            current: phrase.chars(),
            state: State::Space,
        }
    }

    fn handle_space(&mut self, c: char) -> bool {
        if c.is_alphabetic() {
            self.state = if c.is_uppercase() {
                State::WordU
            } else {
                State::WordL
            };
            true
        } else {
            false
        }
    }

    fn handle_word_l(&mut self, c: char) -> bool {
        if c == '\'' {
            self.state = State::WordA;
            false
        } else if !c.is_alphabetic() {
            self.state = State::Space;
            false
        } else if c.is_uppercase() {
            self.state = State::WordU;
            true
        } else {
            false
        }
    }

    fn handle_word_u(&mut self, c: char) -> bool {
        if c == '\'' {
            self.state = State::WordA;
        } else if !c.is_alphabetic() {
            self.state = State::Space;
        } else if c.is_lowercase() {
            self.state = State::WordL;
        }
        false
    }

    fn handle_word_a(&mut self, c: char) -> bool {
        if c.is_alphabetic() {
            self.state = if c.is_uppercase() {
                State::WordU
            } else {
                State::WordL
            };
        } else {
            self.state = State::Space;
        }
        false
    }

    fn state_transition(&mut self, c: char) -> bool {
        match self.state {
            State::Space => self.handle_space(c),
            State::WordL => self.handle_word_l(c),
            State::WordU => self.handle_word_u(c),
            State::WordA => self.handle_word_a(c),
        }
    }
}

impl Iterator for Abbr<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.current.next()?;

            if self.state_transition(c) {
                return Some(c.to_uppercase().to_string());
            }
        }
    }
}

pub fn abbreviate(phrase: &str) -> String {
    Abbr::new(phrase).collect()
}
