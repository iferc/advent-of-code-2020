#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct BoardingPass {
    id: u32,
    row: u32,
    col: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Day05 {
    boarding_passes: Vec<BoardingPass>,
}

impl Day05 {
    pub fn new(data: &str) -> Result<Self, String> {
        Ok(Self { data: data.into() })
    }
}

impl SilverChallenge for Day05 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Ok(self.boarding_passes.iter().max_by_key(|x| x.id).unwrap().id)
    }
}

impl GoldChallenge for Day05 {
    type Answer = ();
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        self.boarding_passes.sort_by_key(|x| x.id);

        let mut current_id = self.boarding_passes[0].id - 1;
        for boarding_pass in &self.boarding_passes {
            if boarding_pass.id > current_id + 1 {
                return Ok(current_id + 1);
            }
            current_id = boarding_pass.id;
        }

        Err("Match not found")
    }
}
