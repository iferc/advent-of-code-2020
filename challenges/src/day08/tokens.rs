use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    NoOperation(i64),
    Jump(i64),
    Accumulate(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndState {
    EndOfStream(i64),
    RecursionInterrupt(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream(Vec<Token>);

pub type TokenStreamError = &'static str;

impl TokenStream {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, token: Token) -> &Self {
        self.0.push(token);
        self
    }

    pub fn accumulate_until_tokens_repeat(&self) -> Result<EndState, TokenStreamError> {
        let mut cursor: usize = 0;
        let mut accumulation: i64 = 0;
        let mut execution_tracker = Vec::new();
        loop {
            // reached end of stream
            if cursor == self.0.len() {
                return Ok(EndState::EndOfStream(accumulation));
            }
            // reached an infinite loop
            if execution_tracker.contains(&cursor) {
                return Ok(EndState::RecursionInterrupt(accumulation));
            }

            execution_tracker.push(cursor);

            match self.0[cursor] {
                Token::NoOperation(_) => {
                    cursor += 1;
                }
                Token::Accumulate(value) => {
                    cursor += 1;
                    accumulation += value;
                }
                Token::Jump(jump_value) => {
                    cursor = self.move_cursor(cursor, jump_value)?;
                }
            }
        }
    }

    pub fn diagnose_bad_token(&self) -> Result<i64, TokenStreamError> {
        for (key, token) in self.0.iter().enumerate() {
            match token {
                &Token::Accumulate(_) => continue,
                &Token::Jump(value) => {
                    let mut test_token_stream = self.clone();
                    test_token_stream.0[key] = Token::NoOperation(value);
                    match test_token_stream.accumulate_until_tokens_repeat() {
                        Ok(EndState::EndOfStream(final_value)) => return Ok(final_value),
                        _ => continue,
                    }
                }
                &Token::NoOperation(value) => {
                    let mut test_token_stream = self.clone();
                    test_token_stream.0[key] = Token::Jump(value);
                    match test_token_stream.accumulate_until_tokens_repeat() {
                        Ok(EndState::EndOfStream(final_value)) => return Ok(final_value),
                        _ => continue,
                    }
                }
            }
        }

        Err("Diagnostic function did not detect a flippable instruction to repair this program.")
    }

    fn move_cursor(&self, cursor: usize, distance: i64) -> Result<usize, TokenStreamError> {
        let signed_cursor =
            i64::try_from(cursor).or(Err("Failed to parse cursor position, likely too large"))?;

        let new_cursor = signed_cursor + distance;
        if new_cursor < 0 {
            return Err("Attempted to jump to an instruction before the start of the program");
        }

        let final_cursor = usize::try_from(new_cursor).or(Err(
            "Failed to parse new cursor position, likely too large or negative",
        ))?;

        if final_cursor > self.0.len() {
            return Err("Attempted to jump to an instruction after the end of the program");
        }

        Ok(final_cursor)
    }
}
