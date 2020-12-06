#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct Day04 {
    data: String,
}

impl Day04 {
    pub fn new(data: &str) -> Result<Self, String> {
        Ok(Self { data: data.into() })
    }
}

impl SilverChallenge for Day04 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI".into())
    }
}

impl GoldChallenge for Day04 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI".into())
    }
}
