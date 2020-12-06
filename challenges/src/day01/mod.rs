#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct Day01 {
    values_asc: Vec<u32>,
    values_desc: Vec<u32>,
}

impl Day01 {
    pub fn new(data: &str) -> Result<Self, String> {
        let maybe_lines: Result<Vec<u32>, _> = data.split_whitespace().map(|s| s.parse()).collect();
        let lines = match maybe_lines {
            Err(_) => return Err("Error parsing number values from input".into()),
            Ok(mut lines) => {
                lines.sort_unstable();
                lines
            }
        };
        let reverse_lines: Vec<_> = lines.clone().into_iter().rev().collect();

        Ok(Self {
            values_asc: lines,
            values_desc: reverse_lines,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SilverSolution {
    numbers: [u32; 2],
    result: u32,
}
impl SilverChallenge for Day01 {
    type Answer = SilverSolution;
    type Error = String;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error> {
        for asc_line in &self.values_asc {
            for desc_line in &self.values_desc {
                // end iteration on collision
                if asc_line > desc_line {
                    break;
                }

                if 2020 == asc_line + desc_line {
                    return Ok(SilverSolution {
                        numbers: [*asc_line, *desc_line],
                        result: asc_line * desc_line,
                    });
                }
            }
        }

        Err("No solution found!".into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoldSolution {
    numbers: [u32; 3],
    result: u32,
}
impl GoldChallenge for Day01 {
    type Answer = GoldSolution;
    type Error = String;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error> {
        for asc_line_1 in &self.values_asc {
            for asc_line_2 in &self.values_asc {
                if asc_line_1 > asc_line_2 {
                    continue;
                }
                for desc_line in &self.values_desc {
                    // end iteration on collision
                    if asc_line_1 > desc_line {
                        break;
                    }

                    if 2020 == asc_line_1 + asc_line_2 + desc_line {
                        // return Ok(format!("{}", asc_line_1 * asc_line_2 * desc_line));
                        return Ok(GoldSolution {
                            numbers: [*asc_line_1, *asc_line_2, *desc_line],
                            result: asc_line_1 * asc_line_2 * desc_line,
                        });
                    }
                }
            }
        }

        Err("No solution found!".into())
    }
}
