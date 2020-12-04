use crate::{GoldChallenge, SilverChallenge};

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Open,
    Tree,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TobogganMap {
    pub elevations: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapSize {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapLocation {
    pub right: usize,
    pub down: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Day03 {
    map: TobogganMap,
    size: MapSize,
    location: MapLocation,
}

impl Day03 {
    pub fn new(data: String) -> Result<Self, String> {
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
}

#[cfg(test)]
const SAMPLE_MAP: &'static str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

#[test]
fn sample_map_construction() {
    let challenge = Day03::new(SAMPLE_MAP.to_string());
    assert_ne!(challenge.ok(), None)
}

#[test]
fn sample_map_navigation() {
    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();

    assert_eq!(challenge.tile(), Some(&Tile::Open));
    assert_eq!(challenge.navigate(3, 1).tile(), Some(&Tile::Open));
    assert_eq!(challenge.navigate(1, 0).tile(), Some(&Tile::Tree));
    assert_eq!(challenge.navigate(2, 1).tile(), Some(&Tile::Tree));
    assert_eq!(challenge.location(), &MapLocation { right: 6, down: 2 });
}

#[test]
fn sample_map_navigation_past_edge_of_map() {
    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();
    assert_eq!(challenge.navigate(13, 0).tile(), Some(&Tile::Tree));

    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();
    assert_eq!(challenge.navigate(14, 0).tile(), Some(&Tile::Tree));

    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();
    assert_eq!(challenge.navigate(15, 0).tile(), Some(&Tile::Open));
}

#[test]
fn sample_map_navigation_past_bottom_of_map() {
    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();
    assert_eq!(challenge.navigate(0, 11).tile(), None);
}

#[test]
fn sample_map_location() {
    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();

    assert_eq!(
        challenge
            .navigate(3, 1)
            .navigate(1, 0)
            .navigate(2, 1)
            .location(),
        &MapLocation { right: 6, down: 2 }
    );
}

impl SilverChallenge for Day03 {
    type Answer = usize;

    fn attempt_silver(&mut self) -> Result<Self::Answer, String>
    where
        Self::Answer: std::fmt::Debug,
    {
        let mut counter = if self.tile() == Some(&Tile::Tree) {
            1
        } else {
            0
        };

        while let Some(tile) = self.navigate(3, 1).tile() {
            if tile == &Tile::Tree {
                counter += 1;
            }
        }

        Ok(counter)
    }
}

#[test]
fn sample_map_silver_solution() {
    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();

    assert_eq!(challenge.attempt_silver(), Ok(7));
}

impl GoldChallenge for Day03 {
    type Answer = usize;

    fn attempt_gold(&mut self) -> Result<Self::Answer, String>
    where
        Self::Answer: std::fmt::Debug,
    {
        let base_counter = if self.tile() == Some(&Tile::Tree) {
            1
        } else {
            0
        };

        let mut counters: [usize; 5] = [
            base_counter,
            base_counter,
            base_counter,
            base_counter,
            base_counter,
        ];

        // since both challenges use the same data,
        // resetting immediately is necessary although it might be
        // worth changing the runner to instantiate each separately
        self.reset();
        while let Some(tile) = self.navigate(1, 1).tile() {
            if tile == &Tile::Tree {
                counters[0] += 1;
            }
        }
        self.reset();
        while let Some(tile) = self.navigate(3, 1).tile() {
            if tile == &Tile::Tree {
                counters[1] += 1;
            }
        }
        self.reset();
        while let Some(tile) = self.navigate(5, 1).tile() {
            if tile == &Tile::Tree {
                counters[2] += 1;
            }
        }
        self.reset();
        while let Some(tile) = self.navigate(7, 1).tile() {
            if tile == &Tile::Tree {
                counters[3] += 1;
            }
        }
        self.reset();
        while let Some(tile) = self.navigate(1, 2).tile() {
            if tile == &Tile::Tree {
                counters[4] += 1;
            }
        }

        let solution = counters.iter().fold(1, |acc, x| acc * x);
        Ok(solution)
    }
}

#[test]
fn sample_map_gold_solution() {
    let mut challenge = Day03::new(SAMPLE_MAP.to_string()).unwrap();

    assert_eq!(challenge.attempt_gold(), Ok(336));
}
