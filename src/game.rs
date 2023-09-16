use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::board;

use super::{Board, Piece, PlacePieceError, Player};

#[derive(Debug)]
pub struct Game {
    pub board: Board<Piece>,
    dim: usize,
    width: usize,
    players: Vec<Player>,
    last_piece: HashMap<Player, (usize, Piece)>,
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
            last_piece: HashMap::with_capacity(players as usize),
        }
    }

    /// # Panics
    ///
    /// - Panics if `piece.player` is `None`.
    pub fn place_piece(&mut self, piece: Piece, coords: board::Idx) -> Result<(), PlacePieceError> {
        match self.board.get(coords.clone()) {
            // The outer optional is for in bounds, the inner optional is for occupied.
            Some(Piece {
                player: Some(_), ..
            }) => return Err(PlacePieceError::Occupied),
            Some(Piece { player: None, .. }) => (), // continue
            None => return Err(PlacePieceError::OutOfBounds),
        }

        self.last_piece.insert(
            piece.player.unwrap(),
            (self.get_index(coords.clone().into()), piece.clone()),
        );
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

        let piece = match &self.last_piece.get(&player) {
            Some((i, piece)) => (*i, piece.clone()),
            None => return false,
        };

        let pieces = board
            .iter()
            .enumerate()
            .filter(|(i, e)| match e {
                Piece {
                    player: Some(p), ..
                } => p == &player,
                Piece { player: None, .. } => false,
            } && *i != piece.0)
            // Remove levels of reference
            .map(|(i, e)| (i, e.clone().clone()))
            .combinations(self.width - 1)
            .map(|mut c| {
                c.push(piece.clone());
                c.sort_unstable_by_key(|e| e.0);
                c
            });

        let pieces: Vec<_> = dbg!(pieces.collect());

        for combination in pieces {
            // Calculate the coordinates of the pieces in each dimension
            let combination_works = (1..=self.dim).all(|dim| {
                // Get the coordinates of the pieces in this dimension
                let coords = combination.iter().map(|(i, _)| self.get_coord(*i, dim));

                // Check if all the pieces are either all the same or all different
                let comp_set: HashSet<usize, RandomState> = HashSet::from_iter(coords.clone());
                return (comp_set.len() == 1 || comp_set.len() == self.width)
                    && Self::combination_no_wraparound(
                        combination.iter().map(|(i, _)| *i).collect(),
                    );
            });

            if combination_works {
                return true;
            }
        }

        false
    }

    fn combination_no_wraparound(coords: Vec<usize>) -> bool {
        if coords.len() <= 2 {
            return false;
        }

        // Get the vector between the first two elements, and check if it works
        // for all the other pieces
        // SAFTEY: coords.len() > 2 checked above
        let first = coords[0];
        let second = coords[1];
        let vector = second as isize - first as isize; // `isize` to prevent underflow

        eprintln!("{:?}, {}, {}, {}", coords, first, second, vector);

        // the second element because it starts checking again from the first
        let mut prev = second;

        // Check if the vector works for all the other pieces
        coords[2..].iter().all(|e| {
            // Check if the vector works for this piece
            // `isize` to prevent "attempt to subtract with overflow"
            let diff = *e as isize - prev as isize;

            eprintln!("{}, {} - {} = {} = {}", vector, e, prev, e - prev, diff);

            prev = *e;

            diff == vector
        })
    }

    fn get_coord(&self, index: usize, dim: usize) -> usize {
        // Kieran and I (Gavin) worked for a while to get this.
        // It works. 🎉
        ((index % self.width.pow(dim as u32)) - (index % self.width.pow((dim - 1) as u32)))
            / self.width.pow((dim - 1) as u32)
    }

    fn get_index(&self, coords: Vec<usize>) -> usize {
        coords
            .iter()
            .rev()
            .enumerate()
            .map(|(i, e)| e * self.width.pow(i as u32))
            .sum()
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

impl Game {
    pub fn display(&self, hide_padding: bool) -> String {
        self.board
            .display(board::Direction::Horizontal, hide_padding)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.board))
    }
}
