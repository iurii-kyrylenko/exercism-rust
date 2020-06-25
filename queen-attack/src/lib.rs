#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(Self { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = (self.rank() - other.rank()).abs();
        let dy = (self.file() - other.file()).abs();

        dx == 0 || dy == 0 || dx == dy
    }

    fn rank(&self) -> i32 {
        self.0.rank
    }

    fn file(&self) -> i32 {
        self.0.file
    }
}
