#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct Day09 {
    numbers: Vec<usize>,
}

impl Day09 {
    pub fn new(data: &str) -> Result<Self, String> {
        let numbers = data
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>();

        match numbers {
            Ok(numbers) => Ok(Self { numbers }),
            Err(_) => Err("Failed to parse input".to_string()),
        }
    }

    pub fn find_invalid_code(
        &self,
        preamble_length: usize,
        contiguous_length: usize,
    ) -> Option<usize> {
        if preamble_length >= self.numbers.len() {
            return None;
        }

        let mut matches = Vec::new();
        for (key, value) in self.numbers.iter().enumerate().skip(preamble_length) {
            if !Self::find_valid_pair(&self.numbers[(key - preamble_length)..key], *value) {
                matches.push(*value);
            } else {
                matches.clear();
            }

            if matches.len() == contiguous_length {
                return Some(matches.iter().sum());
            }
        }

        None
    }

    pub fn find_contiguous_range_to_invalid_code(&self, invalid_code: usize) -> Option<usize> {
        let mut matches = Vec::new();
        for (key_a, _) in self.numbers.iter().enumerate() {
            matches.clear();
            for (_, value_b) in self.numbers.iter().enumerate().skip(key_a) {
                matches.push(*value_b);
                let sum: usize = matches.iter().sum();
                if sum > invalid_code {
                    break;
                } else if matches.len() > 1 && sum == invalid_code {
                    matches.sort();
                    return Some(matches[0] + matches[matches.len() - 1]);
                }
            }
        }

        None
    }

    fn find_valid_pair(collection: &[usize], sum_value: usize) -> bool {
        for (key_a, value_a) in collection.iter().enumerate() {
            for (key_b, value_b) in collection.iter().enumerate() {
                if key_a == key_b {
                    continue;
                }
                if value_a + value_b == sum_value {
                    return true;
                }
            }
        }

        false
    }
}

impl SilverChallenge for Day09 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        match self.find_invalid_code(25, 1) {
            Some(value) => Ok(value),
            None => Err("No result found."),
        }
    }
}

impl GoldChallenge for Day09 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        let invalid_code = match self.find_invalid_code(25, 1) {
            Some(value) => value,
            None => return Err("No invalid code found."),
        };

        match self.find_contiguous_range_to_invalid_code(invalid_code) {
            None => Err("No invalid code found."),
            Some(sum) => Ok(sum),
        }
    }
}
