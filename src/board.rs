use crate::{BoardPiece, CastlingRights, Color, File, Rank, Square};

#[derive(Clone)]
pub struct Board {
    array: [BoardPiece; 64],
    pub turn: Color,
    pub ep_file: Option<File>,
    pub castle_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_count: u16,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            array: [BoardPiece::Empty; 64],
            turn: Color::White,
            ep_file: None,
            castle_rights: CastlingRights::none(),
            halfmove_clock: 0,
            fullmove_count: 1,
        }
    }

    pub fn ep_square(&self) -> Option<Square> {
        self.ep_file.map(|file| {
            let rank = match self.turn {
                Color::White => Rank::R6,
                Color::Black => Rank::R3,
            };
            Square::from((rank, file))
        })
    }

    pub fn piece_at(&self, sq: Square) -> BoardPiece {
        self.array[sq as usize]
    }

    pub fn set_piece_at(&mut self, sq: Square, piece: BoardPiece) {
        self.array[sq as usize] = piece;
    }
}
