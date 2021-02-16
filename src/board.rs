use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::moves::StateChange;
use crate::{
    BoardPiece, CastlingRights, Color, File, Move, MoveType, PieceType, Rank, SidePiece, Square,
};

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const INIT_FEN_LEN: usize = 8 * 8 + 7 + 1 + 4 + 2 + 2 + 3 + 5;
const INIT_MOVE_HIST_LEN: usize = 32;
const INIT_MOVE_LIST_LEN: usize = 32;

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

    fn debug_validate_move(&self, mv: Move) {
        // TODO: Change assertions to debug
        let from_bpiece = self.piece_at(mv.from());
        let to_bpiece = self.piece_at(mv.to());

        if let BoardPiece::Piece(piece) = from_bpiece {
            assert_eq!(piece.color(), self.turn, "Cannot move enemy piece");
        } else {
            unreachable!("A piece must be moved");
        }
        if let BoardPiece::Piece(piece) = to_bpiece {
            assert_eq!(piece.color(), !self.turn, "Cannot capture own piece");
        }

        match mv.move_type() {
            MoveType::Normal => {}

            MoveType::DoublePush => {
                assert_eq!(
                    mv.from().file(),
                    mv.to().file(),
                    "Double pawn push cannot change file"
                );
            }

            MoveType::EnPassant => {
                assert_eq!(
                    Some(mv.to().file()),
                    self.ep_file,
                    "En-passant file must match board"
                );
                assert_eq!(
                    mv.from().rank(),
                    match self.turn {
                        Color::White => Rank::R5,
                        Color::Black => Rank::R4,
                    },
                    "En-passant must be from rank 5 (white) or 4 (black)"
                );
                assert_eq!(
                    mv.to().rank(),
                    match self.turn {
                        Color::White => Rank::R6,
                        Color::Black => Rank::R3,
                    },
                    "En-passant must be to rank 6 (white) or 3 (black)"
                );
                assert_eq!(
                    (mv.from().file() as i8 - mv.to().file() as i8).abs(),
                    1,
                    "En-passant must be a single diagonal step"
                );
                assert_eq!(
                    to_bpiece,
                    BoardPiece::Empty,
                    "En-passant to location must be empty"
                );
                if let Some(ep_pawn_sq) = mv.to().down(self.turn) {
                    assert_eq!(
                        self.piece_at(ep_pawn_sq),
                        BoardPiece::piece(PieceType::Pawn, !self.turn),
                        "Must be an enemy pawn behind en-passant square"
                    );
                } else {
                    unreachable!("Invalid en-passant square");
                }
            }

            MoveType::Castle => {}

            MoveType::Promotion(promo) => {
                assert_eq!(
                    mv.to().rank(),
                    match self.turn {
                        Color::White => Rank::R8,
                        Color::Black => Rank::R1,
                    },
                    "Promotion cannot occur on non-terminal rank"
                );
                assert!(
                    matches!(
                        promo,
                        PieceType::Rook | PieceType::Bishop | PieceType::Knight | PieceType::Queen
                    ),
                    "Promotion piece must be valid"
                );
            }
        }
    }

    pub fn make_move(&mut self, mv: Move) {
        self.debug_validate_move(mv);
        let from_bpiece = self.piece_at(mv.from());
        let to_bpiece = self.piece_at(mv.to());
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
                // Set en-passant target
                self.ep_file = Some(mv.to().file());
            }

            MoveType::EnPassant => {
                if let Some(ep_pawn_sq) = mv.to().down(self.turn) {
                    // Capture double-pushed pawn
                    self.set_piece_at(ep_pawn_sq, BoardPiece::Empty);
                } else {
                    unreachable!("Invalid en-passant square");
                }
            }

            MoveType::Castle => {
                // FIXME: Implement castling
                unimplemented!("Make castle move");
            }

            MoveType::Promotion(promo) => {
                // Promote pawn to promoted piece
                self.set_piece_at(mv.to(), BoardPiece::piece(promo, self.turn));
            }
        }

        self.history.push(state);
        // FIXME: Increment halfmove clock
        if self.turn == Color::Black {
            self.fullmove_count += 1;
        }
        self.turn = !self.turn;
    }

    pub fn undo_move(&mut self) -> Option<StateChange> {
        let state = self.history.pop()?;
        let mv = state.last_move;
        self.turn = !self.turn;
        // FIXME: Restore halfmove clock
        if self.turn == Color::Black {
            self.fullmove_count -= 1;
        }
        self.ep_file = state.last_ep_file;
        self.set_piece_at(mv.from(), self.piece_at(mv.to()));
        self.set_piece_at(mv.to(), state.captured);

        match mv.move_type() {
            MoveType::Normal | MoveType::DoublePush => {}

            MoveType::EnPassant => {
                if let Some(ep_pawn_sq) = mv.to().down(self.turn) {
                    // Restore captured pawn
                    self.set_piece_at(ep_pawn_sq, BoardPiece::piece(PieceType::Pawn, !self.turn));
                } else {
                    unreachable!("Invalid en-passant square");
                }
            }

            MoveType::Castle => {
                // TODO: Implement undo castling
                unimplemented!("Undo castle move");
            }

            MoveType::Promotion(_) => {
                // Restore pawn
                self.set_piece_at(mv.from(), BoardPiece::piece(PieceType::Pawn, self.turn));
            }
        }
        Some(state)
    }

    pub fn gen_pseudo_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(INIT_MOVE_LIST_LEN);
        for sq in Square::iter() {
            if let BoardPiece::Piece(piece) = self.piece_at(sq) {
                if piece.color() == self.turn {
                    match piece.piece_type() {
                        PieceType::Pawn => self.gen_pawn_moves(sq, &mut moves),
                        PieceType::Knight => self.gen_knight_moves(sq, &mut moves),
                        PieceType::King => self.gen_king_moves(sq, &mut moves),
                        PieceType::Rook => self.gen_rook_moves(sq, &mut moves),
                        _ => {}
                    }
                }
            }
        }
        moves
    }

    fn gen_pawn_moves(&self, sq: Square, moves: &mut Vec<Move>) {
        fn add_promo_moves(from: Square, to: Square, moves: &mut Vec<Move>) {
            for &promo in [PieceType::Queen, PieceType::Knight].iter() {
                moves.push(Move::new(from, to, MoveType::Promotion(promo)));
            }
        }

        let up = sq.up(self.turn).expect("Invalid rank for pawn");
        if self.piece_at(up) == BoardPiece::Empty {
            if let Some(up_up) = up.up(self.turn) {
                // Move forward
                moves.push(Move::normal(sq, up));
                // Double push
                if ((self.turn == Color::White && sq.rank() == Rank::R2)
                    || (self.turn == Color::Black && sq.rank() == Rank::R7))
                    && self.piece_at(up_up) == BoardPiece::Empty
                {
                    moves.push(Move::new(sq, up_up, MoveType::DoublePush));
                }
            } else {
                //  Pawn promotion
                add_promo_moves(sq, up, moves);
            }
        }

        for diag in [up.left(self.turn), up.right(self.turn)]
            .iter()
            .filter_map(|&x| x)
        {
            if let BoardPiece::Piece(capture) = self.piece_at(diag) {
                if capture.color() != self.turn {
                    if up.rank() == Rank::R1 || up.rank() == Rank::R1 {
                        // Pawn promotion and capture
                        add_promo_moves(sq, diag, moves);
                    } else {
                        // Capture diagonally
                        moves.push(Move::normal(sq, diag));
                    }
                }
            }
            // En-passant capture
            if self.ep_square() == Some(diag) {
                moves.push(Move::new(sq, diag, MoveType::EnPassant));
            }
        }
    }

    fn gen_knight_moves(&self, sq: Square, moves: &mut Vec<Move>) {
        for to in [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ]
        .iter()
        .map(|(dr, df)| (sq.rank() as i8 + dr, sq.file() as i8 + df))
        .filter_map(|coords| Square::try_from(coords).ok())
        {
            if match self.piece_at(to) {
                BoardPiece::Empty => true,
                BoardPiece::Piece(piece) => piece.color() != self.turn,
            } {
                moves.push(Move::normal(sq, to));
            }
        }
    }

    fn gen_king_moves(&self, sq: Square, moves: &mut Vec<Move>) {
        for vert in [sq.up(Color::White), Some(sq), sq.down(Color::White)]
            .iter()
            .filter_map(|&x| x)
        {
            for to in [
                vert.left(Color::White),
                if sq != vert { Some(vert) } else { None },
                vert.right(Color::White),
            ]
            .iter()
            .filter_map(|&x| x)
            {
                if match self.piece_at(to) {
                    BoardPiece::Empty => true,
                    BoardPiece::Piece(piece) => piece.color() != self.turn,
                } {
                    moves.push(Move::normal(sq, to));
                }
            }
        }
        // TODO: Generate castling moves
    }

    fn gen_rook_moves(&self, sq: Square, moves: &mut Vec<Move>) {
        for next in [Square::up, Square::down, Square::left, Square::right].iter() {
            let mut prev = sq;
            while let Some(to) = next(prev, Color::White) {
                prev = to;
                match self.piece_at(to) {
                    BoardPiece::Piece(piece) => {
                        if piece.color() != self.turn {
                            // Capture
                            moves.push(Move::normal(sq, to));
                        }
                        break;
                    }
                    BoardPiece::Empty => {
                        // Move to empty square
                        moves.push(Move::normal(sq, to));
                    }
                }
            }
        }
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
