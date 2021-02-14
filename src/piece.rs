use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SidePiece {
    WPawn,
    WRook,
    WKnight,
    WBishop,
    WQueen,
    WKing,
    BPawn,
    BRook,
    BKnight,
    BBishop,
    BQueen,
    BKing,
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl SidePiece {
    pub fn piece_type(self) -> PieceType {
        use PieceType::*;
        use SidePiece::*;
        // TODO: Benchmark against bit conversion
        match self {
            WPawn | BPawn => Pawn,
            WRook | BRook => Rook,
            WKnight | BKnight => Knight,
            WBishop | BBishop => Bishop,
            WQueen | BQueen => Queen,
            WKing | BKing => King,
        }
    }

    pub fn color(self) -> Color {
        use SidePiece::*;
        // TODO: Benchmark against bit conversion
        match self {
            WPawn | WRook | WKnight | WBishop | WQueen | WKing => Color::White,
            BPawn | BRook | BKnight | BBishop | BQueen | BKing => Color::Black,
        }
    }
}

impl From<(PieceType, Color)> for SidePiece {
    fn from((piece_type, color): (PieceType, Color)) -> Self {
        use PieceType::*;
        use SidePiece::*;
        // TODO: Benchmark against bit construction
        match color {
            Color::White => match piece_type {
                Pawn => WPawn,
                Rook => WRook,
                Knight => WKnight,
                Bishop => WBishop,
                Queen => WQueen,
                King => WKing,
            },
            Color::Black => match piece_type {
                Pawn => BPawn,
                Rook => BRook,
                Knight => BKnight,
                Bishop => BBishop,
                Queen => BQueen,
                King => BKing,
            },
        }
    }
}

impl fmt::Debug for SidePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SidePiece::*;
        let c = match self {
            WPawn => 'P',
            WRook => 'R',
            WKnight => 'N',
            WBishop => 'B',
            WQueen => 'Q',
            WKing => 'K',
            BPawn => 'p',
            BRook => 'r',
            BKnight => 'n',
            BBishop => 'b',
            BQueen => 'q',
            BKing => 'k',
        };
        write!(f, "{}", c)
    }
}

impl fmt::Display for SidePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SidePiece::*;
        let c = match self {
            WPawn => '♙',
            WRook => '♖',
            WKnight => '♘',
            WBishop => '♗',
            WQueen => '♕',
            WKing => '♔',
            BPawn => '♟',
            BRook => '♜',
            BKnight => '♞',
            BBishop => '♝',
            BQueen => '♛',
            BKing => '♚',
        };
        write!(f, "{}", c)
    }
}
