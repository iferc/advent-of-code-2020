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
