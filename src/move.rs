use anyhow::{anyhow, Result};
use regex::Regex;

use crate::board::*;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn from_coords_checked(x: isize, y: isize) -> Result<Self> {
        if x < 0 || y < 0 || x > BOARD_SIZE as isize || y > BOARD_SIZE as isize {
            return Err(anyhow!("Index error"));
        }
        let idx = Board::coords_to_idx(x as usize, y as usize);

        Self::from_idx_checked(idx)
    }

    #[allow(clippy::unwrap_used)]
    pub fn from_str<S: Into<String>>(buffer: S) -> Result<Self> {
        let buffer: String = buffer.into();
        let expected_pattern = Regex::new(r"(?m)[A-Ha-h][1-8]")?;

        if expected_pattern.is_match(&buffer) {
            // SAFETY: guarentted not to panic as the regex to enter this path
            // requires length == 2
            let x: char = buffer.chars().nth(0).unwrap();
            let y: usize = buffer.chars().nth(1).unwrap() as usize - 48;
            let x_range: Vec<char> = (97u8..97u8 + BOARD_SIZE as u8).map(|n| n as char).collect();
            let y_range: Vec<usize> = (1..BOARD_SIZE + 1).collect();

            if !x_range.contains(&x) || !y_range.contains(&y) {
                return Err(anyhow!("invalid location selected"));
            }

            let pos = Position::from_idx(Board::coords_to_idx(x as usize - 97, y - 1));
            Ok(pos)
        } else {
            Err(anyhow!("Could not parse the string"))
        }
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

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = Board::idx_to_coords(self.0);

        let x = (x as u8 + 97) as char;
        let y = y + 1;

        write!(f, "Position {{ {x}{y} }}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn from_positions(from: Position, to: Position) -> Self {
        Self { from, to }
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
