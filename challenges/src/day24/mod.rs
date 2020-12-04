use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct Day24 {
    data: String,
}

impl Day24 {
    pub fn new(data: String) -> Result<Self, String> {
        Ok(Self { data })
    }
}

impl SilverChallenge for Day24 {
    type Answer = ();
    fn attempt_silver(&mut self) -> Result<Self::Answer, String>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI".into())
    }
}

impl GoldChallenge for Day24 {
    type Answer = ();
    fn attempt_gold(&mut self) -> Result<Self::Answer, String>
    where
        Self::Answer: std::fmt::Debug,
    {
        Err("NYI".into())
    }
}
