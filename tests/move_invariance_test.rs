extern crate qchess;

use qchess::*;

#[test]
#[ignore]
fn test_move_invariance_default() {
    let mut game = Game::default();
    let n = test_make_undo_move(&mut game, 5);
    eprintln!("Moves tested: {}", n);
}

fn test_make_undo_move(game: &mut Game, max_depth: u8) -> usize {
    if max_depth == 0 {
        return 1;
    }
    let og_fen = game.board.to_fen();
    let mut n = 1;
    for mv in game.board.gen_pseudo_moves() {
        game.make_move(mv);
        // Recurse to check sub-board configuration
        n += test_make_undo_move(game, max_depth - 1);
        game.undo_move();
        // TODO: Consider to clone or hash instead of FEN string
        // Assert board is same as before applying/undoing move
        assert_eq!(
            og_fen,
            game.board.to_fen(),
            "FEN doesn't match:\n{:?}\n{:?}",
            game,
            mv,
        );
    }
    n
}
