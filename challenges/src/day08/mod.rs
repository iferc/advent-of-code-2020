#[cfg(test)]
mod tests;
mod tokens;

use crate::{GoldChallenge, SilverChallenge};
use tokens::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Day08 {
    tokens: TokenStream,
}

impl Day08 {
    pub fn new(data: &str) -> Result<Self, String> {
        let mut tokens = TokenStream::new();
        let lines = data.split("\n").collect::<Vec<_>>();

        for line in lines {
            let words = line.split_whitespace().collect::<Vec<_>>();
            if words.len() != 2 {
                return Err(format!("too many words found on line: {}", line));
            }

            tokens.push(match (words[0], words[1].parse()) {
                ("nop", Ok(value)) => Token::NoOperation(value),
                ("jmp", Ok(value)) => Token::Jump(value),
                ("acc", Ok(value)) => Token::Accumulate(value),
                _ => return Err(format!("Unrecognized instruction: {}", line)),
            });
        }

        Ok(Self { tokens })
    }
}

impl SilverChallenge for Day08 {
    type Answer = i64;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        match self.tokens.accumulate_until_tokens_repeat() {
            Ok(EndState::RecursionInterrupt(state)) => Ok(state),
            _ => Err("Recursion did not happen."),
        }
    }
}

impl GoldChallenge for Day08 {
    type Answer = i64;
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        self.tokens.diagnose_bad_token()
    }
}
