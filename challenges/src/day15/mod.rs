use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug)]
pub struct Day15 {
    data: String,
}

impl Day15 {
    pub fn new(data: String) -> Result<Self, String> {
        Ok(Self { data })
    }
}

impl SilverChallenge for Day15 {
    type Answer = ();
    fn attempt_silver(&self) -> Result<Self::Answer, String>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI".into())
    }
}

impl GoldChallenge for Day15 {
    type Answer = ();
    fn attempt_gold(&self) -> Result<Self::Answer, String>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI".into())
    }
}