extern crate qchess;

use qchess::*;

#[test]
#[ignore]
fn test_move_invariance_default() {
    let mut board = Board::default();
    let n = test_make_undo_move(&mut board, 5);
    eprintln!("Moves tested: {}", n);
}

fn test_make_undo_move(board: &mut Board, max_depth: u8) -> usize {
    if max_depth == 0 {
        return 0;
    }
    let og_fen = board.to_fen();
    let moves = board.gen_pseudo_moves();
    let mut n = moves.len();
    for mv in moves {
        board.make_move(mv);
        // Recurse to check sub-board configuration
        n += test_make_undo_move(board, max_depth - 1);
        board.undo_move();
        // TODO: Consider to clone or hash instead of FEN string
        // Assert board is same as before applying/undoing move
        assert_eq!(
            og_fen,
            board.to_fen(),
            "FEN doesn't match:\n{:?}\n{:?}",
            board,
            mv,
        );
    }
    n
}
