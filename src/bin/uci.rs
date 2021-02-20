extern crate anyhow;
extern crate qchess;

use std::fmt;
use std::str::FromStr;
use std::{io, io::Write};

use anyhow::{anyhow, bail};

use qchess::{BoardPiece, Game, Move, MoveType, PieceType};

#[derive(Clone, Debug)]
enum UciInput {
    UciFirst,
    Debug(bool),
    IsReady,
    UciNewGame,
    Position(Game),
    // Go,
    Stop,
    Quit,
}

#[derive(Clone, Debug)]
enum UciOutput {
    Id,
    UciOk,
    ReadyOk,
    // BestMove(Move),
    // Info(String),
}

fn ui_mainloop() -> anyhow::Result<()> {
    loop {
        let command = get_input_command()?;
        match command {
            UciInput::UciFirst => {
                reply(UciOutput::Id)?;
                reply(UciOutput::UciOk)?;
            }
            UciInput::IsReady => reply(UciOutput::ReadyOk)?,
            UciInput::Quit => return Ok(()),
            UciInput::Debug(_) | UciInput::UciNewGame => {}
            UciInput::Position(game) => {
                eprintln!("{:?}", game);
            }
            _ => todo!("Not done yet"),
        }
    }
}

impl FromStr for UciInput {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let subs: Vec<_> = input.split_ascii_whitespace().collect();
        let (&first, args) = subs
            .split_first()
            .ok_or_else(|| anyhow!("No command found"))?;
        match (args.len(), first) {
            (0, "uci") => Ok(UciInput::UciFirst),
            (0, "isready") => Ok(UciInput::IsReady),
            (0, "ucinewgame") => Ok(UciInput::UciNewGame),
            (0, "stop") => Ok(UciInput::Stop),
            (0, "quit") => Ok(UciInput::Quit),
            (1, "debug") => match args[0] {
                "on" => Ok(UciInput::Debug(true)),
                "off" => Ok(UciInput::Debug(false)),
                _ => Err(anyhow!("Must be `debug [on|off]`")),
            },
            (n, "position") if n >= 1 => {
                let game = gen_game_from_uci(input, args)?;
                Ok(UciInput::Position(game))
            }
            (_, "go") => unimplemented!("`go` not supported yet"),
            _ => Err(anyhow!("Unvalid command: `{}`", input)),
        }
    }
}

impl fmt::Display for UciOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(
                f,
                "id name {} {}\nid author {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_AUTHORS"),
            ),
            Self::UciOk => write!(f, "uciok"),
            Self::ReadyOk => write!(f, "readyok"),
        }
    }
}

fn gen_game_from_uci(input: &str, args: &[&str]) -> anyhow::Result<Game> {
    let (leftover, mut game) = match args[0] {
        "startpos" => (&args[1..], Game::default()),
        "fen" if args.len() >= 7 => {
            let fen = args[1..(1 + 6)].join(" ");
            (&args[(1 + 6)..], Game::from_fen(&fen)?)
        }
        _ => bail!("Invalid position: `{}`", input),
    };
    if let Some((&is_moves, moves)) = leftover.split_first() {
        if is_moves != "moves" {
            bail!("Invalid option after position: `{}`", input);
        }
        for &move_str in moves {
            let mv = move_str
                .parse()
                .and_then(|og_mv| fix_move(&game, og_mv))
                .map_err(|_| anyhow!("Invalid move: `{}`", move_str))?;
            game.make_move(mv);
        }
    }
    Ok(game)
}

fn fix_move(game: &Game, mv: Move) -> Result<Move, ()> {
    let piece = match game.board.piece_at(mv.from()) {
        BoardPiece::Piece(p) => p,
        BoardPiece::Empty => return Err(()),
    };
    let to_bpiece = game.board.piece_at(mv.to());
    // These are _not_ comprehensive checks, just the bare minimum. Assumes valid moves.
    let mv_type = match piece.piece_type() {
        PieceType::Pawn => {
            if (mv.from().rank() as i8 - mv.to().rank() as i8).abs() == 2 {
                MoveType::DoublePush
            } else if mv.from().file() != mv.to().file() && to_bpiece == BoardPiece::Empty {
                MoveType::EnPassant
            } else {
                // Promotion has already been handled by `parse()`
                mv.move_type()
            }
        }
        PieceType::King => {
            if (mv.from().file() as i8 - mv.to().file() as i8).abs() > 1 {
                MoveType::Castle
            } else {
                mv.move_type()
            }
        }
        _ => mv.move_type(),
    };
    Ok(Move::new(mv.from(), mv.to(), mv_type))
}

fn get_input_command() -> anyhow::Result<UciInput> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse()
}

fn reply(ret: UciOutput) -> io::Result<()> {
    writeln!(io::stdout(), "{}", ret)
}

fn main() -> anyhow::Result<()> {
    ui_mainloop()
}
