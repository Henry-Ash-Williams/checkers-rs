#![allow(clippy::needless_range_loop, unused_variables)]

use std::cmp::{max, min};
use std::{fmt, ops::Range};

use anyhow::{anyhow, Result};

use tabled::{
    builder::Builder,
    settings::{style::Style, themes::Colorization, Color},
};

use crate::player::Player;
use crate::tile::{Tile, TileKind};

pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    board: [Tile; BOARD_SIZE * BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        let mut board = [Tile::empty(); BOARD_SIZE * BOARD_SIZE];

        for y in 0..3 {
            for x in 0..BOARD_SIZE {
                if (x + (y % 2)) % 2 == 0 {
                    board[Board::coords_to_idx(x, y)].take_ownership(Player::Black);
                }
            }
        }

        for y in (BOARD_SIZE - 3)..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if (x + (y % 2)) % 2 == 0 {
                    board[Board::coords_to_idx(x, y)].take_ownership(Player::White);
                }
            }
        }

        Self { board }
    }

    pub fn board(&self) -> &[Tile] {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut [Tile] {
        &mut self.board
    }

    /// Convert a pair of coordinates, `x`, and `y` to an index in the board array
    pub fn coords_to_idx(x: usize, y: usize) -> usize {
        (y * BOARD_SIZE) + x
    }

    /// Convert an index in the `boards` array to a pair of x, and y coordinates
    pub fn idx_to_coords(idx: usize) -> (usize, usize) {
        (idx % BOARD_SIZE, idx / BOARD_SIZE)
    }

    /// Calculate the manhattan distance between two tiles on the board
    fn delta(from: usize, to: usize) -> (isize, isize) {
        let from = Self::idx_to_coords(from);
        let to = Self::idx_to_coords(to);
        (
            to.0 as isize - from.0 as isize,
            to.1 as isize - from.1 as isize,
        )
    }

    /// Get the number of remaining peices for a given player
    pub fn get_remaining_peices(&self, player: Player) -> usize {
        self.into_iter()
            .clone()
            .filter(|tile| tile.occupied_by == Some(player))
            .count()
    }

    /// Handle moving the king
    pub fn handle_king_movement(
        &mut self,
        moving_player: Player,
        from: usize,
        to: usize,
    ) -> Result<()> {
        if from == to {
            return Err(anyhow!(
                "Cannot move to the same position as where the peice started"
            ));
        }

        let delta = Self::delta(from, to);

        // Verify peice is moving diagonally
        let (dx, dy) = delta;

        if dx.abs() != dy.abs() {
            return Err(anyhow!("Peice must move diagonally"));
        }

        // Make sure the peice is able to move, i.e. is not blocked by any
        // friendly peices, and capture any enemy peices in its way

        let from_coords = Self::idx_to_coords(from);
        let to_coords = Self::idx_to_coords(to);

        let (sx, sy) = from_coords;
        let (tx, ty) = to_coords;

        // Generate a list of indicies the king must move over to get to the target
        let moves: Vec<usize> = (min(sx, tx)..max(sx, tx))
            .zip(min(sy, ty)..max(sy, ty))
            .map(|(x, y)| Self::coords_to_idx(x, y))
            .collect();

        // Check the move is not blocked by any friendly peices, make sure to ignore the moving tile
        let move_contains_friendly_peices = moves
            .iter()
            .any(|idx| self.board[*idx].occupied_by == Some(moving_player) && *idx != from);

        if move_contains_friendly_peices {
            return Err(anyhow!("Move blocked by friendly peice(s)"));
        }

        // Capture the peices in the way of the king
        let captured_peices: Vec<usize> = moves
            .iter()
            .filter(|idx| self.board[**idx].occupied_by == Some(!moving_player))
            .copied()
            .collect();

        for idx in captured_peices {
            self.board[idx].leave()
        }

        self.board[from].leave();
        self.board[from].demote();
        self.board[to].take_ownership(moving_player);
        self.board[to].promote();

        Ok(())
    }

    /// Check if a player has won
    pub fn has_player_won(&self, player: Player) -> bool {
        self.get_remaining_peices(!player) == 0
    }

    pub fn get_idx_of_player_peices(&self, player: Player) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.occupied_by == Some(!player) || tile.is_empty())
            .map(|(idx, _)| idx)
            .collect()
    }

    pub fn has_king(&self, player: Player) -> bool {
        self.board
            .iter()
            .any(|tile| tile.kind() == TileKind::King && tile.occupied_by == Some(player))
    }

    /// Make a move
    pub fn make_move(&mut self, turn_id: usize, from: usize, to: usize) -> Result<()> {
        let delta = Self::delta(from, to);

        // Check no-ones already won
        if self.has_player_won(Player::Black) {
            return Err(anyhow!("Black has already won!"));
        } else if self.has_player_won(Player::White) {
            return Err(anyhow!("White has already won!"));
        }

        if from == to {
            return Err(anyhow!("Cannot move to the same position"));
        }

        // check they're not trying to move a white piece
        let moving_player = if turn_id % 2 == 0 {
            Player::Black
        } else {
            Player::White
        };
        if !moving_player == self.board[from].get_owner()? {
            return Err(anyhow!("Cannot move the other players piece!"));
        }

        // Check that normal tiles only move +1 tile diagonally forward
        if let TileKind::Normal = self.board[from].kind() {
            let (dx, dy) = delta;

            // Check that the peice is moving diagonally
            if dx.abs() != dy.abs() {
                return Err(anyhow!(
                    "Normal peices cannot move more than one tile diagonally"
                ));
            }

            // Check that the peice is only moving one tile
            if dx.abs() > 1 {
                return Err(anyhow!("Normal peices can only move one tile diagonally"));
            }

            // Check that the peice is moving forwards
            if dy == if let Player::Black = moving_player { -1 } else { 1 } {
                return Err(anyhow!("Normal peices cannot move backwards"));
            }
        } else {
            return self.handle_king_movement(moving_player, from, to);
        }

        // Check if target tile is occupied
        if !self.board[to].is_empty() {
            if !moving_player == self.board[to].get_owner()? {
                // Handle taking an opponents peice by jumping over it
                let to_coords = Self::idx_to_coords(to);
                let next_tile = (
                    (to_coords.0 as isize + delta.0) as usize,
                    (to_coords.1 as isize + delta.1) as usize,
                );

                if self.board[Self::coords_to_idx(next_tile.0, next_tile.1)].is_empty() {
                    self.board[from].leave();
                    self.board[to].leave();
                    self.board[Self::coords_to_idx(next_tile.0, next_tile.1)]
                        .take_ownership(moving_player);
                    Ok(())
                } else {
                    Err(anyhow!("Next tile is already occupied!"))
                }
            } else {
                Err(anyhow!("You already occupy this tile!"))
            }
        } else {
            // Simple move without capturing an enemy peice
            self.board[from].leave();
            self.board[to].take_ownership(moving_player);

            // Check if the peice needs to be promoted
            if Self::idx_to_coords(to).1
                == if let Player::Black = moving_player {
                    BOARD_SIZE - 1
                } else {
                    0
                }
            {
                self.board[to].promote();
            }
            Ok(())
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_white = Color::BG_WHITE | Color::FG_BLACK;
        let color_black = Color::FG_WHITE | Color::BG_BLACK;

        let mut table = Builder::default();
        for y in 0..BOARD_SIZE {
            let mut row = vec![" ".to_string(); BOARD_SIZE];
            for x in 0..BOARD_SIZE {
                row[x] = format!("{}", self.board[Board::coords_to_idx(x, y)]);
            }
            table.push_record(row);
        }

        let mut table = table.build();
        table
            .with(Style::empty())
            .with(Colorization::chess(color_white, color_black));
        let table_str = format!("{}", table);
        let table_str: Vec<&str> = table_str.split('\n').collect();
        write!(f, "  ")?;
        for i in 0..BOARD_SIZE {
            write!(f, " {} ", (97 + i as u8) as char)?;
        }
        writeln!(f)?;

        for (idx, row) in table_str.iter().enumerate() {
            writeln!(f, "{} {row}", idx + 1)?;
        }
        write!(f, "")
    }
}

