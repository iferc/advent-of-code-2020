mod questionaire;
#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};
use questionaire::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Day06 {
    questionaire_groups: Vec<QuestionaireGroup>,
}

impl Day06 {
    const GROUP_DELIM: &'static str = "\n\n";

    pub fn new(data: &str) -> Result<Self, String> {
        let mut questionaire_groups = Vec::new();

        let text_groups: Vec<_> = data.split(Self::GROUP_DELIM).collect();
        for text_group in text_groups {
            questionaire_groups.push(QuestionaireGroup::new(text_group));
        }

        Ok(Self {
            questionaire_groups,
        })
    }
}

impl SilverChallenge for Day06 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        let mut total = 0;
        for questionaire_group in &self.questionaire_groups {
            total += questionaire_group.sum_of_any_yes();
        }
        Ok(total)
    }
}

impl GoldChallenge for Day06 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        let mut total = 0;
        for questionaire_group in &self.questionaire_groups {
            total += questionaire_group.sum_of_all_yes();
        }
        Ok(total)
    }
}
