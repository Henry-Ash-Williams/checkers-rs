use crate::player::Player;
use anyhow::{anyhow, Result};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileKind {
    Normal,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub occupied_by: Option<Player>,
    pub kind: TileKind,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.occupied_by {
                None => " ",
                Some(t) => {
                    match (t, self.kind()) {
                        (Player::Black, TileKind::Normal) => "",
                        (Player::White, TileKind::Normal) => "",
                        (Player::Black, TileKind::King) => "󱟜",
                        (Player::White, TileKind::King) => "",
                    }
                }
            }
        )
    }
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            occupied_by: None,
            kind: TileKind::Normal,
        }
    }

    pub fn occupied(player: Player) -> Self {
        Self {
            occupied_by: Some(player),
            kind: TileKind::Normal,
        }
    }

    pub fn take_ownership(&mut self, player: Player) {
        self.occupied_by = Some(player)
    }

    pub fn promote(&mut self) {
        self.kind = TileKind::King;
    }

    pub fn get_owner(&self) -> Result<Player> {
        match self.occupied_by {
            Some(p) => Ok(p),
            None => Err(anyhow!("This piece is not occupied by anyone!")),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.occupied_by.is_none()
    }

    pub fn leave(&mut self) {
        self.occupied_by = None;
    }

    pub fn kind(&self) -> TileKind {
        self.kind
    }

    pub fn demote(&mut self) {
        self.kind = TileKind::Normal;
    }
}
