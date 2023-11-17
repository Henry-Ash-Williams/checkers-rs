#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};

use std::mem::size_of;

use board::{Board, BOARD_SIZE};
use clearscreen::clear;
use game::Game;

use crate::{
    player::Player,
    tile::{Tile, TileKind},
};

mod board;
mod game;
mod player;
mod tile;

fn main() -> Result<()> {
    clear()?;
    let mut game = Game::new();
    let winner = game.run()?;
    println!("WINNER: {:?}", winner);
    Ok(())
}
