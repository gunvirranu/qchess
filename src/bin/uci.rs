extern crate anyhow;
extern crate qchess;

use std::fmt;
use std::str::FromStr;
use std::{io, io::Write};

use qchess::{Game, Move};

#[derive(Clone, Debug)]
enum UciInput {
    UciFirst,
    Debug(bool),
    IsReady,
    UciNewGame,
    Position(Game),
    Go,
    Stop,
    Quit,
}

#[derive(Clone, Debug)]
enum UciOutput {
    Id,
    UciOk,
    ReadyOk,
    BestMove(Move),
    Info(String),
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
            _ => todo!("Not done yet"),
        }
    }
}

impl FromStr for UciInput {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let subs: Vec<_> = input.split_ascii_whitespace().collect();
        let first = *subs.get(0).ok_or(anyhow::anyhow!("No command found"))?;
        match (subs.len(), first) {
            (1, "uci") => Ok(UciInput::UciFirst),
            (1, "isready") => Ok(UciInput::IsReady),
            (1, "ucinewgame") => Ok(UciInput::UciNewGame),
            (1, "stop") => Ok(UciInput::Stop),
            (1, "quit") => Ok(UciInput::Quit),
            (2, "debug") => match subs[1] {
                "on" => Ok(UciInput::Debug(true)),
                "off" => Ok(UciInput::Debug(false)),
                _ => Err(anyhow::anyhow!("Must be `debug [on|off]`")),
            },
            (_, "position") => unimplemented!("`position` not supported yet"),
            (_, "go") => unimplemented!("`go` not supported yet"),
            _ => Err(anyhow::anyhow!("Unknown command: `{}`", first)),
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
            _ => todo!("Not done yet"),
        }
    }
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
