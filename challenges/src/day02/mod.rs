#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Day02 {
    passwords: Vec<Password>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordPolicy {
    amount_range: Range<usize>,
    symbol: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password {
    policy: PasswordPolicy,
    password: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Password, String> {
        let segments = password.split_whitespace().collect::<Vec<_>>();

        if segments.len() != 3 {
            return Err(format!("Unable to parse input line: {}", password));
        }

        let amount_range = str_to_range(segments[0])?;
        let symbol = {
            let mut symbol_chars = segments[1].to_string();
            symbol_chars.pop();
            symbol_chars
        };

        Ok(Password {
            policy: PasswordPolicy {
                amount_range,
                symbol,
            },
            password: segments[2].to_string(),
        })
    }
}

fn str_to_range(s: &str) -> Result<Range<usize>, String> {
    let pair: Result<Vec<usize>, _> = s
        .split('-')
        .map(|string| string.parse())
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    match pair {
        Err(_) => Err(format!("Failed to pair string to range: {}.", s)),
        Ok(endpoints) if endpoints.len() != 2 => {
            Err(format!("Failed to pair string to range: {}.", s))
        }
        Ok(endpoints) => Ok(Range {
            start: endpoints[0],
            end: endpoints[1],
        }),
    }
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
