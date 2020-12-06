mod password;
#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};
use password::Password;

#[derive(Debug, Clone, PartialEq)]
pub struct Day02 {
    passwords: Vec<Password>,
}

impl Day02 {
    pub fn new(data: &str) -> Result<Self, String> {
        let lines = data.lines().collect::<Vec<_>>();
        let mut passwords = Vec::with_capacity(lines.len());

        for line in lines {
            match Password::new(line) {
                Ok(password) => passwords.push(password),
                Err(error) => return Err(error),
            }
        }

        Ok(Day02 { passwords })
    }
}

impl SilverChallenge for Day02 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        let occurrence_counts: Vec<_> = self
            .passwords
            .iter()
            .map(|p| {
                let occurrence_count = p
                    .password
                    .matches(p.policy.symbol.as_str())
                    .collect::<Vec<_>>()
                    .len();

                p.policy.amount_range.start <= occurrence_count
                    && p.policy.amount_range.end >= occurrence_count
            })
            .filter(|validity| *validity)
            .collect();

        Ok(occurrence_counts.len())
    }
}

impl GoldChallenge for Day02 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        let occurrence_counts: Vec<_> = self
            .passwords
            .iter()
            .map(|p| {
                let password_chars: Vec<_> = p.password.chars().collect();
                let first_index_char = password_chars[p.policy.amount_range.start - 1].to_string();
                let second_index_char = password_chars[p.policy.amount_range.end - 1].to_string();

                match (first_index_char, second_index_char) {
                    (a, b) if a == p.policy.symbol && b != p.policy.symbol => true,
                    (a, b) if a != p.policy.symbol && b == p.policy.symbol => true,
                    _ => false,
                }
            })
            .filter(|validity| *validity)
            .collect();

        Ok(occurrence_counts.len())
    }
}
