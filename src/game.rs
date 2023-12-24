use clearscreen::clear;
use dialoguer::Select;
use std::{collections::HashMap, io::stdin, sync::PoisonError};

use anyhow::{anyhow, Context, Error, Result};

use crate::{
    board::{self, *},
    king_moves::KING_MOVES,
    player::{self, *},
    r#move::{Move, Position},
    tile::*,
};

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
    moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            move_id: 0,
            mode: GameMode::select_gamemode().unwrap_or(GameMode::HumanVsHuman),
            moves: Vec::new(),
        }
    }

    pub fn from_board(board: &[Tile; 64], mode: GameMode) -> Self {
        let mut b = Board::empty();
        *b.board_mut() = *board;
        Self {
            board: b,
            move_id: 0,
            mode,
            moves: Vec::new(),
        }
    }

    pub fn get_current_player(&self) -> Player {
        if self.move_id % 2 == 0 {
            Player::Black
        } else {
            Player::White
        }
    }

    /// This function looks at the current board and determines the best possible move it can make in that given state.
    /// In order to figure this out, it uses the negamax algorithm, which is a variant of the minimax algorithm.  
    pub fn get_best_move(&self) -> Result<Move> {
        let moves = self.generate_all_possible_moves()?;
        Ok(*moves.first().unwrap())
    }

    pub fn generate_all_possible_moves(&self) -> Result<Vec<Move>> {
        let moving_player = self.get_current_player();
        let friendly_peices = self.board.get_idx_of_player_peices(moving_player);

        Ok(friendly_peices
            .iter()
            .map(|peice| self.generate_moves_for_peice(*peice).unwrap())
            .flatten()
            .collect())
    }

    pub fn generate_moves_for_peice(&self, peice: Position) -> Result<Vec<Move>> {
        let selected_peice = self.board[peice];
        let mut potential_moves = Vec::new();

        if selected_peice.is_empty() {
            return Err(anyhow!("Selected peice is empty!"));
        }

        match selected_peice.kind {
            TileKind::Normal => {
                let (x, y) = peice.coords();
                let y_offset: isize = if let Player::Black = selected_peice.occupied_by.unwrap() {
                    1
                } else {
                    -1
                };
                match Position::from_coords_checked(x as isize + 1, y as isize + y_offset) {
                    Ok(pos) => potential_moves.push(Move::new(peice.idx(), pos.idx())),
                    Err(_) => (),
                };

                match Position::from_coords_checked(x as isize - 1, y as isize + y_offset) {
                    Ok(pos) => potential_moves.push(Move::new(peice.idx(), pos.idx())),
                    Err(_) => (),
                };
            }
            TileKind::King => {
                let moves = KING_MOVES[peice.idx()].to_vec();

                for king_move in moves {
                    potential_moves.push(Move::new(peice.idx(), king_move));
                }
            }
        }

        Ok(potential_moves
            .iter()
            .filter(|m| self.is_valid_move(**m))
            .copied()
            .collect())
    }

    fn evaluate_board(&self, moving_player: Player) -> isize {
        let friendly_peices = self.board.get_idx_of_player_peices(moving_player);
        let enemy_peices = self.board.get_idx_of_player_peices(!moving_player);
        // let distance_to_promotion = todo!();
        // Get number of peices where a capture is possible
        let potential_captures = friendly_peices
            .iter()
            .filter(|idx| self.board.can_capture(moving_player, **idx))
            .count();
        let vulnerable_peices = enemy_peices
            .iter()
            .filter(|idx| self.board.can_capture(!moving_player, **idx))
            .count();

        potential_captures as isize - vulnerable_peices as isize
    }

    pub fn run(&mut self) -> Result<Player> {
        loop {
            let moving_player = self.get_current_player();
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

            if let GameMode::HumanVsHuman = self.mode {
                clear()?;
            };

            println!("{}", self.board);
            self.get_stats();
            println!("Valid Moves:");
            let this_move = loop {
                let this_move = match self.mode {
                    GameMode::HumanVsHuman => self.get_user_move(),
                    GameMode::HumanVsAi if self.move_id % 2 == 0 => self.get_user_move(),
                    GameMode::HumanVsAi => self.get_best_move(),
                    GameMode::AiVsAi => self.get_best_move(),
                };
                if let Ok(m) = this_move {
                    break m;
                } else {
                    println!("{this_move:?}");
                }
            };
            self.moves.push(this_move);
            let opp_count_before_move = self.board.get_remaining_peices(!moving_player);
            self.board.make_move(self.move_id, this_move)?;
            let opp_count_after_move = self.board.get_remaining_peices(!moving_player);

            self.move_id += if opp_count_after_move <= opp_count_before_move {
                1
            } else {
                2
            }
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
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

    pub fn get_user_move(&self) -> Result<Move> {
        println!("Moving player: {}", self.get_current_player());
        let valid_moves = self.generate_all_possible_moves()?;
        let selection = Select::new()
            .with_prompt("Select a move (use arrow keys to make your selection)")
            .items(&valid_moves)
            .interact()?;
        Ok(*valid_moves.get(selection).unwrap())
    }

    #[allow(clippy::clone_on_copy)]
    // Lint disabled here as we need to make a deep copy to prevent the game
    // from actually making the move before we know if its valid or not
    fn is_valid_move(&self, this_move: Move) -> bool {
        let mut board_cpy = self.board.clone();
        board_cpy.make_move(self.move_id, this_move).is_ok()
    }
}
