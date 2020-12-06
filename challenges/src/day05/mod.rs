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
        let found_boarding_passes: Vec<_> = data.split_whitespace().collect();

        let mut boarding_passes = Vec::with_capacity(found_boarding_passes.len());
        for boarding_pass in found_boarding_passes {
            if boarding_pass.len() != 10 {
                return Err("Invalid boarding pass length.".to_string());
            }

            let row_digits: Result<String, _> = boarding_pass
                .chars()
                .take(7)
                .map(|ch| match ch {
                    'F' => Ok('0'),
                    'B' => Ok('1'),
                    _ => return Err(format!("Unrecognized format for row: {}.", ch)),
                })
                .collect();

            let col_digits: Result<String, _> = boarding_pass
                .chars()
                .skip(7)
                .take(3)
                .map(|ch| match ch {
                    'L' => Ok('0'),
                    'R' => Ok('1'),
                    _ => return Err(format!("Unrecognized format for col: {}.", ch)),
                })
                .collect();

            match (row_digits, col_digits) {
                (Ok(row_str), Ok(col_str)) => {
                    let id_str: String = format!("{}{}", row_str, col_str);

                    let row = u32::from_str_radix(row_str.as_str(), 2);
                    let col = u32::from_str_radix(col_str.as_str(), 2);
                    let id = u32::from_str_radix(id_str.as_str(), 2);

                    match (row, col, id) {
                        (Ok(row), Ok(col), Ok(id)) => {
                            boarding_passes.push(BoardingPass { id, row, col })
                        }
                        _ => return Err("Failed to parse to binary".to_string()),
                    }
                }
                _ => return Err("Failed to collect and convert characters".to_string()),
            }
        }

        Ok(Self { boarding_passes })
    }
}

impl SilverChallenge for Day05 {
    type Answer = u32;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Ok(self.boarding_passes.iter().max_by_key(|x| x.id).unwrap().id)
    }
}

impl GoldChallenge for Day05 {
    type Answer = u32;
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
