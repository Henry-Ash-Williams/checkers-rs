use clearscreen::clear;
use dialoguer::Select;
use regex::Regex;
use std::io::stdin;

use anyhow::{anyhow, Context, Error, Result};

use crate::{board::*, player::{*, self}, tile::*};

pub enum GameMode {
    HumanVsHuman,
    HumanVsAi,
    AiVsAi,
}

impl GameMode {
    pub fn select_gamemode() -> Result<Self> {
        let options = vec!["Human vs Human", "Human vs AI", "AI vs AI"];
        let selection = Select::new()
            .with_prompt("Select a gamemode (use the arrow keys to make your selection)")
            .items(&options)
            .interact()?;

        Ok(match selection {
            0 => Self::HumanVsHuman,
            1 => Self::HumanVsAi,
            2 => Self::AiVsAi,
            _ => unreachable!(),
        })
    }
}

pub struct Game {
    board: Board,
    move_id: usize,
    mode: GameMode,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            move_id: 0,
            mode: GameMode::select_gamemode().unwrap_or(GameMode::HumanVsHuman),
        }
    }

    /// This function looks at the current board and determines the best possible move it can make in that given state.
    /// In order to figure this out, it uses the negamax algorithm, which is a variant of the minimax algorithm.  
    pub fn get_best_move(&self) -> (usize, usize) {
        todo!()
    }

    /* fn evaluate_board(&self, moving_player: Player) -> usize {
        let friendly_peices = self.board.get_idx_of_player_peices(moving_player);
        let enemy_peices = self.board.get_idx_of_player_peices(!moving_player);
        let distance_to_promotion = todo!();
        // Get number of peices where a capture is possible
        let potential_captures = friendly_peices.iter().filter(|idx| self.board.can_capture(moving_player, **idx)).count();
        let vulnerable_peices = enemy_peices.iter().filter(|idx| self.board.can_capture(!moving_player, **idx)).count();

        (distance_to_promotion + potential_captures) - vulnerable_peices
    } */

    pub fn run(&mut self) -> Result<Player> {
        loop {
            match (
                self.board.get_remaining_peices(Player::Black),
                self.board.get_remaining_peices(Player::White),
            ) {
                (0, _) => {
                    return Ok(Player::Black);
                }
                (_, 0) => {
                    return Ok(Player::White);
                }
                (_, _) => (),
            };

            match self.mode { 
                GameMode::HumanVsHuman => clear()?,
                _ => (), 
            };

            println!("{}", self.board);
            self.get_stats();
            let (from, to) = loop {
                let this_move = match self.mode {
                    GameMode::HumanVsHuman => self.get_user_move(),
                    GameMode::HumanVsAi if self.move_id % 2 == 0 => self.get_user_move(),
                    GameMode::HumanVsAi => Ok(self.get_best_move()),
                    GameMode::AiVsAi => Ok(self.get_best_move()),
                };
                if let Ok(m) = this_move {
                    break m;
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
        println!(
            "{}\t{}",
            self.board.get_remaining_peices(Player::Black),
            self.board.get_remaining_peices(Player::White)
        );
    }

    pub fn get_idx(x: char, y: usize) -> Result<usize> {
        let x_range: Vec<char> = (97u8..97u8 + BOARD_SIZE as u8).map(|n| n as char).collect();
        let y_range: Vec<usize> = (1..BOARD_SIZE + 1).collect();

        if !x_range.contains(&x) || !y_range.contains(&y) {
            return Err(anyhow!("Invalid location selected"));
        }

        Ok(Board::coords_to_idx(x as usize - 97, y - 1))
    }

    pub fn get_user_move(&self) -> Result<(usize, usize)> {
        println!(
            "Moving player: {}",
            if self.move_id % 2 == 0 {
                "Black"
            } else {
                "White"
            }
        );
        loop {
            let from = Self::get_location("Enter the source tile for your move: xy")?;
            let to = Self::get_location("Enter the destination tile for your move: xy")?;

            if self.is_valid_move(from, to) {
                break Ok((from, to));
            } else {
                println!("Invalid move selected, try again");
            }
        }
    }

    fn get_location(prompt: &str) -> Result<usize> {
        let expected_pattern = Regex::new(r"(?m)[A-Ha-h][1-8]")?;
        let mut buf = String::new();
        let stdin = stdin();
        loop {
            buf.clear();
            println!("{}", prompt);
            stdin.read_line(&mut buf)?;

            if expected_pattern.is_match(&buf) {
                let x: char = buf.chars().nth(0).unwrap();
                let y: usize = buf.chars().nth(1).unwrap() as usize - 48;
                let pos = Self::get_idx(x, y)?;
                break Ok(pos);
            }
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
