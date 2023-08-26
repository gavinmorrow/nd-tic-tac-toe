use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

type Idx = VecDeque<usize>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Board<T> {
    Nd(Vec<Board<T>>),
    Piece(T),
}

impl<T> Board<T> {
    /// # Panics
    ///
    /// Panics if `dim` is empty.
    pub fn new<V: Clone>(mut dim: Vec<usize>, initial_value: V) -> Board<V> {
        // FIXME: there might be a cleaner way without all the mutation

        if dim.is_empty() {
            panic!("Cannot create a board with no dimensions");
        }

        // Start with the innermost dimension, and work our way out.
        let mut board = vec![Board::Piece(initial_value); dim.pop().unwrap()];

        for dim in dim.into_iter().rev() {
            board = vec![Board::Nd(board); dim];
        }

        Board::Nd(board)
    }
}

impl<T> Board<T> {
    pub fn get_board(&self, mut coords: Idx) -> Option<&Board<T>> {
        if coords.is_empty() {
            return Some(self);
        }

        match self {
            Board::Nd(boards) => {
                if let Some(board) = boards.get(coords[0]) {
                    coords.pop_front();
                    board.get_board(coords)
                } else {
                    None
                }
            }
            Board::Piece(_) => None,
        }
    }

    // FIXME: This is copy-pasted from the `get_board()` method.
    // I haven't been able to find a good solution. <https://stackoverflow.com/questions/41436525/>
    pub fn get_board_mut(&mut self, mut coords: Idx) -> Option<&mut Board<T>> {
        if coords.is_empty() {
            return Some(self);
        }

        match self {
            Board::Nd(boards) => {
                if let Some(board) = boards.get_mut(coords[0]) {
                    coords.pop_front();
                    board.get_board_mut(coords)
                } else {
                    None
                }
            }
            Board::Piece(_) => None,
        }
    }

    pub fn get(&self, coords: Idx) -> Option<&T> {
        self.get_board(coords).and_then(|board| match board {
            Board::Nd(_) => None,
            Board::Piece(piece) => Some(piece),
        })
    }

    pub fn get_mut(&mut self, coords: Idx) -> Option<&mut T> {
        self.get_board_mut(coords).and_then(|board| match board {
            Board::Nd(_) => None,
            Board::Piece(piece) => Some(piece),
        })
    }

    pub fn flatten(&self) -> Vec<&T> {
        match self {
            Board::Nd(boards) => {
                boards
                    .iter()
                    .fold(vec![], |mut acc, e| {
						acc.append(&mut e.flatten());
						acc
					})
            }
            Board::Piece(piece) => vec![piece],
        }
    }
}

impl<T> Index<Idx> for Board<T> {
    type Output = Board<T>;

    fn index(&self, index: Idx) -> &Self::Output {
        self.get_board(index).unwrap()
    }
}

impl<T> IndexMut<Idx> for Board<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.get_board_mut(index).unwrap()
    }
}

impl<T> From<T> for Board<T> {
    fn from(piece: T) -> Self {
        Board::Piece(piece)
    }
}
