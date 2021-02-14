use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::moves::StateChange;
use crate::{BoardPiece, CastlingRights, Color, File, Move, MoveType, Rank, SidePiece, Square};

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const INIT_FEN_LEN: usize = 8 * 8 + 7 + 1 + 4 + 2 + 2 + 3 + 5;
const INIT_MOVE_HIST_LEN: usize = 32;

#[derive(Clone)]
pub struct Board {
    array: [BoardPiece; 64],
    pub turn: Color,
    pub ep_file: Option<File>,
    pub castle_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_count: u16,
    history: Vec<StateChange>,
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
            history: Vec::with_capacity(INIT_MOVE_HIST_LEN),
        }
    }

    pub fn default() -> Self {
        Self::from_fen(DEFAULT_FEN).unwrap()
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

    // Doesn't completely validate on purpose, just some checks here and there.
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let fen_vec: Vec<&str> = fen.split_ascii_whitespace().collect();
        if fen_vec.len() != 6 {
            return Err(FenError);
        }
        let mut board = Self::empty();
        // 1. Piece placement
        let ranks: Vec<&str> = fen_vec[0].rsplit('/').collect();
        if ranks.len() != 8 {
            return Err(FenError);
        }
        // Construct board
        let mut sq = 0u8;
        for rank in ranks.iter() {
            let prev_sq = sq;
            for letter in rank.chars() {
                let piece = {
                    if let Ok(spiece) = SidePiece::try_from(letter) {
                        BoardPiece::Piece(spiece)
                    } else {
                        let n = letter.to_digit(9).ok_or(())?;
                        sq += n as u8 - 1;
                        BoardPiece::Empty
                    }
                };
                let square = Square::try_from(sq).unwrap();
                board.set_piece_at(square, piece);
                sq += 1;
            }
            if prev_sq + 8 != sq {
                return Err(FenError);
            }
        }
        // 2. Side to move
        board.turn = match fen_vec[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError),
        };
        // 3. Castling rights
        board.castle_rights = fen_vec[2].parse()?;
        // 4. En passant
        board.ep_file = {
            if fen_vec[3] == "-" {
                None
            } else {
                let sq = Square::from_str(fen_vec[3])?;
                if (board.turn == Color::White && sq.rank() != Rank::R6)
                    || (board.turn == Color::Black && sq.rank() != Rank::R3)
                {
                    return Err(FenError);
                }
                Some(sq.file())
            }
        };
        // 5. Halfmove clock
        board.halfmove_clock = fen_vec[4].parse().map_err(|_| FenError)?;
        // 6. Fullmove counter
        board.fullmove_count = fen_vec[5].parse().map_err(|_| FenError)?;
        Ok(board)
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

    pub fn make_move(&mut self, mv: Move) {
        // TODO: Change assertions to debug
        let from_bpiece = self.piece_at(mv.from());
        let to_bpiece = self.piece_at(mv.to());
        if let BoardPiece::Piece(piece) = from_bpiece {
            assert_eq!(piece.color(), self.turn, "Cannot move enemy piece");
        } else {
            unreachable!("A piece must be moved");
        }
        if let BoardPiece::Piece(piece) = to_bpiece {
            assert_eq!(piece.color(), !self.turn, "Must capture enemy piece");
        }

        let state = StateChange {
            last_move: mv,
            captured: to_bpiece,
            last_ep_file: self.ep_file,
            last_castle_rights: self.castle_rights,
        };
        self.set_piece_at(mv.to(), from_bpiece);
        self.set_piece_at(mv.from(), BoardPiece::Empty);
        self.ep_file = None;

        match mv.move_type() {
            MoveType::Normal => {}

            MoveType::DoublePush => {
                assert_eq!(
                    mv.from().file(),
                    mv.to().file(),
                    "Double pawn push cannot change file"
                );
                // Set en-passant target
                self.ep_file = Some(mv.to().file());
            }

            MoveType::EnPassant => {
                // FIXME: Implement en-passant
                unimplemented!("Make en-passant move");
            }

            MoveType::Castle => {
                // FIXME: Implement castling
                unimplemented!("Make castle move");
            }

            MoveType::Promotion(_promo) => {
                // FIXME: Implement promotion
                unimplemented!("Make promotion move");
            }
        }

        self.history.push(state);
        // FIXME: Increment halfmove clock
        if self.turn == Color::Black {
            self.fullmove_count += 1;
        }
        self.turn = !self.turn;
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

#[derive(Copy, Clone)]
pub struct FenError;

impl Error for FenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<()> for FenError {
    fn from(_: ()) -> Self {
        Self
    }
}

impl fmt::Debug for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FenError(\"{}\")", self)
    }
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid FEN string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_to_default_fen() {
        let board = Board::default();
        let fen = board.to_fen();
        assert_eq!(fen, DEFAULT_FEN);
    }
}
