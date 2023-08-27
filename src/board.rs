use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut},
};

pub(crate) type Idx = VecDeque<usize>;

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
            Board::Nd(boards) => boards.iter().fold(vec![], |mut acc, e| {
                acc.append(&mut e.flatten());
                acc
            }),
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }
}

/// # Panics
///
/// Assumes that the strings are rectangular (all the lines are the same length,
/// and both strings are the same size in whatever dimension they are being
/// combined in).
/// May panic or not work as expected otherwise.
fn combine_multiline_strings(strings: Vec<String>, direction: Direction) -> String {
    match direction {
        Direction::Horizontal => {
            // Optimization for allocating the output string
            let mut strings = strings
                .iter()
                .map(|s| s.lines().map(|s| s.to_string()).collect::<Vec<_>>());

			let init = strings.next().unwrap();

            let combined_lines = strings.fold(init, |mut vec, lines| {
                for (i, line) in lines.iter().enumerate() {
					// eprintln!("vec: {:?}, i: {i}, line: {line}", vec);
                    vec[i].push_str(&format!(" {}", line));
                }

                vec
            });

            format!(" {} ", combined_lines.join(" \n "))

            // combined.fold(String::with_capacity(len), |mut acc, e| {
            //     writeln!(acc, "{}-{}", e.0, e.1).unwrap();
            //     acc
            // })
            // combined.map(|(a, b)| format!("{a}{b}")).join("\n")
        }
        Direction::Vertical => {
            // Just append the strings
            format!("\n{}\n", strings.join("\n"))
        }
    }
}

impl<T: Display + std::fmt::Debug> Board<T> {
    fn display(&self, direction: Direction) -> String {
        match self {
            Board::Nd(boards) => {
                let boards = boards.iter().map(|board| board.display(direction.next()));
				combine_multiline_strings(boards.collect(), direction)

                // dbg!(chunks
                //     .into_iter()
                //     .map(|mut chunk| {
                //         let a = dbg!(chunk.next().unwrap());
                //         let b = dbg!(chunk.next());
                //         dbg!(combine_multiline_strings(
                //             a,
                //             b.unwrap_or_default(),
                //             dbg!(direction)
                //         ))
                //     })
                //     .join(""))
            }
            // .fold(String::new(), |acc, board| {
            //     combine_multiline_strings(acc, board, direction)
            // }),
            Board::Piece(piece) => piece.to_string(),
        }
    }
}

impl<T: Display + std::fmt::Debug> Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display(Direction::Horizontal).as_str())
    }
}
