use std::{
    collections::{hash_map::RandomState, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::board;

use super::{Board, Piece, PlacePieceError, Player};

#[derive(Debug)]
pub struct Game {
    board: Board<Piece>,
    dim: usize,
    width: usize,
    players: Vec<Player>,
}

impl Game {
    pub fn new(dim: usize, players: u32) -> Self {
        let width = dim + 1;
        Self {
            board: Board::<Option<Piece>>::new(vec![width; dim], Piece::empty()),
            dim,
            width,
            players: (0..players)
                .map(|n| n.try_into().expect("Valid number of players"))
                .collect(),
        }
    }

    pub fn place_piece(&mut self, piece: Piece, coords: board::Idx) -> Result<(), PlacePieceError> {
        match self.board.get(coords.clone()) {
            // The outer optional is for in bounds, the inner optional is for occupied.
            Some(Piece {
                player: Some(_), ..
            }) => return Err(PlacePieceError::Occupied),
            Some(Piece { player: None, .. }) => (), // continue
            None => return Err(PlacePieceError::OutOfBounds),
        }

        self.board[coords] = piece.into();
        Ok(())
    }

    pub fn check_win(&self, player: Player) -> bool {
        // Check this piece with all the other pieces
        // For there to be a win, each dimension must satisfy one of the following:
        // 1. All the pieces are the same
        // 2. All the pieces are different

        // Collect all the pieces for the current player
        let board = self.board.flatten();

        let pieces = board
            .iter()
            .enumerate()
            .filter(|(_, e)| match e {
                Piece {
                    player: Some(p), ..
                } => p == &player,
                Piece { player: None, .. } => false,
            })
            .combinations(self.width);

        for combination in pieces {
            // Calculate the coordinates of the pieces in each dimension
            let combination_works = (1..=self.dim).all(|dim| {
                // Get the coordinates of the pieces in this dimension
                let coords = combination.iter().map(|(i, _)| self.get_coord(*i, dim));

                // Check if all the pieces are either all the same or all different
                let comp_set: HashSet<usize, RandomState> = HashSet::from_iter(coords.clone());
                return (comp_set.len() == 1 || comp_set.len() == self.width)
                    && !Self::combination_has_wraparound(
                        combination.iter().map(|(i, _)| *i).collect(),
                    );
            });

            if combination_works {
                return true;
            }
        }

        return false;
    }

    fn combination_has_wraparound(coords: Vec<usize>) -> bool {
        if coords.len() <= 2 {
            return false;
        }

        // Get the vector between the first two elements, and check if it works
        // for all the other pieces
        // SAFTEY: coords.len() > 2 checked above
        let first = coords[0];
        let second = coords[1];
        let vector = second as isize - first as isize; // `isize` to prevent underflow

        // Check if the vector works for all the other pieces
        if coords[2..].iter().all(|e| {
            // Check if the vector works for this piece
            // `isize` to prevent "attempt to subtract with overflow"
            let diff = *e as isize - first as isize;
            diff % vector == 0
        }) {
            false
        } else {
            true
        }
    }

    fn get_coord(&self, index: usize, dim: usize) -> usize {
        // Kieran and I (Gavin) worked for a while to get this.
        // It works. 🎉
        ((index % self.width.pow(dim as u32)) - (index % self.width.pow((dim - 1) as u32)))
            / self.width.pow((dim - 1) as u32)
    }

    fn get_coords(&self, index: usize) -> Vec<usize> {
        (1..=self.dim)
            .map(|dim| self.get_coord(index, dim))
            .collect()
    }

    pub fn current_player(&self) -> Player {
        // Get number of pieces on the board
        let num_pieces = self
            .board
            .flatten()
            .iter()
            .filter(|e| e.player.is_some())
            .count();

        // Get the player whose turn it is
        self.players[num_pieces % self.players.len()]
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.board).as_str())
    }
}
