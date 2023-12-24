use std::fmt::{self, Display};
use std::ops::Not;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Player {
    Black,
    White,
}

impl Not for Player {
    type Output = Player;

    fn not(self) -> Self::Output {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Player::Black => "Black",
                Player::White => "White",
            }
        )
    }
}
