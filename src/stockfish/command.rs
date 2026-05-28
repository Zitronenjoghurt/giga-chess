//! Source: https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html#quit
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SfCommand {
    /// Will tell stockfish to use UCI
    Uci,
    /// Used to synchronize the engine with the GUI
    /// Required once before the engine is tasked with a search
    IsReady,
    /// Signals the engine to treat the following position as a new game
    /// Will also have to be synchronized through isready
    UciNewGame,
    /// Stop calculating as soon as possible
    Stop,
    /// Quit the engine as soon as possible
    Quit,
    /// Set up a position, if no FEN is given, the starting position is used
    /// If the position is from a new game, ucinewgame has to be sent first
    /// It's best to use the moves instead of the raw FEN so the engine has more game-knowledge (repetition, etc.)
    Position(SfPosition),
    SetOption {
        name: String,
        value: Option<String>,
    },
    /// Start searching for the best move
    Go(SfGo),
}

impl Display for SfCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uci => write!(f, "uci"),
            Self::IsReady => write!(f, "isready"),
            Self::UciNewGame => write!(f, "ucinewgame"),
            Self::Stop => write!(f, "stop"),
            Self::Quit => write!(f, "quit"),
            Self::Position(pos) => write!(f, "{}", pos),
            Self::SetOption { name, value } => {
                write!(f, "setoption name {} ", name)?;
                if let Some(value) = value {
                    write!(f, "value {}", value)?;
                }
                Ok(())
            }
            Self::Go(go) => write!(f, "{}", go),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SfPosition {
    pub fen: Option<String>,
    pub moves: Vec<String>,
}

impl Display for SfPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "position ")?;
        if let Some(fen) = &self.fen {
            write!(f, "fen {} ", fen)?;
        } else {
            write!(f, "startpos ")?;
        }
        write!(f, "moves {}", self.moves.join(" "))
    }
}

#[derive(Debug, Default, Clone)]
pub struct SfGo {
    /// Stop the search once this depth is reached
    pub depth: Option<u64>,
    /// Stop search if approaching this time in ms
    pub move_time: Option<u64>,
    /// Search until the stop command is sent
    pub infinite: bool,
    /// How much time white has left in ms
    pub white_time: Option<u64>,
    /// How much time black has left in ms
    pub black_time: Option<u64>,
    /// How much time white receives per move in ms
    pub white_inc: Option<u64>,
    /// How much time black receives per move in ms
    pub black_inc: Option<u64>,
    /// Restrict the search to the given moves
    pub search_moves: Vec<String>,
}

impl Display for SfGo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "go")?;
        if self.infinite {
            return write!(f, " infinite");
        }
        if let Some(depth) = self.depth {
            write!(f, " depth {}", depth)?;
        }
        if let Some(time) = self.move_time {
            write!(f, " movetime {}", time)?;
        }
        if let Some(time) = self.white_time {
            write!(f, " wtime {}", time)?;
        }
        if let Some(time) = self.black_time {
            write!(f, " btime {}", time)?;
        }
        if let Some(inc) = self.white_inc {
            write!(f, " winc {}", inc)?;
        }
        if let Some(inc) = self.black_inc {
            write!(f, " binc {}", inc)?;
        }
        if !self.search_moves.is_empty() {
            write!(f, " searchmoves {}", self.search_moves.join(" "))?;
        }
        Ok(())
    }
}
