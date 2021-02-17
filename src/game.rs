use crate::moves::StateChange;
use crate::{Board, FenError, Move};
use std::fmt;

const INIT_MOVE_HIST_LEN: usize = 32;

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    history: Vec<StateChange>,
}

impl Game {
    pub fn from_board(board: Board) -> Self {
        Game {
            board,
            history: Vec::with_capacity(INIT_MOVE_HIST_LEN),
        }
    }

    pub fn empty() -> Self {
        Self::from_board(Board::empty())
    }

    pub fn default() -> Self {
        Self::from_board(Board::default())
    }

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        Board::from_fen(fen).map(Self::from_board)
    }

    pub fn make_move(&mut self, mv: Move) {
        let state = self.board.make_move(mv);
        self.history.push(state);
    }

    pub fn undo_move(&mut self) -> Option<StateChange> {
        let state = self.history.pop()?;
        self.board.undo_move(state);
        Some(state)
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
