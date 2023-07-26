use std::collections::{hash_map::RandomState, HashSet};

use itertools::Itertools;
use ndarray::{Array, IxDyn};

type Board = Array<Option<Player>, IxDyn>;

struct Game {
    board: Board,
    dim: usize,
    width: usize,
    players: Vec<Player>,
}

impl Game {
    fn new(dim: usize, players: u32) -> Self {
        let width = dim + 1;
        Self {
            board: Array::from_elem(vec![width; dim], None),
            dim,
            width,
            players: (0..players).map(Player).collect(),
        }
    }

    fn place_piece(&mut self, piece: Piece) -> Result<(), PlacePieceError> {
        let Piece { player, coords } = piece;

        // Convert coords to something that can be used by indexing
        let coords = &coords[..];

        match self.board.get(coords) {
            // The outer optional is for in bounds, the inner optional is for occupied.
            Some(Some(_)) => return Err(PlacePieceError::Occupied),
            Some(None) => (), // continue
            None => return Err(PlacePieceError::OutOfBounds),
        }

        self.board[coords] = Some(player);
        Ok(())
    }

    fn check_win(&self, player: Player) -> bool {
        // Check this piece with all the other pieces
        // For there to be a win, each dimension must satisfy one of the following:
        // 1. All the pieces are the same
        // 2. All the pieces are different

        // Collect all the pieces for the current player
        let pieces = self
            .board
            .iter()
            .enumerate()
            .filter(|(_, e): &(usize, &Option<Player>)| match e {
                Some(p) => p == &player,
                None => false,
            })
            .map(|(i, p)| (i, p.expect("Just filtered out all the Nones")))
            .combinations(self.width);

        for combination in pieces {
            // Calculate the coordinates of the pieces in each dimension
            let combination_works = (1..=self.dim).all(|dim| {
                // Get the coordinates of the pieces in this dimension
                let coords = combination.iter().map(|(i, _)| self.get_coord(*i, dim));

                // Check if all the pieces are either all the same or all different
                let comp_set: HashSet<usize, RandomState> = HashSet::from_iter(coords);
                return comp_set.len() == 1 || comp_set.len() == self.dim;
            });

            if combination_works {
                return true;
            }
        }

        return false;
    }

    fn get_coord(&self, index: usize, dim: usize) -> usize {
        // Kieran and I (Gavin) worked for a while to get this.
        // It works. 🎉
        ((index % self.width.pow(dim as u32)) - (index % self.width.pow((dim - 1) as u32)))
            / self.width.pow((dim - 1) as u32)
    }
}

struct Piece {
    player: Player,
    coords: Vec<usize>,
}

impl Piece {
    fn new(player: Player, coords: Vec<usize>) -> Self {
        Self { player, coords }
    }
}

#[derive(Debug)]
enum PlacePieceError {
    OutOfBounds,
    Occupied,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Player(u32);

#[cfg(test)]
mod test {
    use super::*;

    fn setup_winner(winner: Player, loser: Player) -> Game {
        let mut game = Game::new(3, 2);
        let p0 = loser;
        let p1 = winner;

        // Insert pieces
        game.place_piece(Piece::new(p0, vec![1, 1, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![2, 2, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![1, 2, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![1, 3, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 1, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 1, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 3, 3])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 0, 0])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 2, 2])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 3, 3])).unwrap();
        game.place_piece(Piece::new(p0, vec![1, 1, 3])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 2, 2])).unwrap();

        game
    }

    #[test]
    fn test_p0_win() {
        let p0 = Player(0);
        let p1 = Player(1);
        let game = setup_winner(p0, p1);

        // Check winner
        assert!(game.check_win(p0));
        assert!(!game.check_win(p1));
    }

    #[test]
    fn test_p1_win() {
        let p0 = Player(0);
        let p1 = Player(1);
        let game = setup_winner(p1, p0);

        // Check winner
        assert!(!game.check_win(p0));
        assert!(game.check_win(p1));
    }
}
