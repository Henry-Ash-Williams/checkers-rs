use clearscreen::clear;
use regex::Regex;
use std::io::stdin;

use anyhow::{anyhow, Result, Error, Context};

use crate::{board::*, player::*, tile::*};

pub struct Game {
    board: Board,
    move_id: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            move_id: 0,
        }
    }

    pub fn run(&mut self) -> Result<Player> {
        loop {
            match (self.board.get_remaining_peices(Player::Black), self.board.get_remaining_peices(Player::White)) {
                (0, _) => { return Ok(Player::Black); },
                (_, 0) => { return Ok(Player::White); },
                (_, _) => (),
            };
            
            clear()?;
            println!("{}", self.board);
            self.get_stats();
            let (from, to) = loop {
                let this_move = self.get_move();
                if this_move.is_ok() {
                    break this_move.unwrap();
                } else {
                    println!("{this_move:?}");
                }
            };
            self.board.make_move(self.move_id, from, to)?;
            self.move_id += 1;
        }
    }

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn get_stats(&self) {
        println!("Move No #{}", self.move_id + 1);
        println!("Remaining Peices:");
        println!("Black\tWhite");
        println!("{}\t{}", self.board.get_remaining_peices(Player::Black), self.board.get_remaining_peices(Player::White));
    }

    pub fn get_idx(x: char, y: usize) -> Result<usize> {
        let x_range: Vec<char> = (97u8..97u8 + BOARD_SIZE as u8).map(|n| n as char).collect();
        let y_range: Vec<usize> = (1..BOARD_SIZE + 1).collect();

        if !x_range.contains(&x) || !y_range.contains(&y) {
            return Err(anyhow!("Invalid location selected"));
        }

        Ok(Board::coords_to_idx(x as usize - 97, y - 1))
    }

    pub fn get_move(&self) -> Result<(usize, usize)> {
        let stdin = stdin();
        let mut from_buf = String::new();
        let mut to_buf = String::new();
        let expected_pattern = Regex::new(r"(?m)\([a-hA-H],\s?[1-8]\)")?;
        println!(
            "Moving player: {}",
            if self.move_id % 2 == 0 {
                "Black"
            } else {
                "White"
            }
        );
        let from = loop {
            from_buf.clear();
            println!("Enter the source tile for your move: (x, y)");
            stdin.read_line(&mut from_buf)?;

            if expected_pattern.is_match(&from_buf) {
                let from_parts: Vec<&str> = from_buf[1..from_buf.len() - 2].split(',').collect();
                let sx: char = match from_parts.first().unwrap().parse() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let sy: usize = match from_parts.get(1).unwrap().parse() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let from = Self::get_idx(sx, sy)?;
                break from;
            }
        };
        let to = loop {
            to_buf.clear();
            println!("Enter the destination tile for your move: (x, y)");
            stdin.read_line(&mut to_buf)?;

            if expected_pattern.is_match(&to_buf) {
                let to_parts: Vec<&str> = to_buf[1..from_buf.len() - 2].split(',').collect();
                let tx: char = match to_parts.first().unwrap().parse() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let ty: usize = match to_parts.get(1).unwrap().parse() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let to = Self::get_idx(tx, ty)?;
                break to;
            }
        };

        if !self.is_valid_move(from, to) {
            Err(anyhow!("Invalid move selected"))
        } else {
            Ok((from, to))
        }
    }

    #[allow(clippy::clone_on_copy)]
    // Lint disabled here as we need to make a deep copy to prevent the game 
    // from actually making the move
    fn is_valid_move(&self, from: usize, to: usize) -> bool {
        let mut board_cpy = self.board.clone();
        board_cpy.make_move(self.move_id, from, to).is_ok()
    }
}
