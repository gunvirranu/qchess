extern crate anyhow;
extern crate qchess;

use std::io;
use std::str::FromStr;

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

fn ui_mainloop() -> anyhow::Result<()> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command: UciInput = input.trim().parse()?;
        dbg!(command);
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

fn main() -> anyhow::Result<()> {
    ui_mainloop()
}
