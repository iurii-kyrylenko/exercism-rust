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
}

impl Iterator for Abbr<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.current.next()?;

            match self.state {
                State::Space => {
                    if c.is_alphabetic() {
                        self.state = if c.is_uppercase() {
                            State::WordU
                        } else {
                            State::WordL
                        };
                        return Some(c.to_uppercase().to_string());
                    }
                }
                State::WordL => {
                    if c == '\'' {
                        self.state = State::WordA;
                        continue;
                    }
                    if !c.is_alphabetic() {
                        self.state = State::Space;
                        continue;
                    }
                    if c.is_uppercase() {
                        self.state = State::WordU;
                        return Some(c.to_uppercase().to_string());
                    }
                }
                State::WordU => {
                    if c == '\'' {
                        self.state = State::WordA;
                        continue;
                    }
                    if !c.is_alphabetic() {
                        self.state = State::Space;
                        continue;
                    }
                    if c.is_lowercase() {
                        self.state = State::WordL;
                        continue;
                    }
                }
                State::WordA => {
                    if !c.is_alphabetic() {
                        self.state = State::Space;
                        continue;
                    }
                    if c.is_alphabetic() {
                        self.state = if c.is_uppercase() {
                            State::WordU
                        } else {
                            State::WordL
                        };
                        continue;
                    }
                }
            }
        }
    }
}

pub fn abbreviate(phrase: &str) -> String {
    Abbr::new(phrase).collect()
}
