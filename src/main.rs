#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};

use board::{Board, BOARD_SIZE};
use game::Game;

use crate::{player::Player, tile::{Tile, TileKind}};

mod board;
mod game;
mod player;
mod tile;

fn stats(b: Board) {
    println!(
        "Remaining peices: \nBlack\tWhite\n{}\t{}",
        b.get_remaining_peices(Player::Black),
        b.get_remaining_peices(Player::White)
    );
}

fn main() -> Result<()> {
    let mut game = Game::new(); 

    let winner = game.run()?;
    println!("WINNER: {:?}", winner);
    Ok(()) 

}
