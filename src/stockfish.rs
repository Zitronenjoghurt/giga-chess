use crate::stockfish::command::SfCommand;
use crate::stockfish::error::SfResult;
use crate::stockfish::event::SfEvent;
use crate::stockfish::reader::{FromTokens, TokenReader};

pub mod command;
pub mod error;
pub mod event;
pub mod reader;

pub struct StockfishManager {
    outbox: Vec<SfCommand>,
}

impl Default for StockfishManager {
    fn default() -> Self {
        let mut manager = Self { outbox: Vec::new() };
        manager.send(SfCommand::Uci);
        manager
    }
}

impl StockfishManager {
    pub fn read_line(&mut self, line: &str) -> SfResult<Option<SfEvent>> {
        if line.trim().is_empty() {
            return Ok(None);
        }
        let mut reader = TokenReader::new(line);
        let event = SfEvent::parse(&mut reader)?;
        Ok(Some(event))
    }

    pub fn drain_commands(&mut self) -> Vec<SfCommand> {
        std::mem::take(&mut self.outbox)
    }
}

// Commands
impl StockfishManager {
    pub fn send(&mut self, cmd: SfCommand) {
        self.outbox.push(cmd);
    }
}
