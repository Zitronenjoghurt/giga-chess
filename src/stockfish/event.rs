use crate::stockfish::error::{SfError, SfResult};
use crate::stockfish::reader::{FromTokens, TokenReader};

#[derive(Debug, Clone)]
pub enum SfEvent {
    Ok,
    ReadyOk,
    Id(SfId),
    Option(SfOption),
    BestMove {
        mv: String,
        ponder: Option<String>,
    },
    /// The engine outputting what it's thinking
    Info(SfInfo),
}

impl FromTokens for SfEvent {
    fn parse(reader: &mut TokenReader) -> SfResult<Self> {
        let event_type = reader.try_next()?;

        match event_type {
            "id" => Ok(SfEvent::Id(SfId::parse(reader)?)),
            "option" => Ok(SfEvent::Option(SfOption::parse(reader)?)),
            "readyok" => Ok(SfEvent::ReadyOk),
            "uciok" => Ok(SfEvent::Ok),
            "info" => Ok(SfEvent::Info(SfInfo::parse(reader)?)),
            "bestmove" => {
                let mv = reader.try_next()?.to_string();
                let ponder = match reader.peek() {
                    Some("ponder") => {
                        reader.try_next()?;
                        Some(reader.try_next()?.to_string())
                    }
                    _ => None,
                };
                Ok(SfEvent::BestMove { mv, ponder })
            }
            _ => Err(SfError::UnknownEvent {
                event_type: event_type.to_string(),
                data: reader.consume(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SfId {
    Name(String),
    Author(String),
}

impl FromTokens for SfId {
    fn parse(reader: &mut TokenReader) -> SfResult<Self> {
        let id_type = reader.try_next()?;
        match id_type {
            "name" => Ok(SfId::Name(reader.consume())),
            "author" => Ok(SfId::Author(reader.consume())),
            _ => Err(SfError::UnknownIdValue {
                id_type: id_type.to_string(),
                value: reader.consume(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SfOption {
    pub name: String,
    pub value: SfOptionValue,
}

impl FromTokens for SfOption {
    fn parse(reader: &mut TokenReader) -> SfResult<Self> {
        reader.assert_next("name")?;
        let name = reader.read_till("type")?;
        let value = SfOptionValue::parse(reader)?;
        Ok(Self { name, value })
    }
}

#[derive(Debug, Clone)]
pub enum SfOptionValue {
    Button,
    Check(bool),
    String(String),
    Spin { default: i64, min: i64, max: i64 },
}

impl FromTokens for SfOptionValue {
    fn parse(reader: &mut TokenReader) -> SfResult<Self> {
        let option_type = reader.try_next()?;

        match option_type {
            "button" => Ok(SfOptionValue::Button),
            "check" => Ok(SfOptionValue::Check(reader.parse_assert_prefix("default")?)),
            "string" => Ok(SfOptionValue::String(
                reader.consume_assert_prefix("default")?,
            )),
            "spin" => Ok(SfOptionValue::Spin {
                default: reader.parse_assert_prefix("default")?,
                min: reader.parse_assert_prefix("min")?,
                max: reader.parse_assert_prefix("max")?,
            }),
            _ => Err(SfError::UnknownOptionValue {
                value_type: option_type.to_string(),
                value: reader.consume(),
            }),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SfInfo {
    /// How many moves ahead the engine has fully searched
    pub depth: Option<u32>,
    /// Deepest line reached due to selective extensions, always >= depth
    pub selective_depth: Option<u32>,
    /// Which line this is when MultiPV > 1 (1-indexed)
    pub multi_pv: Option<u32>,
    /// Evaluation in centipawns or moves to mate, may have upper/lowerbound qualifier
    pub score: Option<SfScore>,
    /// Win/Draw/Loss probabilities out of 1000, only sent when UCI_ShowWDL is enabled
    pub wdl: Option<SfWdl>,
    /// Total positions evaluated so far
    pub nodes: Option<u64>,
    /// Nodes per second, the engine's raw speed
    pub nps: Option<u64>,
    /// How full the transposition table is, out of 1000
    pub hash_full: Option<u32>,
    /// How many endgame tablebase positions were looked up
    pub tablebase_hits: Option<u64>,
    /// Time spent searching in ms
    pub time: Option<u64>,
    /// The best sequence of moves found so far, always last in the info line
    pub pv: Vec<String>,
    /// The move currently being searched
    pub current_move: Option<String>,
    /// Which move number is currently being searched (1-indexed)
    pub current_move_number: Option<u32>,
    /// Arbitrary string message from the engine, eats everything remaining
    pub string: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SfScore {
    Cp { value: i32, bound: Option<SfBound> },
    Mate { value: i32, bound: Option<SfBound> },
}

#[derive(Debug, Clone)]
pub enum SfBound {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Default)]
pub struct SfWdl {
    pub win: u32,
    pub draw: u32,
    pub loss: u32,
}

impl FromTokens for SfInfo {
    fn parse(reader: &mut TokenReader) -> SfResult<Self> {
        let mut info = SfInfo::default();

        while let Ok(token) = reader.try_next() {
            match token {
                "depth" => info.depth = Some(reader.parse_next()?),
                "seldepth" => info.selective_depth = Some(reader.parse_next()?),
                "multipv" => info.multi_pv = Some(reader.parse_next()?),
                "nodes" => info.nodes = Some(reader.parse_next()?),
                "nps" => info.nps = Some(reader.parse_next()?),
                "hashfull" => info.hash_full = Some(reader.parse_next()?),
                "tbhits" => info.tablebase_hits = Some(reader.parse_next()?),
                "time" => info.time = Some(reader.parse_next()?),
                "currmove" => info.current_move = Some(reader.try_next()?.to_string()),
                "currmovenumber" => info.current_move_number = Some(reader.parse_next()?),
                "score" => {
                    let kind = reader.try_next()?;
                    let value: i32 = reader.parse_next()?;
                    let bound = match reader.peek() {
                        Some("upperbound") => {
                            reader.try_next()?;
                            Some(SfBound::Upper)
                        }
                        Some("lowerbound") => {
                            reader.try_next()?;
                            Some(SfBound::Lower)
                        }
                        _ => None,
                    };
                    info.score = Some(match kind {
                        "cp" => SfScore::Cp { value, bound },
                        "mate" => SfScore::Mate { value, bound },
                        _ => return Err(SfError::UnknownScoreType(kind.to_string())),
                    });
                }
                "wdl" => {
                    info.wdl = Some(SfWdl {
                        win: reader.parse_next()?,
                        draw: reader.parse_next()?,
                        loss: reader.parse_next()?,
                    });
                }
                "pv" => {
                    while let Ok(mv) = reader.try_next() {
                        info.pv.push(mv.to_string());
                    }
                    break;
                }
                "string" => {
                    info.string = Some(reader.consume());
                    break;
                }
                _ => {}
            }
        }

        Ok(info)
    }
}
