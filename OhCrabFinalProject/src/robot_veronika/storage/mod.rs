use std::fmt;
use robotics_lib::world::tile::Content;

#[derive(Debug, Clone)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Creates a new instance of `Coordinate`, called only inside of the common crate
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    /// Returns the row of the coordinate
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// Returns the column of the coordinate
    pub fn get_col(&self) -> usize {
        self.col
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut  fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Eq for Position {}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

#[derive(Debug, Clone)]
pub struct StorageInfo{
    position: Position,
    // Content is potentially not needed
    content: Content,
    quantity: u32,
    coefficient: u32
}

impl Eq for StorageInfo {}

impl PartialEq for StorageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}