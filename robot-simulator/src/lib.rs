#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::{East, North, South, West};

impl Direction {
    fn right(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

pub struct Robot {
    direction: Direction,
    position: (i32, i32),
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            direction: d,
            position: (x, y),
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            direction: self.direction.right(),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            direction: self.direction.left(),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.position;

        let position = match self.direction {
            North => (x, y + 1),
            East => (x + 1, y),
            South => (x, y - 1),
            West => (x - 1, y),
        };

        Self { position, ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => panic!("Invalid instruction!"),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
