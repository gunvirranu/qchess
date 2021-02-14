use std::fmt;

use crate::{BoardPiece, File, PieceType, Square};

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

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CastlingRights(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateChange {
    pub last_move: Move,
    pub captured: BoardPiece,
    pub last_ep_file: Option<File>,
    pub last_castle_rights: CastlingRights,
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

impl CastlingRights {
    pub fn new(wk: bool, wq: bool, bk: bool, bq: bool) -> Self {
        Self((wk as u8) << 3 | (wq as u8) << 2 | (bk as u8) << 1 | (bq as u8))
    }

    pub fn none() -> Self {
        Self::new(false, false, false, false)
    }

    pub fn all() -> Self {
        Self::new(true, true, true, true)
    }

    pub fn white_king(self) -> bool {
        self.0 & 0b1000 != 0
    }

    pub fn white_queen(self) -> bool {
        self.0 & 0b0100 != 0
    }

    pub fn black_king(self) -> bool {
        self.0 & 0b0010 != 0
    }

    pub fn black_queen(self) -> bool {
        self.0 & 0b0001 != 0
    }
}

impl fmt::Debug for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.white_king() { 'K' } else { '-' })?;
        write!(f, "{}", if self.white_queen() { 'Q' } else { '-' })?;
        write!(f, "{}", if self.black_king() { 'k' } else { '-' })?;
        write!(f, "{}", if self.black_queen() { 'q' } else { '-' })
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 0 {
            return write!(f, "-");
        }
        if self.white_king() {
            write!(f, "K")?;
        }
        if self.white_queen() {
            write!(f, "Q")?;
        }
        if self.black_king() {
            write!(f, "k")?;
        }
        if self.black_queen() {
            write!(f, "q")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_castling_rights() {
        for &(a, b, c, d) in [
            (false, false, false, false),
            (false, false, false, true),
            (false, true, false, true),
            (true, false, false, false),
            (true, false, true, false),
            (true, true, true, true),
        ]
        .iter()
        {
            let rights = CastlingRights::new(a, b, c, d);
            assert_eq!(rights.white_king(), a);
            assert_eq!(rights.white_queen(), b);
            assert_eq!(rights.black_king(), c);
            assert_eq!(rights.black_queen(), d);
        }
    }
}
