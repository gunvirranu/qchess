use std::fmt;

use crate::{PieceType, Square};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveType {
    Normal,
    EnPassant,
    DoublePush,
    Castle,
    Promotion(PieceType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Move {
    from: Square,
    to: Square,
    move_type: MoveType,
}

impl Move {
    pub fn new(from: Square, to: Square, move_type: MoveType) -> Self {
        Self {
            from,
            to,
            move_type,
        }
    }

    // Construct a normal move
    pub fn normal(from: Square, to: Square) -> Self {
        Self::new(from, to, MoveType::Normal)
    }

    pub fn from(self) -> Square {
        self.from
    }

    pub fn to(self) -> Square {
        self.to
    }

    pub fn move_type(self) -> MoveType {
        self.move_type
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Chess algebraic notation for moves is annoying.
        // Also, it requires info not contained in `Move`.
        write!(f, "{}->{}", self.from(), self.to())
    }
}
