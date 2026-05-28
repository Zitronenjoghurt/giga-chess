use crate::stockfish::error::{SfError, SfResult};
use std::iter::Peekable;
use std::str::FromStr;

pub struct TokenReader<'a> {
    tokens: Peekable<std::str::SplitWhitespace<'a>>,
}

impl<'a> TokenReader<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            tokens: line.split_whitespace().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&str> {
        self.tokens.peek().copied()
    }

    pub fn try_next(&mut self) -> SfResult<&'a str> {
        self.tokens.next().ok_or(SfError::UnexpectedEof)
    }

    pub fn assert_next(&mut self, expected: &'a str) -> SfResult<()> {
        let token = self.try_next()?;
        if token == expected {
            Ok(())
        } else {
            Err(SfError::UnexpectedToken {
                expected: expected.to_string(),
                got: token.to_string(),
            })
        }
    }

    pub fn parse_next<T: FromStr>(&mut self) -> SfResult<T> {
        let token = self.try_next()?;
        token
            .parse::<T>()
            .map_err(|_| SfError::ParseFailed(token.to_string()))
    }

    pub fn parse_assert_prefix<T: FromStr>(&mut self, prefix: &'a str) -> SfResult<T> {
        self.assert_next(prefix)?;
        self.parse_next()
    }

    /// Consumes the terminator and swallows it
    pub fn read_till(&mut self, terminator: &str) -> SfResult<String> {
        let mut result = String::new();
        let mut found = false;

        while let Some(token) = self.tokens.clone().next() {
            if token == terminator {
                let _ = self.tokens.next();
                found = true;
                break;
            }

            let target = self.tokens.next().unwrap();

            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(target);
        }

        if !found {
            return Err(SfError::MissingToken(terminator.to_string()));
        }

        Ok(result)
    }

    pub fn consume(&mut self) -> String {
        let rest: Vec<&str> = self.tokens.by_ref().collect();
        rest.join(" ")
    }

    pub fn consume_assert_prefix(&mut self, prefix: &'a str) -> SfResult<String> {
        self.assert_next(prefix)?;
        Ok(self.consume())
    }
}

pub trait FromTokens: Sized {
    fn parse(reader: &mut TokenReader) -> SfResult<Self>;
}
