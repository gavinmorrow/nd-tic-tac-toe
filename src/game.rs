use std::{
    collections::{hash_map::RandomState, HashSet, VecDeque},
    fmt::{Debug, Display},
};

use itertools::Itertools;

use super::{Board, Piece, PlacePieceError, Player};

#[derive(Debug)]
pub struct Game {
    board: Board<Option<Piece>>,
    dim: usize,
    width: usize,
    players: Vec<Player>,
}

impl Game {
    pub fn new(dim: usize, players: u32) -> Self {
        let width = dim + 1;
        Self {
            board: Board::<Option<Piece>>::new(vec![width; dim], None),
            dim,
            width,
            players: (0..players)
                .map(|n| n.try_into().expect("Valid number of players"))
                .collect(),
        }
    }

    pub fn place_piece(&mut self, piece: Piece) -> Result<(), PlacePieceError> {
        // Convert coords to something that can be used by indexing
        let coords: VecDeque<usize> = piece.coords.clone().into();

        match self.board.get(coords.clone()) {
            // The outer optional is for in bounds, the inner optional is for occupied.
            Some(Some(_)) => return Err(PlacePieceError::Occupied),
            Some(None) => (), // continue
            None => return Err(PlacePieceError::OutOfBounds),
        }

        self.board[coords] = Some(piece).into();
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
                Some(p) => p.player == player,
                None => false,
            })
            .map(|(i, p)| (i, p.as_ref().expect("Just filtered out all the Nones")))
            .combinations(self.width);

        for combination in pieces {
            // Calculate the coordinates of the pieces in each dimension
            let combination_works = (1..=self.dim).all(|dim| {
                // Get the coordinates of the pieces in this dimension
                let coords = combination.iter().map(|(i, _)| self.get_coord(*i, dim));

                // Check if all the pieces are either all the same or all different
                let comp_set: HashSet<usize, RandomState> = HashSet::from_iter(coords);
                return comp_set.len() == 1 || comp_set.len() == self.width;
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

    fn get_coords(&self, index: usize) -> Vec<usize> {
        (1..=self.dim)
            .map(|dim| self.get_coord(index, dim))
            .collect()
    }

    pub fn current_player(&self) -> Player {
        // Get number of pieces on the board
        let num_pieces = self.board.flatten().iter().filter(|e| e.is_some()).count();

        // Get the player whose turn it is
        self.players[num_pieces % self.players.len()]
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)
    }

    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     todo!()
    // }

    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     // println!("{:?}", self.board);

    //     if self.dim != 4 {
    //         todo!();
    //     }

    //     // Get a 5x5 array of the 5x5 grids
    //     let mut grids: [[String; 5]; 5] = Default::default();
    //     for (i, grid_row) in self.board.outer_iter().enumerate() {
    //         for (j, grid) in grid_row.outer_iter().enumerate() {
    //             let grid_x = j;
    //             let grid_y = i;

    //             // Format grid
    //             let mut gstr = String::new();
    //             let mut rows = Vec::new();
    //             for row in grid.rows() {
    //                 let mut out = Vec::new();
    //                 for cell in row {
    //                     match cell {
    //                         Some(p) => out.push(p.symbol()),
    //                         None => out.push(' '),
    //                     }
    //                 }
    //                 let out = out.iter().join(" | ");
    //                 rows.push(out);
    //             }
    //             let rows = rows
    //                 .iter()
    //                 .join(&format!("\n{}\n", "-".repeat(rows[0].len())));
    //             gstr.push_str(&rows);

    //             grids[grid_x][grid_y] = gstr;
    //         }
    //     }

    //     // Join each row of grids
    //     let mut out = Vec::new();
    //     for row in grids.iter() {
    //         // Combine rows (multiline string)
    //         let mut rows = vec![String::new(); 9 /* 5 + 4 */];

    //         for grid in row {
    //             let lines: Vec<&str> = grid.lines().collect();
    //             for (i, line) in lines.iter().enumerate() {
    //                 rows[i].push_str(line);
    //                 rows[i].push_str("    ");
    //             }
    //         }

    //         let row = rows.iter().join("\n");

    //         out.push(row);
    //     }

    //     // Join each row of grids
    //     let mut out = out.iter().join(&format!(
    //         "\n{}\n\n",
    //         " ".repeat(out[0].lines().into_iter().next().unwrap().len())
    //     ));

    //     for player in self.players.iter() {
    //         out = out.replace(player.symbol(), &player.with_color());
    //     }

    //     f.write_str(&out)?;

    //     Ok(())
    // }

    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     if self.dim % 2 == 0 {
    //         // Grid
    //         for row in self.board.outer_iter() {
    //             for grid_row in row.outer_iter() {
    //                 for cell in grid_row {
    //                     match cell {
    //                         Some(p) => write!(f, "{}", p.player.with_color())?,
    //                         None => write!(f, ".")?,
    //                     }
    //                     write!(f, " ")?;
    //                 }
    //                 write!(f, "|")?;
    //             }
    //             writeln!(f, "/")?;
    //         }
    //     } else {
    //         // Row
    //         for row in self.board.outer_iter() {
    //             for cell in row {
    //                 match cell {
    //                     Some(p) => write!(f, "{}", p.player.with_color())?,
    //                     None => write!(f, ".")?,
    //                 }
    //                 write!(f, "|")?;
    //             }
    //             writeln!(f, "/")?;
    //         }
    //     }

    //     Ok(())
    // }
}
