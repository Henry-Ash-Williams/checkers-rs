use clearscreen::clear;
use dialoguer::Select;
use rand::Rng;
use slab_tree::{Tree, TreeBuilder, self};
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

    #[allow(clippy::clone_on_copy)]
    /// This function looks at the current board and determines the best possible move it can make in that given state.
    /// In order to figure this out, it uses the negamax algorithm, which is a variant of the minimax algorithm.  
    pub fn get_best_move(&self) -> Result<Move> {
        let mut search_space = TreeBuilder::new().with_root((self.board, self.board.evaluate_board(self.get_current_player()))).build();
        let moves = self.generate_all_possible_moves(self.board, self.get_current_player())?;
        let move_evaluations: HashMap<Move, isize> = HashMap::from_iter(moves.iter().map(|potential_move| (*potential_move, self.evaluate_move(*potential_move).unwrap())));
        let mut root = search_space.root_mut().unwrap();
        move_evaluations.iter().for_each(|possible_move| {
            let (possible_move, evaluation) = possible_move;
            let mut board_cpy = self.board.clone(); 
            board_cpy.make_move(self.move_id, *possible_move).unwrap();
            root.append((board_cpy, *evaluation));
        });

        let mut rng = rand::thread_rng(); 
        let best_move = move_evaluations
            .iter()
            .max_by(|me_lhs, me_rhs| me_lhs.1.cmp(me_rhs.1));

        let best_move = match best_move {
            Some(m) => *m.0, 
            None => {
                if moves.is_empty() {
                    return Err(anyhow!("No moves remain!"));
                }

                let idx = rng.gen_range(0..moves.len());

                *moves.get(idx).unwrap()
            }
        }; 

        println!("AI chose move {best_move}"); 

        Ok(best_move)
    }

    pub fn generate_all_possible_moves(&self, board: Board, moving_player: Player) -> Result<Vec<Move>> {
        let friendly_peices = board.get_idx_of_player_peices(moving_player);

        Ok(friendly_peices
            .iter()
            .flat_map(|peice| self.generate_moves_for_peice(*peice).unwrap())
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

                if let Ok(pos) = Position::from_coords_checked(x as isize + 1, y as isize + y_offset) {
                    potential_moves.push(Move::new(peice.idx(), pos.idx()))
                }

                if let Ok(pos) =  Position::from_coords_checked(x as isize - 1, y as isize + y_offset) {
                    potential_moves.push(Move::new(peice.idx(), pos.idx()))
                }
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


    pub fn run(&mut self) -> Result<Player> {
        loop {
            if self.move_id >= 2 * (12 * 12) {
                println!("Too many moves");
                break Err(anyhow!("Too many moves"));
            }
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
            let this_move = loop {
                let this_move = match self.mode {
                    GameMode::HumanVsHuman => self.get_user_move(),
                    GameMode::HumanVsAi if self.move_id % 2 == 0 => self.get_user_move(),
                    GameMode::HumanVsAi => self.get_best_move(),
                    GameMode::AiVsAi => self.get_best_move(),
                };

                if let Ok(m) = this_move {
                    break m;
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
        let valid_moves = self.generate_all_possible_moves(self.board, self.get_current_player())?;
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


    #[allow(clippy::clone_on_copy)]
    // See Game::is_valid move for explaination of why I'm disabling the lint 
    fn evaluate_move(&self, this_move: Move) -> Result<isize> {
        let mut board_cpy = self.board.clone(); 
        board_cpy.make_move(self.move_id, this_move)?;
        Ok(board_cpy.evaluate_board(!self.get_current_player()))
    }


}
