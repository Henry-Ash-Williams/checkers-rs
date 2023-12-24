#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::{anyhow, Result};

use std::mem::size_of;

use clearscreen::clear;

use crate::{
    board::{Board, BOARD_SIZE},
    game::{Game, GameMode},
    player::Player,
    r#move::{Move, Position},
    tile::{Tile, TileKind},
};

mod board;
mod game;
mod king_moves;
mod r#move;
mod player;
mod tile;

fn main() -> Result<()> {
    clear()?;
    let mut game = Game::new();
    let winner = game.run()?;
    println!("WINNER: {:?}", winner);
    Ok(())
}

//  macro_rules! tile {
//     ($owner:expr) => {
//         Tile {
//             occupied_by: Some($owner),
//             kind: TileKind::Normal,
//         }
//     };
//     () => {
//         Tile {
//             occupied_by: None,
//             kind: TileKind::Normal,
//         }
//     };
// }

// macro_rules! king {
//     ($owner:expr) => {
//         Tile {
//             occupied_by: Some($owner),
//             kind: TileKind::King,
//         }
//     };
// }

// fn main() {
//     let mut board = Board::new();
//     // remove all peices from the board
//     board.board_mut().iter_mut().for_each(|tile| tile.leave());

//     assert_eq!(board.get_remaining_peices(Player::White), 0);
//     assert_eq!(board.get_remaining_peices(Player::Black), 0);

//     board.board_mut()[42].take_ownership(Player::Black);
//     board.board_mut()[42].promote();
//     board.board_mut()[28].take_ownership(Player::White);

//     println!("{board}");
//     println!("{}", );
//     assert_eq!(board.board_mut()[42].kind(), TileKind::King);
//     assert_eq!(board.board_mut()[28], tile!(Player::White));

//     assert!(board.make_move(0, Move::new(42, 14)).is_ok());
//     assert!(board.make_move(0, Move::new(42, 42)).is_err());

//     assert_eq!(board.board_mut()[14], king!(Player::Black));
//     assert_eq!(board.board_mut()[28], tile!());

//     board.board_mut()[14].leave();
//     board.board_mut()[35] = king!(Player::Black);
//     board.board_mut()[17].take_ownership(Player::White);

//     assert!(board.make_move(0, Move::new(35, 17)).is_ok());
//     assert_eq!(board.board_mut()[17], king!(Player::Black));

//     board.board_mut()[44].take_ownership(Player::White);
//     board.board_mut()[53].take_ownership(Player::White);

//     assert!(board.make_move(0, Move::new(17, 62)).is_ok());
// }
