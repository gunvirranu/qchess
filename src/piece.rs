use std::convert::TryFrom;
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

// Yes, this is basically an Option, but I wanted a type, not just an alias.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BoardPiece {
    Empty,
    Piece(SidePiece),
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

impl TryFrom<char> for SidePiece {
    type Error = ();

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        use SidePiece::*;
        match letter {
            'K' => Ok(WKing),
            'Q' => Ok(WQueen),
            'R' => Ok(WRook),
            'B' => Ok(WBishop),
            'N' => Ok(WKnight),
            'P' => Ok(WPawn),
            'k' => Ok(BKing),
            'q' => Ok(BQueen),
            'r' => Ok(BRook),
            'b' => Ok(BBishop),
            'n' => Ok(BKnight),
            'p' => Ok(BPawn),
            _ => Err(()),
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

impl BoardPiece {
    pub fn piece(piece_type: PieceType, color: Color) -> Self {
        Self::Piece(SidePiece::from((piece_type, color)))
    }
}

impl fmt::Debug for BoardPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Piece(side_piece) => write!(f, "({:?})", side_piece),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Color::*;
    use PieceType::*;
    use SidePiece::*;

    #[test]
    fn test_sidepiece_piece_type() {
        assert_eq!(WPawn.piece_type(), Pawn);
        assert_eq!(BPawn.piece_type(), Pawn);
        assert_eq!(WRook.piece_type(), Rook);
        assert_eq!(BRook.piece_type(), Rook);
        assert_eq!(WKnight.piece_type(), Knight);
        assert_eq!(WKnight.piece_type(), Knight);
        assert_eq!(WBishop.piece_type(), Bishop);
        assert_eq!(BBishop.piece_type(), Bishop);
        assert_eq!(WQueen.piece_type(), Queen);
        assert_eq!(BQueen.piece_type(), Queen);
        assert_eq!(WKing.piece_type(), King);
        assert_eq!(BKing.piece_type(), King);
    }

    #[test]
    fn test_sidepiece_color() {
        assert_eq!(WPawn.color(), White);
        assert_eq!(WRook.color(), White);
        assert_eq!(WKnight.color(), White);
        assert_eq!(WBishop.color(), White);
        assert_eq!(WQueen.color(), White);
        assert_eq!(WKing.color(), White);
        assert_eq!(BPawn.color(), Black);
        assert_eq!(BRook.color(), Black);
        assert_eq!(BKnight.color(), Black);
        assert_eq!(BBishop.color(), Black);
        assert_eq!(BQueen.color(), Black);
        assert_eq!(BKing.color(), Black);
    }

    #[test]
    fn test_sidepiece_from_piecetype_color() {
        assert_eq!(SidePiece::from((Pawn, White)), WPawn);
        assert_eq!(SidePiece::from((Pawn, Black)), BPawn);
        assert_eq!(SidePiece::from((Rook, White)), WRook);
        assert_eq!(SidePiece::from((Rook, Black)), BRook);
        assert_eq!(SidePiece::from((Knight, White)), WKnight);
        assert_eq!(SidePiece::from((Knight, Black)), BKnight);
        assert_eq!(SidePiece::from((Bishop, White)), WBishop);
        assert_eq!(SidePiece::from((Bishop, Black)), BBishop);
        assert_eq!(SidePiece::from((Queen, White)), WQueen);
        assert_eq!(SidePiece::from((Queen, Black)), BQueen);
        assert_eq!(SidePiece::from((King, White)), WKing);
        assert_eq!(SidePiece::from((King, Black)), BKing);
    }
}
