use crate::game::outcome::GameOutcome;
use crate::prelude::{Color, Session};
use crate::session::config::StartingPosition;
use std::fmt::Write;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "bit-codec",
    derive(bit_codec::BitEncode, bit_codec::BitDecode)
)]
pub struct PgnHeaders {
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<String>,
    pub round: Option<String>,
    pub white: Option<String>,
    pub black: Option<String>,
    pub extra: Vec<(String, String)>,
}

pub fn session_pgn(session: &Session) -> String {
    let mut pgn = String::new();
    let h = &session.config().pgn;
    let result = outcome_pgn(session.game().outcome());

    write_tag(&mut pgn, "Event", h.event.as_deref().unwrap_or("?"));
    write_tag(&mut pgn, "Site", h.site.as_deref().unwrap_or("?"));
    write_tag(&mut pgn, "Date", h.date.as_deref().unwrap_or("????.??.??"));
    write_tag(&mut pgn, "Round", h.round.as_deref().unwrap_or("?"));
    write_tag(&mut pgn, "White", h.white.as_deref().unwrap_or("?"));
    write_tag(&mut pgn, "Black", h.black.as_deref().unwrap_or("?"));
    write_tag(&mut pgn, "Result", result);

    if let StartingPosition::Fen(fen) = &session.config().starting_position {
        write_tag(&mut pgn, "SetUp", "1");
        write_tag(&mut pgn, "FEN", fen);
    }

    for (key, value) in &h.extra {
        write_tag(&mut pgn, key, value);
    }

    pgn.push('\n');

    let mut line_len = 0;
    for (i, san) in session.san_history().iter().enumerate() {
        let mut token = String::new();
        if i % 2 == 0 {
            write!(&mut token, "{}. ", i / 2 + 1).unwrap();
        }
        token.push_str(san);

        if line_len + token.len() + 1 > 80 && line_len > 0 {
            pgn.push('\n');
            line_len = 0;
        } else if line_len > 0 {
            pgn.push(' ');
            line_len += 1;
        }

        pgn.push_str(&token);
        line_len += token.len();
    }

    if !session.san_history().is_empty() {
        pgn.push(' ');
    }
    pgn.push_str(result);

    pgn.push('\n');

    pgn
}

pub fn outcome_pgn(outcome: Option<GameOutcome>) -> &'static str {
    match outcome {
        Some(GameOutcome::Decisive {
            winner: Color::White,
            ..
        }) => "1-0",
        Some(GameOutcome::Decisive {
            winner: Color::Black,
            ..
        }) => "0-1",
        Some(GameOutcome::Draw(_)) => "1/2-1/2",
        None => "*",
    }
}

fn write_tag(pgn: &mut String, key: &str, value: &str) {
    writeln!(pgn, "[{key} \"{value}\"]").unwrap();
}

// ToDo: Parse from PGN
