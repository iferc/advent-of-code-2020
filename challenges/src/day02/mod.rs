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

fn str_to_range(s: &str) -> Result<Range<usize>, String> {
    let pair: Result<Vec<usize>, _> = s
        .split('-')
        .map(|string| string.parse())
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    match pair {
        Err(_) => Err(format!("Failed to pair string to range: {}.", s)),
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
            let segments = line.split_whitespace().collect::<Vec<_>>();

            if segments.len() != 3 {
                return Err(format!("Unable to parse input line: {}", line));
            }

            let amount_range = str_to_range(segments[0])?;
            let symbol = {
                let mut symbol_chars = segments[1].to_string();
                symbol_chars.pop();
                symbol_chars
            };

            passwords.push(Password {
                policy: PasswordPolicy {
                    amount_range,
                    symbol,
                },
                password: segments[2].to_string(),
            });
        }

        Ok(Day02 { passwords })
    }
}

#[test]
fn sample_1_parses_as_1_valid() {
    let input = "1-3 a: abcde".into();
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![Password {
                policy: PasswordPolicy {
                    amount_range: 1..3,
                    symbol: "a".to_string(),
                },
                password: "abcde".to_string(),
            }],
        })
    );
}

#[test]
fn sample_2_parses_as_0_valid() {
    let input = "1-3 b: cdefg".into();
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![Password {
                policy: PasswordPolicy {
                    amount_range: 1..3,
                    symbol: "b".to_string(),
                },
                password: "cdefg".to_string(),
            }],
        })
    );
}

#[test]
fn sample_3_parses_as_1_valid() {
    let input = "2-9 c: ccccccccc".into();
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![Password {
                policy: PasswordPolicy {
                    amount_range: 2..9,
                    symbol: "c".to_string(),
                },
                password: "ccccccccc".to_string(),
            }],
        })
    );
}

#[test]
fn sample_all_1_through_3_parses_as_2_valid() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".into();
    let challenge = Day02::new(input);
    assert_eq!(
        challenge,
        Ok(Day02 {
            passwords: vec![
                Password {
                    policy: PasswordPolicy {
                        amount_range: 1..3,
                        symbol: "a".to_string(),
                    },
                    password: "abcde".to_string(),
                },
                Password {
                    policy: PasswordPolicy {
                        amount_range: 1..3,
                        symbol: "b".to_string(),
                    },
                    password: "cdefg".to_string(),
                },
                Password {
                    policy: PasswordPolicy {
                        amount_range: 2..9,
                        symbol: "c".to_string(),
                    },
                    password: "ccccccccc".to_string(),
                }
            ],
        })
    );
}

impl SilverChallenge for Day02 {
    type Answer = usize;
    type Error = String;

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

#[test]
fn sample_1_silver_parses_as_1_valid() {
    let input = "1-3 a: abcde".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(1));
}

#[test]
fn sample_2_silver_parses_as_0_valid() {
    let input = "1-3 b: cdefg".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(0));
}

#[test]
fn sample_3_silver_parses_as_1_valid() {
    let input = "2-9 c: ccccccccc".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(1));
}

#[test]
fn sample_all_silver_1_through_3_parses_as_2_valid() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_silver();
    assert_eq!(answer.ok(), Some(2));
}

impl GoldChallenge for Day02 {
    type Answer = usize;
    type Error = String;

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

#[test]
fn sample_1_gold_parses_as_1_valid() {
    let input = "1-3 a: abcde".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(1));
}

#[test]
fn sample_2_gold_parses_as_0_valid() {
    let input = "1-3 b: cdefg".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(0));
}

#[test]
fn sample_3_gold_parses_as_1_valid() {
    let input = "2-9 c: ccccccccc".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(0));
}

#[test]
fn sample_all_gold_1_through_3_parses_as_2_valid() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".into();
    let mut challenge = Day02::new(input).unwrap();
    let answer = challenge.attempt_gold();
    assert_eq!(answer.ok(), Some(1));
}
