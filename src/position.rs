use std::convert::TryFrom;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rank {
    R1, R2, R3, R4, R5, R6, R7, R8,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum File {
    A, B, C, D, E, F, G, H,
}

impl Square {
    pub fn rank(self) -> Rank {
        use Square::*;
        // TODO: Benchmark with manual bit version
        match self {
            A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1 => Rank::R1,
            A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2 => Rank::R2,
            A3 | B3 | C3 | D3 | E3 | F3 | G3 | H3 => Rank::R3,
            A4 | B4 | C4 | D4 | E4 | F4 | G4 | H4 => Rank::R4,
            A5 | B5 | C5 | D5 | E5 | F5 | G5 | H5 => Rank::R5,
            A6 | B6 | C6 | D6 | E6 | F6 | G6 | H6 => Rank::R6,
            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7 => Rank::R7,
            A8 | B8 | C8 | D8 | E8 | F8 | G8 | H8 => Rank::R8,
        }
    }

    pub fn file(self) -> File {
        use Square::*;
        // TODO: Benchmark against manual bit version
        match self {
            A1 | A2 | A3 | A4 | A5 | A6 | A7 | A8 => File::A,
            B1 | B2 | B3 | B4 | B5 | B6 | B7 | B8 => File::B,
            C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 => File::C,
            D1 | D2 | D3 | D4 | D5 | D6 | D7 | D8 => File::D,
            E1 | E2 | E3 | E4 | E5 | E6 | E7 | E8 => File::E,
            F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 => File::F,
            G1 | G2 | G3 | G4 | G5 | G6 | G7 | G8 => File::G,
            H1 | H2 | H3 | H4 | H5 | H6 | H7 | H8 => File::H,
        }
    }
}

impl From<(Rank, File)> for Square {
    fn from((rank, file): (Rank, File)) -> Self {
        // TODO: Benchmark against unsafe version
        Self::try_from((rank as u8, file as u8)).unwrap()
    }
}

impl TryFrom<(u8, u8)> for Square {
    type Error = ();

    fn try_from((rank, file): (u8, u8)) -> Result<Self, Self::Error> {
        if rank < 8 && file < 8 {
            Self::try_from(8 * rank + file)
        } else {
            Err(())
        }
    }
}

impl TryFrom<(i8, i8)> for Square {
    type Error = ();

    fn try_from((rank, file): (i8, i8)) -> Result<Self, Self::Error> {
        Self::try_from((rank as u8, file as u8))
    }
}

impl TryFrom<u8> for Square {
    type Error = ();

    #[rustfmt::skip]
    fn try_from(index: u8) -> Result<Self, Self::Error> {
        // TODO: Benchmark against `transmute`
        use Square::*;
        match index {
            // Generated via some Python code
             0 => Ok(A1),  1 => Ok(B1),  2 => Ok(C1),  3 => Ok(D1),
             4 => Ok(E1),  5 => Ok(F1),  6 => Ok(G1),  7 => Ok(H1),
             8 => Ok(A2),  9 => Ok(B2), 10 => Ok(C2), 11 => Ok(D2),
            12 => Ok(E2), 13 => Ok(F2), 14 => Ok(G2), 15 => Ok(H2),
            16 => Ok(A3), 17 => Ok(B3), 18 => Ok(C3), 19 => Ok(D3),
            20 => Ok(E3), 21 => Ok(F3), 22 => Ok(G3), 23 => Ok(H3),
            24 => Ok(A4), 25 => Ok(B4), 26 => Ok(C4), 27 => Ok(D4),
            28 => Ok(E4), 29 => Ok(F4), 30 => Ok(G4), 31 => Ok(H4),
            32 => Ok(A5), 33 => Ok(B5), 34 => Ok(C5), 35 => Ok(D5),
            36 => Ok(E5), 37 => Ok(F5), 38 => Ok(G5), 39 => Ok(H5),
            40 => Ok(A6), 41 => Ok(B6), 42 => Ok(C6), 43 => Ok(D6),
            44 => Ok(E6), 45 => Ok(F6), 46 => Ok(G6), 47 => Ok(H6),
            48 => Ok(A7), 49 => Ok(B7), 50 => Ok(C7), 51 => Ok(D7),
            52 => Ok(E7), 53 => Ok(F7), 54 => Ok(G7), 55 => Ok(H7),
            56 => Ok(A8), 57 => Ok(B8), 58 => Ok(C8), 59 => Ok(D8),
            60 => Ok(E8), 61 => Ok(F8), 62 => Ok(G8), 63 => Ok(H8),
            _ => Err(()),
        }
    }
}

impl TryFrom<i8> for Square {
    type Error = ();

    fn try_from(index: i8) -> Result<Self, Self::Error> {
        Self::try_from(index as u8)
    }
}
