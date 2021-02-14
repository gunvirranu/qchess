use std::convert::TryFrom;
use std::fmt;

use crate::{BoardPiece, CastlingRights, Color, File, Rank, Square};

const INIT_FEN_LEN: usize = 8 * 8 + 7 + 1 + 4 + 2 + 2 + 3 + 5;

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

    // Convert board to FEN
    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(INIT_FEN_LEN);
        // 1. Piece placement
        let mut empty = 0;
        for r in (0u8..8).rev() {
            for f in 0..8 {
                let sq = Square::try_from((r, f)).unwrap();
                match self.piece_at(sq) {
                    BoardPiece::Empty => empty += 1,
                    BoardPiece::Piece(piece) => {
                        if empty != 0 {
                            fen.push_str(&empty.to_string());
                        }
                        fen.push_str(&format!("{:?}", piece));
                        empty = 0;
                    }
                }
            }
            if empty != 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            fen.push('/');
        }
        fen.pop();
        // 2. Side to move
        fen.push(' ');
        fen.push(match self.turn {
            Color::White => 'w',
            Color::Black => 'b',
        });
        // 3. Castling rights
        fen.push_str(&format!(" {}", self.castle_rights));
        // 4. En passant
        match self.ep_square() {
            None => fen.push_str(" -"),
            Some(sq) => fen.push_str(&format!(" {}", sq)),
        }
        // 5. Halfmove clock
        fen.push_str(&format!(" {}", self.halfmove_clock));
        // 6. Fullmove counter
        fen.push_str(&format!(" {}", self.fullmove_count));
        fen
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Side to move     {:?}", self.turn)?;
        writeln!(f, "Castling rights  {:?}", self.castle_rights)?;
        writeln!(f, "En passant       {:?}", self.ep_square())?;
        writeln!(f, "Halfmove clock   {:?}", self.halfmove_clock)?;
        writeln!(f, "Fullmove count   {:?}", self.fullmove_count)?;
        writeln!(f, "  +-----------------+")?;
        for i in (0u8..8).rev() {
            write!(f, "{} |", i + 1)?;
            for j in 0..8 {
                let sq = Square::try_from((i, j)).unwrap();
                match self.piece_at(sq) {
                    BoardPiece::Empty => {
                        write!(f, " -")?;
                    }
                    BoardPiece::Piece(piece) => {
                        write!(f, " {:?}", piece)?;
                    }
                }
            }
            writeln!(f, " |")?;
        }
        writeln!(f, "  +-----------------+")?;
        writeln!(f, "    a b c d e f g h")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   ╔═════════════════╗")?;
        for i in (0u8..8).rev() {
            write!(f, " {} ║", i + 1)?;
            for j in 0..8 {
                let sq = Square::try_from((i, j)).unwrap();
                match self.piece_at(sq) {
                    BoardPiece::Empty => {
                        if i % 2 == j % 2 {
                            write!(f, " ·")?;
                        } else {
                            write!(f, "  ")?;
                        }
                    }
                    BoardPiece::Piece(piece) => {
                        write!(f, " {}", piece)?;
                    }
                }
            }
            writeln!(f, " ║")?;
        }
        writeln!(f, "   ╚═════════════════╝")?;
        writeln!(f, "     a b c d e f g h")
    }
}
