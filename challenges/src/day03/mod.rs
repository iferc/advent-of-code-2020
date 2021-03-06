#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct Day03 {
    data: String,
}

impl Day03 {
    pub fn new(data: &str) -> Result<Self, String> {
        Ok(Self { data: data.into() })
    }
}

impl SilverChallenge for Day03 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI")
    }
}

impl GoldChallenge for Day03 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI")
    }
}
