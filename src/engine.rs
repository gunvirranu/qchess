use std::sync::mpsc;

use crate::Game;

#[derive(Clone, Debug)]
pub enum EngineCommand {
    SetGame(Game),
    Go(GoConfig),
    Stop,
}

#[derive(Clone, Debug)]
pub struct GoConfig {
    depth: u8,
}

pub fn engine_mainloop(rx: mpsc::Receiver<EngineCommand>) -> anyhow::Result<()> {
    let mut game;
    loop {
        let com = rx.recv()?;
        match com {
            EngineCommand::SetGame(g) => game = g,
            EngineCommand::Go(goconf) => {
                todo!();
            }
            EngineCommand::Stop => {
                todo!();
            }
        }
    }
}
