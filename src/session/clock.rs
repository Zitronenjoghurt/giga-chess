use crate::prelude::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChessClock {
    /// Remaining time in milliseconds
    remaining_ms: [u64; 2],
    /// Fischer time increment in milliseconds
    increment_ms: [u64; 2],
    /// Milliseconds since UNIX epoch
    last_move_timestamp_ms: Option<u64>,
    active: Color,
}

impl ChessClock {
    pub fn new(time_ms: u64, increment_ms: u64) -> Self {
        Self {
            remaining_ms: [time_ms, time_ms],
            increment_ms: [increment_ms, increment_ms],
            last_move_timestamp_ms: None,
            active: Color::White,
        }
    }

    pub fn from_config(config: &ChessClockConfig) -> Self {
        Self {
            remaining_ms: [config.white_ms, config.black_ms],
            increment_ms: [config.white_inc_ms, config.black_inc_ms],
            last_move_timestamp_ms: None,
            active: Color::White,
        }
    }

    pub fn switch(&mut self, now_ms: u64) -> bool {
        let index = self.active as usize;
        if let Some(last) = self.last_move_timestamp_ms {
            let elapsed = now_ms.saturating_sub(last);
            self.remaining_ms[index] = self.remaining_ms[index].saturating_sub(elapsed);
            self.remaining_ms[index] += self.increment_ms[index];
        }
        self.last_move_timestamp_ms = Some(now_ms);

        let timeout = self.remaining_ms[index] == 0;
        self.active = self.active.opposite();
        timeout
    }

    pub fn remaining_ms(&self, color: Color, now_ms: u64) -> u64 {
        let base = self.remaining_ms[color as usize];
        if color == self.active {
            if let Some(last) = self.last_move_timestamp_ms {
                let elapsed = now_ms.saturating_sub(last);
                base.saturating_sub(elapsed)
            } else {
                base
            }
        } else {
            base
        }
    }

    pub fn increment_ms(&self, color: Color) -> u64 {
        self.increment_ms[color as usize]
    }

    pub fn is_out_of_time(&self, color: Color, now_ms: u64) -> bool {
        self.remaining_ms(color, now_ms) == 0
    }

    pub fn active(&self) -> Color {
        self.active
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChessClockConfig {
    pub white_ms: u64,
    pub black_ms: u64,
    pub white_inc_ms: u64,
    pub black_inc_ms: u64,
}
