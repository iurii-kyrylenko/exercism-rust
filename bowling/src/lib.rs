#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum State {
    R1,
    R2,
    R10_1,
    R10_2_1,
    R10_2_2,
    R10_3_1,
    R10_3_2,
    End,
}

pub struct BowlingGame {
    frame: u16,
    state: State,
    bonus: u16,
    prev: u16,
    score: u16,
    successive_strikes: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: 1,
            state: State::R1,
            bonus: 0,
            prev: 0,
            score: 0,
            successive_strikes: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.process(pins)
    }

    pub fn score(&self) -> Option<u16> {
        match self.state {
            State::End => Some(self.score),
            _ => None,
        }
    }

    fn process(&mut self, pins: u16) -> Result<(), Error> {
        match self.state {
            State::R1 => self.process_r1(pins),
            State::R2 => self.process_r2(pins),
            State::R10_1 => self.process_r10_1(pins),
            State::R10_2_1 => self.process_r10_2_1(pins),
            State::R10_2_2 => self.process_r10_2_2(pins),
            State::R10_3_1 => self.process_r10_3_1(pins),
            State::R10_3_2 => self.process_r10_3_2(pins),
            _ => Err(Error::GameComplete),
        }
    }

    fn update_score(&mut self, pins: u16) {
        self.score += pins;
        if self.bonus > 0 {
            self.score += pins;
            self.bonus -= 1;
        }
    }

    fn process_r1(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.successive_strikes > 1 {
            self.score += pins;
        }

        self.update_score(pins);

        match pins {
            0..=9 => {
                self.prev = pins;
                self.successive_strikes = 0;
                self.state = State::R2;
            }
            _ => {
                self.successive_strikes += 1;
                self.bonus = 2;
                self.frame += 1;
                self.state = if self.frame < 10 {
                    State::R1
                } else {
                    State::R10_1
                };
            }
        };

        Ok(())
    }

    fn process_r2(&mut self, pins: u16) -> Result<(), Error> {
        if self.prev + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.update_score(pins);

        if self.prev + pins == 10 {
            self.bonus = 1;
        }

        self.frame += 1;
        self.state = if self.frame < 10 {
            State::R1
        } else {
            State::R10_1
        };

        Ok(())
    }

    fn process_r10_1(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.successive_strikes > 1 {
            self.score += pins;
        }

        self.update_score(pins);

        match pins {
            0..=9 => {
                self.prev = pins;
                self.state = State::R10_2_1;
            }
            _ => self.state = State::R10_2_2,
        };

        Ok(())
    }

    fn process_r10_2_1(&mut self, pins: u16) -> Result<(), Error> {
        if self.prev + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.update_score(pins);

        self.state = if self.prev + pins == 10 {
            State::R10_3_2
        } else {
            State::End
        };

        Ok(())
    }

    fn process_r10_2_2(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.update_score(pins);

        match pins {
            0..=9 => {
                self.prev = pins;
                self.state = State::R10_3_1;
            }
            _ => {
                self.state = State::R10_3_2;
            }
        };

        Ok(())
    }

    fn process_r10_3_1(&mut self, pins: u16) -> Result<(), Error> {
        if self.prev + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.update_score(pins);

        self.state = State::End;

        Ok(())
    }

    fn process_r10_3_2(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.update_score(pins);

        self.state = State::End;

        Ok(())
    }
}
