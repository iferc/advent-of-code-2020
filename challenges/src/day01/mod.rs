#[cfg(test)]
mod tests;

use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub struct Day01 {
    values_asc: Vec<usize>,
    values_desc: Vec<usize>,
}

impl Day01 {
    pub fn new(data: &str) -> Result<Self, String> {
        let maybe_lines: Result<Vec<usize>, _> =
            data.split_whitespace().map(|s| s.parse()).collect();
        let lines = match maybe_lines {
            Err(_) => return Err("Error parsing number values from input".to_string()),
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
    numbers: [usize; 2],
    result: usize,
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

                if let Some(2020) = asc_line.checked_add(*desc_line) {
                    // safe multiplication,
                    let result = match asc_line.checked_mul(*desc_line) {
                        None => {
                            return Err(format!(
                                "Integer overflowed, numbers too large to multiply: {} {}.",
                                asc_line, desc_line
                            ));
                        }
                        Some(result) => result,
                    };

                    return Ok(SilverSolution {
                        numbers: [*asc_line, *desc_line],
                        result,
                    });
                }
            }
        }

        Err("No solution found!".to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoldSolution {
    numbers: [usize; 3],
    result: usize,
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
                    if asc_line_1 > desc_line || asc_line_2 > desc_line {
                        break;
                    }

                    // safe addition, possibly overkill but interesting to try
                    let sum = match asc_line_1.checked_add(*asc_line_2) {
                        None => {
                            return Err(format!(
                                "Integer overflowed, numbers too large to sum: {} {}.",
                                asc_line_1, asc_line_2
                            ));
                        }
                        Some(factor) => match factor.checked_add(*desc_line) {
                            None => {
                                return Err(format!(
                                    "Integer overflowed, numbers too large to sum: {} {} {}.",
                                    asc_line_1, asc_line_2, desc_line
                                ));
                            }
                            Some(result) => result,
                        },
                    };

                    if 2020 == sum {
                        // safe multiplication, there must be a less verbose way to do this
                        let result = match asc_line_1.checked_mul(*asc_line_2) {
                            None => {
                                return Err(format!(
                                    "Integer overflowed, numbers too large to multiply: {} {}.",
                                    asc_line_1, asc_line_2
                                ));
                            }
                            Some(factor) => match factor.checked_mul(*desc_line) {
                                None => {
                                    return Err(format!(
                                        "Integer overflowed, numbers too large to multiply: {} {} {}.",
                                        asc_line_1, asc_line_2, desc_line
                                    ));
                                }
                                Some(result) => result,
                            },
                        };

                        return Ok(GoldSolution {
                            numbers: [*asc_line_1, *asc_line_2, *desc_line],
                            result,
                        });
                    }
                }
            }
        }

        Err("No solution found!".to_string())
    }
}
