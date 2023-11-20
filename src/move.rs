use anyhow::{anyhow, Result};

use crate::board::*;

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(usize);

impl Position {
    pub fn from_idx(idx: usize) -> Self {
        if idx > (BOARD_SIZE * BOARD_SIZE) - 1 {
            panic!("Position {} outside of board limits", Self(idx));
        }

        Self(idx)
    }

    pub fn from_idx_checked(idx: usize) -> Result<Self> {
        if idx > BOARD_SIZE * BOARD_SIZE - 1 {
            Err(anyhow!("Position {} outside of board limits", Self(idx)))
        } else {
            Ok(Self(idx))
        }
    }

    pub fn from_coords(x: usize, y: usize) -> Self {
        let idx = Board::coords_to_idx(x, y);

        Self::from_idx(idx)
    }

    pub fn from_coords_checked(x: usize, y: usize) -> Result<Self> {
        let idx = Board::coords_to_idx(x, y);

        Self::from_idx_checked(idx)
    }

    pub fn idx(&self) -> usize {
        self.0
    }

    pub fn coords(&self) -> (usize, usize) {
        Board::idx_to_coords(self.0)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = Board::idx_to_coords(self.0);

        let x = (x as u8 + 97) as char;
        let y = y + 1;

        write!(f, "{x}{y}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    from: Position,
    to: Position,
}

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from: Position(from),
            to: Position(to),
        }
    }

    /// Calculate the manhattan distance between two tiles on the board
    pub fn delta(&self) -> (isize, isize) {
        let from = self.from.coords();
        let to = self.to.coords();
        (
            to.0 as isize - from.0 as isize,
            to.1 as isize - from.1 as isize,
        )
    }

    pub fn from(&self) -> Position {
        self.from
    }

    pub fn to(&self) -> Position {
        self.to
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
