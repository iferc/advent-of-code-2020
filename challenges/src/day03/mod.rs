#[cfg(test)]
mod tests;
mod toboganning;

use crate::{GoldChallenge, SilverChallenge};
use toboganning::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Day03 {
    map: TobogganMap,
    size: MapSize,
    location: MapLocation,
}

impl Day03 {
    pub fn new(data: &str) -> Result<Self, String> {
        let lines: Vec<_> = data.lines().collect();

        let height = lines.len();
        if height == 0 {
            return Err("No map data given.".to_string());
        }
        let width = lines[0].len();
        if width == 0 {
            return Err("No map data given.".to_string());
        }

        let mut elevations = Vec::with_capacity(height);
        for line in lines {
            if line.len() != width {
                return Err("Inconsistent horizontal map lengths.".to_string());
            }

            let mut row = Vec::with_capacity(width);
            for tile in line.chars() {
                row.push(match tile {
                    '.' => Tile::Open,
                    '#' => Tile::Tree,
                    symbol => return Err(format!("Unrecognized tile: {}.", symbol)),
                });
            }

            elevations.push(row);
        }

        Ok(Self {
            map: TobogganMap { elevations },
            size: MapSize { height, width },
            location: MapLocation { right: 0, down: 0 },
        })
    }

    pub fn reset(&mut self) -> &mut Self {
        self.location.right = 0;
        self.location.down = 0;

        self
    }

    pub fn navigate(&mut self, right: usize, down: usize) -> &mut Self {
        self.location.right += right;
        self.location.down += down;

        self
    }

    pub fn location(&self) -> &MapLocation {
        &self.location
    }

    pub fn tile(&self) -> Option<&Tile> {
        let periodic_right = self.location.right % self.size.width;

        if self.location.down >= self.size.height {
            return None;
        }

        Some(&self.map.elevations[self.location.down][periodic_right])
    }

    pub fn count_trees_for_slope(&mut self, right: usize, left: usize) -> usize {
        let mut counter = if self.tile() == Some(&Tile::Tree) {
            1
        } else {
            0
        };

        while let Some(tile) = self.navigate(right, left).tile() {
            if tile == &Tile::Tree {
                counter += 1;
            }
        }

        counter
    }
}

impl SilverChallenge for Day03 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_silver(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        Ok(self.reset().count_trees_for_slope(3, 1))
    }
}

impl GoldChallenge for Day03 {
    type Answer = usize;
    type Error = &'static str;

    fn attempt_gold(&mut self) -> Result<Self::Answer, Self::Error>
    where
        Self::Answer: std::fmt::Debug,
    {
        // since both challenges use the same data,
        // resetting immediately is necessary although it might be
        // worth changing the runner to instantiate each separately

        let counters: [usize; 5] = [
            self.reset().count_trees_for_slope(1, 1),
            self.reset().count_trees_for_slope(3, 1),
            self.reset().count_trees_for_slope(5, 1),
            self.reset().count_trees_for_slope(7, 1),
            self.reset().count_trees_for_slope(1, 2),
        ];

        let solution = counters.iter().fold(1, |acc, x| acc * x);

        Ok(solution)
    }
}
