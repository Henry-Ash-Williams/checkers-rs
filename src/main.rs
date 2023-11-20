#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};

use std::mem::size_of;

use clearscreen::clear;

use crate::{
    board::{Board, BOARD_SIZE},
    game::{Game, GameMode},
    player::Player,
    r#move::Position,
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
    let mut game = Game::new();
    let winner = game.run()?;
    println!("WINNER: {:?}", winner);
    Ok(())
} */

fn main() -> Result<()> {
    for i in 0..64 {
        let mut board = Board::new();
        // remove all peices from the board
        board.board_mut().iter_mut().for_each(|tile| tile.leave());

        board.board_mut()[i].take_ownership(Player::Black);
        board.board_mut()[i].promote();
        board.board_mut()[28].take_ownership(Player::White);
        let mut game = Game::from_board(board.board(), GameMode::HumanVsHuman);
        let moves = game.generate_moves_for_peice(Position::from_idx(i))?;
        for king_move in moves.iter() {
            game.board_mut()[king_move.to()].take_ownership(Player::Black);
        }
        println!("{}", game.board());
    }

    Ok(())
}
