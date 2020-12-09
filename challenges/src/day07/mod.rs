mod bags;
#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};
use bags::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Day07 {
    rules: HashMap<String, Vec<(i64, String)>>,
}

impl Day07 {
    pub fn new(data: &str) -> Result<Self, String> {
        let mut rules = HashMap::new();
        let lines = data.split("\n");
        for line in lines {
            let bag: Bag = line.parse().unwrap();
            rules.insert(bag.0, bag.1);
        }
        Ok(Self { rules })
    }
}

impl SilverChallenge for Day07 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Ok(Bags(self.rules.clone()).silver_find("shiny gold"))
    }
}

impl GoldChallenge for Day07 {
    type Answer = i64;
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Ok(Bags(self.rules.clone()).gold_find("shiny gold"))
    }
}