impl IntoIterator for Board {
    type Item = Tile;
    type IntoIter = std::array::IntoIter<Self::Item, { BOARD_SIZE * BOARD_SIZE }>;

    fn into_iter(self) -> Self::IntoIter {
        self.board.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        board::{Tile, TileKind, BOARD_SIZE},
        player::Player,
    };

    use super::Board;

    #[test]
    fn test_board_initializer() {
        let board = Board::new();

        assert_eq!(board.board.len(), BOARD_SIZE * BOARD_SIZE);
        assert_eq!(
            board.board,
            [
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::Black),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: None,
                    kind: TileKind::Normal
                },
                Tile {
                    occupied_by: Some(Player::White),
                    kind: TileKind::Normal
                },
            ]
        );
        // Each player has 12 tiles
        assert_eq!(board.get_remaining_peices(Player::White), 12);
        assert_eq!(board.get_remaining_peices(Player::Black), 12);

        // No kings on the board
        assert_eq!(
            board
                .into_iter()
                .filter(|tile| tile.kind == TileKind::King)
                .count(),
            0
        );
    }

    #[test]
    fn test_movement() {
        let mut board = Board::new();

        // Check we cannot move to the same location as we started
        assert!(board.make_move(0, 20, 20).is_err());

        // Move e3 to f4
        assert!(board.make_move(0, 20, 29).is_ok());

        // Verify that board updates accordingly
        assert_eq!(
            board.board()[29],
            Tile {
                occupied_by: Some(Player::Black),
                kind: TileKind::Normal
            }
        );
        assert_eq!(
            board.board()[20],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            }
        );

        assert!(board.make_move(1, 43, 36).is_ok());

        // Verify that board updates accordingly
        assert_eq!(
            board.board()[36],
            Tile {
                occupied_by: Some(Player::White),
                kind: TileKind::Normal
            },
        );
        assert_eq!(
            board.board()[43],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            },
        );

        assert_eq!(
            board.board()[36],
            Tile {
                occupied_by: Some(Player::White),
                kind: TileKind::Normal
            }
        );

        assert!(board.make_move(0, 29, 36).is_ok());

        // Verify that capturing works as expected
        assert_eq!(
            board.board()[29],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            }
        );
        assert_eq!(
            board.board()[36],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            }
        );

        assert_eq!(
            board.board()[43],
            Tile {
                occupied_by: Some(Player::Black),
                kind: TileKind::Normal
            }
        );

        assert!(board.make_move(1, 52, 43).is_ok());

        assert_eq!(
            board.board()[43],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            }
        );

        assert_eq!(
            board.board()[52],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            }
        );

        assert_eq!(
            board.board()[34],
            Tile {
                occupied_by: Some(Player::White),
                kind: TileKind::Normal
            }
        );

        assert!(board.make_move(0, 18, 10).is_err());
    }

    #[test]
    fn test_king_promotion() {
        let mut board = Board::new();

        // Remove all white tiles from the board to make my life easier
        board
            .board
            .iter_mut()
            .filter(|tile| tile.is_empty() || tile.occupied_by == Some(Player::White))
            .for_each(|black_tile| black_tile.leave());

        // Add one white peice back so the game doesn't think black's won
        board.board[63].take_ownership(Player::White);

        assert!(board.make_move(0, 22, 29).is_ok());
        assert!(board.make_move(0, 29, 36).is_ok());
        assert!(board.make_move(0, 36, 43).is_ok());
        assert!(board.make_move(0, 43, 50).is_ok());
        assert!(board.make_move(0, 50, 59).is_ok());
        assert!(board.board[59].kind() == TileKind::King);
    }

    #[test]
    fn test_king_movement() {
        let mut board = Board::new();
        // remove all peices from the board
        board.board.iter_mut().for_each(|tile| tile.leave());

        assert_eq!(board.get_remaining_peices(Player::White), 0);
        assert_eq!(board.get_remaining_peices(Player::Black), 0);

        board.board[42].take_ownership(Player::Black);
        board.board[42].promote();
        board.board[28].take_ownership(Player::White);

        assert_eq!(board.board[42].kind(), TileKind::King);
        assert_eq!(
            board.board[28],
            Tile {
                occupied_by: Some(Player::White),
                kind: TileKind::Normal
            }
        );

        assert!(board.make_move(0, 42, 14).is_ok());
        assert!(board.make_move(0, 42, 42).is_err());

        assert_eq!(
            board.board[14],
            Tile {
                occupied_by: Some(Player::Black),
                kind: TileKind::King
            }
        );
        assert_eq!(
            board.board[28],
            Tile {
                occupied_by: None,
                kind: TileKind::Normal
            }
        );

        board.board[14].leave();
        board.board[35] = Tile {
            occupied_by: Some(Player::Black),
            kind: TileKind::King,
        };
        board.board[17].take_ownership(Player::White);

        assert!(board.make_move(0, 35, 17).is_ok());
        assert_eq!(
            board.board[17],
            Tile {
                occupied_by: Some(Player::Black),
                kind: TileKind::King
            }
        );

        board.board[44].take_ownership(Player::White);
        board.board[53].take_ownership(Player::White);

        assert!(board.make_move(0, 17, 62).is_ok());

    }
}
