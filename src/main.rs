#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::{anyhow, Result};

use std::mem::size_of;

use clearscreen::clear;

use crate::{
    board::{Board, BOARD_SIZE},
    game::{Game, GameMode},
    player::Player,
    r#move::{Position, Move},
    tile::{Tile, TileKind},
};

mod board;
mod game;
mod king_moves;
mod r#move;
mod player;
mod tile;

/* fn main() -> Result<()> {
    clear()?;
    println!("{}", board::FORCE_CAPTURE);
    let mut game = Game::new();
    let winner = game.run()?;
    println!("WINNER: {:?}", winner);
    Ok(())
} */

fn main() {
    let mut board = Board::new();
    // remove all peices from the board
    board.board_mut().iter_mut().for_each(|tile| tile.leave());

    assert_eq!(board.get_remaining_peices(Player::White), 0);
    assert_eq!(board.get_remaining_peices(Player::Black), 0);

    board[Position::from_idx(42)].take_ownership(Player::Black);
    board[Position::from_idx(42)].promote();
    board[Position::from_idx(28)].take_ownership(Player::White);

    assert_eq!(board[Position::from_idx(42)].kind(), TileKind::King);
    assert_eq!(
        board[Position::from_idx(28)],
        Tile {
            occupied_by: Some(Player::White),
            kind: TileKind::Normal
        }
    );

    assert!(board.make_move(0, Move::new(42, 14)).is_ok());
    assert!(board.make_move(0, Move::new(42, 42)).is_err());

    assert_eq!(
        board[Position::from_idx(14)],
        Tile {
            occupied_by: Some(Player::Black),
            kind: TileKind::King
        }
    );
    assert_eq!(
        board[Position::from_idx(28)],
        Tile {
            occupied_by: None,
            kind: TileKind::Normal
        }
    );

    board[Position::from_idx(14)].leave();
    board[Position::from_idx(35)] = Tile {
        occupied_by: Some(Player::Black),
        kind: TileKind::King,
    };
    board[Position::from_idx(17)].take_ownership(Player::White);

    assert!(board.make_move(0, Move::new(35, 17)).is_ok());
    assert_eq!(
        board[Position::from_idx(17)],
        Tile {
            occupied_by: Some(Player::Black),
            kind: TileKind::King
        }
    );

    board[Position::from_idx(44)].take_ownership(Player::White);
    board[Position::from_idx(53)].take_ownership(Player::White);

    assert!(board.make_move(0, Move::new(17, 62)).is_ok());
}
