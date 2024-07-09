#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;

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