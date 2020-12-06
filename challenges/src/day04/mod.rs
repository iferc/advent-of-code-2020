#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};
use passports::{BasicPassport, StrictPassport, PASSPORT_SEPARATOR};

#[derive(Debug, Clone, PartialEq)]
pub struct Day04 {
    passports: Vec<BasicPassport>,
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
        Ok(self
            .passports
            .iter()
            .map(|passport| passport.has_required_fields())
            .fold(0, |acc, passport_validity| {
                acc + if passport_validity { 1 } else { 0 }
            }))
    }
}

impl GoldChallenge for Day04 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Ok(self
            .passports
            .clone()
            .into_iter()
            .map(|passport| StrictPassport::from(passport))
            .map(|passport| passport.is_valid())
            .fold(0, |acc, passport_validity| {
                acc + if passport_validity { 1 } else { 0 }
            }))
    }
}
