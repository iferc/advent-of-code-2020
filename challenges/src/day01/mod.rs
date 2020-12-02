use crate::{GoldChallenge, SilverChallenge};

pub struct Day01 {
    data: String,
}

impl Day01 {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl SilverChallenge for Day01 {
    type Answer = String;
    fn attempt_silver(&self) -> Result<Self::Answer, String> {
        Err("NYI".into())
    }
}

impl GoldChallenge for Day01 {
    type Answer = String;
    fn attempt_gold(&self) -> Result<Self::Answer, String> {
        Err("NYI".into())
    }
}
