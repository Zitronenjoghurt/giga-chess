use chrono::Local;

#[derive(Default)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PGNMetadata {
    event: Option<String>,
    site: Option<String>,
    date: Option<String>,
    round: Option<String>,
    white: Option<String>,
    black: Option<String>,
}

impl PGNMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn now() -> Self {
        let date = Local::now().format("%Y.%m.%d").to_string();
        Self {
            date: Some(date),
            ..Self::default()
        }
    }

    pub fn format(&self, result: Option<&str>, fen: Option<&str>) -> String {
        let mut pgn = String::new();
        if let Some(event) = &self.event {
            pgn.push_str(&format!("[Event \"{}\"]\n", event));
        }
        if let Some(site) = &self.site {
            pgn.push_str(&format!("[Site \"{}\"]\n", site));
        }
        if let Some(date) = &self.date {
            pgn.push_str(&format!("[Date \"{}\"]\n", date));
        }
        if let Some(round) = &self.round {
            pgn.push_str(&format!("[Round \"{}\"]\n", round));
        }
        if let Some(white) = &self.white {
            pgn.push_str(&format!("[White \"{}\"]\n", white));
        }
        if let Some(black) = &self.black {
            pgn.push_str(&format!("[Black \"{}\"]\n", black));
        }
        if let Some(result) = result {
            pgn.push_str(&format!("[Result \"{}\"]\n", result));
        }
        if let Some(fen) = fen {
            pgn.push_str(&format!("[FEN \"{}\"]\n", fen));
        }
        pgn
    }

    pub fn event(self, event: &str) -> Self {
        Self {
            event: Some(event.to_string()),
            ..self
        }
    }

    pub fn site(self, site: &str) -> Self {
        Self {
            site: Some(site.to_string()),
            ..self
        }
    }

    pub fn date(self, date: &str) -> Self {
        Self {
            date: Some(date.to_string()),
            ..self
        }
    }

    pub fn round(self, round: &str) -> Self {
        Self {
            round: Some(round.to_string()),
            ..self
        }
    }

    pub fn white(self, white: &str) -> Self {
        Self {
            white: Some(white.to_string()),
            ..self
        }
    }

    pub fn black(self, black: &str) -> Self {
        Self {
            black: Some(black.to_string()),
            ..self
        }
    }
}
